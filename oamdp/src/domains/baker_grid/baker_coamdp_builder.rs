use std::fs;

use itertools::iproduct;
use mdp::baker_grid::BakerGridAction::*;
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
use mdp::mdp_traits::Build;
use mdp::{
    baker_grid::{BakerGridMDP, BakerGridPartialMDP, BakerGridState},
    mdp_traits::BuildFrom,
    policy::softmax_policy::SoftmaxPolicyBuilder,
};
use num_traits::FromPrimitive;
use ordered_float::NotNan;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::belief_cost_function::{BeliefCostFunction, BeliefCostType, Objective};
use crate::belief_update_type::ObserveabilityAssumption;
use crate::domains::baker_grid::{
    BakerCommunicationAction, BakerJointAction, CommunicationType, Shape,
};
use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::OAMDPFiniteHorizon;
use crate::observer_model::ExplicitCommunicationModel;

use super::communication_model::{BakerCommunicationModel, BakerCommunicationProb};

#[derive(Serialize, Deserialize)]
pub struct BakerCOAMDPBuilder<const N: usize> {
    partial_mdp: BakerGridPartialMDP,
    #[serde(with = "serde_arrays")]
    pub possible_goals: [BakerGridState; N],
    beta: f32,
    communication_type: CommunicationType,
    communication_actions: Vec<BakerCommunicationAction>,
    communication_cost: f32,
    max_t: usize,
    true_goal: usize,
    #[serde(with = "serde_arrays")]
    shapes: [Shape; N],
    belief_cost_function: BeliefCostFunction<N>,
    cost_type: Objective,
}

impl<const N: usize> BakerCOAMDPBuilder<N> {
    pub fn set_horizon(mut self, horizon: usize) -> BakerCOAMDPBuilder<N> {
        self.max_t = horizon;

        self
    }
}

impl BakerCOAMDPBuilder<3> {
    pub fn new(instance_id: usize) -> Self {
        let path = format!(
            "{}/src/domains/baker_grid/coamdp_instances/baker_{}.yaml",
            env!("CARGO_MANIFEST_DIR"),
            instance_id
        );
        let data = fs::read_to_string(&path).expect("Unable to read file");
        serde_yaml::from_str(&data).expect("Invalid yaml")
    }

    pub fn random_instance(
        h: usize,
        w: usize,
        obstacles: Vec<(i32, i32)>,
        min_distance_to_goal: usize,
        belief_cost_type: BeliefCostType,
        rng: &mut ThreadRng,
    ) -> Self {
        let possible_goals = pick_possible_goals(h, w, &obstacles, rng);
        let true_goal = rng.gen_range(0, 3);

        let mut initial_state = pick_possible_goal(h, w, &obstacles, rng);
        while (possible_goals[true_goal].i - initial_state.i).abs()
            + (possible_goals[true_goal].j - initial_state.j).abs()
            < min_distance_to_goal as i32
        {
            initial_state = pick_possible_goal(h, w, &obstacles, rng);
        }
        let partial_mdp = BakerGridPartialMDP::new(h, w, obstacles)
            .set_initial_state(initial_state)
            .set_prob_veering(0.1);
        let alpha = rng.gen_range(0.0, 1.0);
        let communication_cost = rng.gen_range(0.0, 0.25);

        BakerCOAMDPBuilder {
            partial_mdp: partial_mdp,
            possible_goals: possible_goals,
            beta: 0.3,
            communication_type: CommunicationType::SoftGenerativeNoise(0.5, 0.1),
            communication_actions: pick_available_messages(rng),
            true_goal: true_goal,
            communication_cost: communication_cost,
            belief_cost_function: match belief_cost_type {
                BeliefCostType::Disimulation => BeliefCostFunction::Disimulation,
                BeliefCostType::TVDistance => {
                    BeliefCostFunction::get_legible_cost_function(true_goal)
                }
                _ => panic!("not implemented"),
            },
            max_t: 20,
            shapes: [Shape::random(rng), Shape::random(rng), Shape::random(rng)],
            cost_type: Objective::LinearCombination(alpha, 1.0 - alpha),
        }
    }
}

