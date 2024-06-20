use itertools::iproduct;
use mdp::common::coordinate2::Coordinate2;
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
use mdp::mdp_traits::Build;
use mdp::mdp_traits::BuildFrom;
use mdp::spelling::Letter;
use mdp::spelling::Letter::*;
use mdp::spelling::SpellingAction::*;
use mdp::spelling::SpellingMDPBuilder;
use mdp::spelling::SpellingMDPE;
use mdp::spelling::SpellingState;
use num_traits::FromPrimitive;
use ordered_float::NotNan;

use crate::belief_cost_function::BeliefCostFunction;
use crate::belief_cost_function::Objective;
use crate::domains::spelling::communication_action::SpellingCommunicationAction;
use crate::domains::spelling::communication_model::SpellingCommunicationModel;
use crate::domains::spelling::joint_action::SpellingJointAction;
use crate::oamdp::oamdp::OAMDP;

use crate::oamdp::OAMDPFiniteHorizon;

pub struct SpellingCOAMDPBuilder<const N: usize> {
    possible_goals: [[Letter; 4]; N],
    beta: f32,
    communication_actions: Vec<SpellingCommunicationAction>,
    max_t: usize,
    mdp: SpellingMDPE<4>,
    builder: SpellingMDPBuilder<4>,
    belief_cost_function: BeliefCostFunction<N>,
    cost_type: Objective,
    communication_cost: f32,
}

impl<const N: usize> SpellingCOAMDPBuilder<N> {
    pub fn set_horizon(mut self, max_t: usize) -> Self {
        self.max_t = max_t;
        self
    }
}

fn get_possible_goals(_id: usize) -> [[Letter; 4]; 3] {
    [[A, R, M, S], [R, A, M, S], [M, A, R, S]]
}

fn get_true_goal(id: usize) -> usize {
    match id {
        1 => 0,
        2 => 0,
        3 => 2,
        4 => 0,
        5 => 0,
        6 => 1,
        7 => 2,
        8 => 0,
        9 => 1,
        10 => 2,
        _ => panic!("not matching id"),
    }
}

fn get_builder(id: usize) -> SpellingMDPBuilder<4> {
    match id {
        1 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        2 => SpellingMDPBuilder::new(
            5,
            5,
            vec![
                Coordinate2::new(1, 1),
                Coordinate2::new(2, 1),
                Coordinate2::new(3, 1),
                Coordinate2::new(1, 3),
                Coordinate2::new(2, 3),
                Coordinate2::new(3, 3),
            ],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        3 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        4 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]),
        ),
        5 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        8 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        6 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        9 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        7 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        10 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        _ => panic!("not matching id"),
    }
}

fn get_belief_cost_function(id: usize) -> BeliefCostFunction<3> {
    match id {
        3 => BeliefCostFunction::Disimulation,
        8 => BeliefCostFunction::Disimulation,
        9 => BeliefCostFunction::Disimulation,
        10 => BeliefCostFunction::Disimulation,
        _ => BeliefCostFunction::get_legible_cost_function(get_true_goal(id)),
    }
}

fn get_objective(_id: usize) -> Objective {
    Objective::LinearCombination(1.0, 1.0)
}

impl SpellingCOAMDPBuilder<3> {
    pub fn new(instance_id: usize) -> Self {
        let true_goal = get_true_goal(instance_id);
        let builder = get_builder(instance_id);
        let possible_goals = get_possible_goals(instance_id);
        let mdp = builder.build_from(&possible_goals[true_goal]);

        SpellingCOAMDPBuilder {
            possible_goals: possible_goals,
            beta: 0.3,
            communication_actions: vec![
                SpellingCommunicationAction::None,
                SpellingCommunicationAction::Announce(A),
                SpellingCommunicationAction::Announce(R),
                SpellingCommunicationAction::Announce(M),
                SpellingCommunicationAction::Announce(S),
            ],
            builder: builder,
            max_t: 20,
            mdp: mdp,
            belief_cost_function: get_belief_cost_function(instance_id),
            cost_type: get_objective(instance_id),
            communication_cost: 0.25,
        }
    }
}

impl
    Build<
        OAMDPFiniteHorizon<
            SpellingCommunicationModel<4, 3>,
            SpellingMDPE<4>,
            SpellingJointAction,
            3,
        >,
    > for SpellingCOAMDPBuilder<3>
{
    fn build(
        self,
    ) -> OAMDPFiniteHorizon<SpellingCommunicationModel<4, 3>, SpellingMDPE<4>, SpellingJointAction, 3>
    {
        let physical_actions = vec![
            North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest, Stay, Toggle,
        ];

        let joint_actions = iproduct!(physical_actions.iter(), self.communication_actions.iter())
            .map(|(a, b)| SpellingJointAction::new(*a, *b))
            .collect::<Vec<_>>();

        let communication_model = SpellingCommunicationModel::from_targets(
            &self.builder,
            self.possible_goals,
            self.communication_cost,
            self.beta,
        );

        let oamdp = FiniteHorizonWrapper::new(
            OAMDP::new(
                communication_model,
                self.mdp,
                self.belief_cost_function,
                [NotNan::from_f32(1.0 / 3.0).unwrap(); 3],
                0.9,
                joint_actions,
                self.cost_type,
            ),
            self.max_t,
        );

        oamdp
    }
}