pub(crate) fn pick_possible_goal(
    h: usize,
    w: usize,
    obstacles: &Vec<(i32, i32)>,
    rng: &mut ThreadRng,
) -> BakerGridState {
    let mut i = rng.gen_range(0, h);
    let mut j = rng.gen_range(0, w);

    while obstacles.contains(&(i as i32, j as i32)) {
        i = rng.gen_range(0, h);
        j = rng.gen_range(0, w);
    }
    BakerGridState::new(i as i32, j as i32)
}

pub(crate) fn pick_possible_goals(
    h: usize,
    w: usize,
    obstacles: &Vec<(i32, i32)>,
    rng: &mut ThreadRng,
) -> [BakerGridState; 3] {
    [
        pick_possible_goal(h, w, obstacles, rng),
        pick_possible_goal(h, w, obstacles, rng),
        pick_possible_goal(h, w, obstacles, rng),
    ]
}

pub(crate) fn pick_available_messages(rng: &mut ThreadRng) -> Vec<BakerCommunicationAction> {
    let mut messages = vec![BakerCommunicationAction::None];
    for m in [
        BakerCommunicationAction::Blue,
        BakerCommunicationAction::Circle,
        BakerCommunicationAction::Green,
        BakerCommunicationAction::Square,
    ] {
        let r = rng.gen_bool(0.8);
        if r {
            messages.push(m)
        }
    }
    messages
}

fn initial_belief<const N: usize>() -> [NotNan<f32>; N] {
    let belief = [NotNan::from_f32(1.0 / (N as f32)).unwrap(); N];
    belief
}

impl<const N: usize>
    Build<OAMDPFiniteHorizon<BakerCommunicationModel<N>, BakerGridMDP, BakerJointAction, N>>
    for BakerCOAMDPBuilder<N>
{
    fn build(
        self,
    ) -> OAMDPFiniteHorizon<BakerCommunicationModel<N>, BakerGridMDP, BakerJointAction, N> {
        let mdp = self
            .partial_mdp
            .build_from(&self.possible_goals[self.true_goal]);

        let physical_actions = vec![
            North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest, Stay,
        ];

        let joint_actions = iproduct!(physical_actions.iter(), self.communication_actions.iter())
            .map(|(a, b)| BakerJointAction::new(*a, *b))
            .collect::<Vec<_>>();

        let communication_prob = BakerCommunicationProb::new(
            self.shapes,
            self.possible_goals,
            self.communication_type,
            self.communication_actions,
            self.communication_cost,
        );
        let om = ExplicitCommunicationModel::new_from_possible_goals(
            &self.partial_mdp,
            &SoftmaxPolicyBuilder::new(self.beta),
            self.possible_goals,
            ObserveabilityAssumption::OnlyActionsAreConsidered,
            communication_prob,
        );

        let oamdp = FiniteHorizonWrapper::new(
            OAMDP::new(
                om,
                mdp,
                self.belief_cost_function,
                initial_belief(),
                0.9,
                joint_actions,
                self.cost_type,
            ),
            self.max_t,
        );

        oamdp
    }
}

impl BakerCOAMDPBuilder<5> {
    pub fn new5(instance_id: usize) -> Self {
        let path = format!(
            "{}/src/domains/baker_grid/coamdp_instances/baker_{}.yaml",
            env!("CARGO_MANIFEST_DIR"),
            instance_id
        );
        let data = fs::read_to_string(&path).expect("Unable to read file");
        serde_yaml::from_str(&data).expect("Invalid yaml")
    }

    pub fn random_instance5(
        h: usize,
        w: usize,
        obstacles: Vec<(i32, i32)>,
        min_distance_to_goal: usize,
        belief_cost_type: BeliefCostType,
        rng: &mut ThreadRng,
    ) -> Self {
        let possible_goals = pick_possible_goals5(h, w, &obstacles, rng);
        let true_goal = rng.gen_range(0, 5);

        let mut initial_state = pick_possible_goal(h, w, &obstacles, rng);
        while (possible_goals[true_goal].i - initial_state.i).abs()
            + (possible_goals[true_goal].j - initial_state.j).abs()
            < min_distance_to_goal as i32
        {
            initial_state = pick_possible_goal(h, w, &obstacles, rng);
        }
        let partial_mdp = BakerGridPartialMDP::new(h, w, obstacles)
            .set_initial_state(initial_state)
            .set_prob_veering(0.1);
        let alpha = rng.gen_range(0.0, 1.0);
        let communication_cost = rng.gen_range(0.0, 0.25);

        BakerCOAMDPBuilder {
            partial_mdp: partial_mdp,
            possible_goals: possible_goals,
            beta: 0.3,
            communication_type: CommunicationType::SoftGenerativeNoise(0.5, 0.1),
            communication_actions: pick_available_messages(rng),
            true_goal: true_goal,
            communication_cost: communication_cost,
            belief_cost_function: match belief_cost_type {
                BeliefCostType::Disimulation => BeliefCostFunction::Disimulation,
                BeliefCostType::TVDistance => {
                    BeliefCostFunction::get_legible_cost_function(true_goal)
                }
                _ => panic!("not implemented"),
            },
            max_t: 20,
            shapes: [
                Shape::random(rng),
                Shape::random(rng),
                Shape::random(rng),
                Shape::random(rng),
                Shape::random(rng),
            ],
            cost_type: Objective::LinearCombination(alpha, 1.0 - alpha),
        }
    }
}

pub(crate) fn pick_possible_goals5(
    h: usize,
    w: usize,
    obstacles: &Vec<(i32, i32)>,
    rng: &mut ThreadRng,
) -> [BakerGridState; 5] {
    [
        pick_possible_goal(h, w, obstacles, rng),
        pick_possible_goal(h, w, obstacles, rng),
        pick_possible_goal(h, w, obstacles, rng),
        pick_possible_goal(h, w, obstacles, rng),
        pick_possible_goal(h, w, obstacles, rng),
    ]
}

// impl Build<OAMDPFiniteHorizon<BakerCommunicationModel<5>, BakerGridMDP, BakerJointAction, 5>>
//     for BakerCOAMDPBuilder<5>
// {
//     fn build(
//         self,
//     ) -> OAMDPFiniteHorizon<BakerCommunicationModel<5>, BakerGridMDP, BakerJointAction, 5> {
//         let mdp = self
//             .partial_mdp
//             .build_from(&self.possible_goals[self.true_goal]);
//
//         let physical_actions = vec![
//             North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest, Stay,
//         ];
//
//         let joint_actions = iproduct!(physical_actions.iter(), self.communication_actions.iter())
//             .map(|(a, b)| BakerJointAction::new(*a, *b))
//             .collect::<Vec<_>>();
//
//         let communication_model_builder = BakerCommunicationModelBuilder::new(
//             self.partial_mdp,
//             SoftmaxPolicyBuilder::new(self.beta),
//             self.communication_type,
//             self.communication_actions,
//             self.communication_cost,
//         );
//
//         let communication_model =
//             communication_model_builder.build(self.possible_goals, self.shapes);
//
//         let oamdp = FiniteHorizonWrapper::new(
//             OAMDP::new(
//                 communication_model,
//                 mdp,
//                 self.belief_cost_function,
//                 [NotNan::from_f32(1.0 / 5.0).unwrap(); 5],
//                 0.9,
//                 joint_actions,
//                 self.cost_type,
//                 //                 Objective::LinearCombination(1.0, 0.1),
//             ),
//             self.max_t,
//         );
//
//         oamdp
//     }
// }
