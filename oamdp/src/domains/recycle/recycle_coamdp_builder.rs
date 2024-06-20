use itertools::iproduct;
use mdp::{
    finite_horizon_wrapper::FiniteHorizonWrapper,
    mdp_traits::{ActionEnumerable, Build},
};
use num_traits::FromPrimitive;
use ordered_float::NotNan;
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::{
    belief_cost_function::{BeliefCostFunction, Objective},
    domains::recycle::{
        builder::RecycleMDPBuilder, Location, RecycleCommunicationAction,
        RecycleCommunicationModel, RecycleJointAction, RecycleMDP,
    },
    oamdp::{oamdp::OAMDP, OAMDPFiniteHorizon},
};

#[derive(Serialize, Deserialize)]
pub struct RecycleCOAMDPBuilder<const NITEM: usize, const N: usize> {
    #[serde(with = "serde_arrays")]
    possible_goals: [[Location; 3]; N],
    beta: f32,
    communication_actions: Vec<RecycleCommunicationAction>,
    max_t: usize,
    builder: RecycleMDPBuilder<NITEM>,
    true_goal: usize,
    belief_cost_function: BeliefCostFunction<N>,
    cost_type: Objective,
    communication_cost: f32,
}

impl<const NITEM: usize, const N: usize> RecycleCOAMDPBuilder<NITEM, N> {
    pub fn set_horizon(mut self, max_t: usize) -> Self {
        self.max_t = max_t;
        self
    }
}

pub(crate) fn pick_available_messages(rng: &mut ThreadRng) -> Vec<RecycleCommunicationAction> {
    let mut messages = vec![RecycleCommunicationAction::None];
    for m in [
        RecycleCommunicationAction::Announce(Location::Compost),
        RecycleCommunicationAction::Announce(Location::Recycle),
        RecycleCommunicationAction::Announce(Location::Trash),
    ] {
        let r = rng.gen_bool(0.8);
        if r {
            messages.push(m)
        }
    }
    messages
}

impl RecycleCOAMDPBuilder<5, 4> {
    pub fn new(instance_id: usize) -> Self {
        let path = format!(
            "{}/src/domains/recycle/coamdp_instances/recycle_{}.yaml",
            env!("CARGO_MANIFEST_DIR"),
            instance_id
        );
        let data = fs::read_to_string(&path).expect("Unable to read file");
        serde_yaml::from_str(&data).expect("Invalid yaml")
    }
}

fn kinds<const N: usize>() -> [usize; N] {
    let mut kinds = [0; N];
    for i in 0..N {
        kinds[i] = i % 3;
    }
    kinds
}

fn initial_locs<const N: usize>(rng: &mut ThreadRng) -> [Location; N] {
    let mut initial_locs = [Location::Compost; N];
    for i in 0..N {
        initial_locs[i] = Location::random_location(rng);
    }
    initial_locs
}

impl<const NITEM: usize> RecycleCOAMDPBuilder<NITEM, 4> {
    pub fn random_instance(rng: &mut ThreadRng) -> Self {
        let success_prob = rng.gen_range(0.3, 0.8);
        let alpha = rng.gen_range(0.0, 1.0);
        let communication_cost = rng.gen_range(0.0, 0.5);

        let builder = RecycleMDPBuilder {
            initial_locs: initial_locs(rng),
            kinds: kinds(),
            actual_success_prob: success_prob,
        };

        RecycleCOAMDPBuilder {
            possible_goals: [
                [Location::Compost, Location::Recycle, Location::Compost],
                [Location::Compost, Location::Recycle, Location::Trash],
                [Location::Compost, Location::Trash, Location::Compost],
                [Location::Compost, Location::Trash, Location::Trash],
            ],
            beta: 0.3,
            communication_actions: pick_available_messages(rng),
            max_t: 20,
            builder: builder,
            true_goal: 1,
            belief_cost_function: BeliefCostFunction::get_legible_cost_function(0),
            cost_type: Objective::LinearCombination(alpha, 1.0 - alpha),
            communication_cost: communication_cost,
        }
    }
}

impl<const NITEM: usize, const N: usize>
    Build<
        OAMDPFiniteHorizon<
            RecycleCommunicationModel<NITEM>,
            RecycleMDP<NITEM>,
            RecycleJointAction,
            N,
        >,
    > for RecycleCOAMDPBuilder<NITEM, N>
{
    fn build(
        self,
    ) -> OAMDPFiniteHorizon<
        RecycleCommunicationModel<NITEM>,
        RecycleMDP<NITEM>,
        RecycleJointAction,
        N,
    > {
        let mdp = self.builder.build(self.possible_goals[self.true_goal]);
        let physical_actions = mdp.enumerate_actions().into_iter().collect::<Vec<_>>();

        let joint_actions = iproduct!(physical_actions.iter(), self.communication_actions.iter())
            .map(|(a, b)| RecycleJointAction::new(**a, *b))
            .collect::<Vec<_>>();

        let communication_model = RecycleCommunicationModel::from_targets(
            self.possible_goals.to_vec(),
            self.communication_cost,
            &self.builder,
        );

        let oamdp = FiniteHorizonWrapper::new(
            OAMDP::new(
                communication_model,
                mdp,
                self.belief_cost_function,
                [NotNan::from_f32(1.0 / (N as f32)).unwrap(); N],
                0.9,
                joint_actions,
                self.cost_type,
                //                 Objective::LinearCombination(1.0, 0.1),
            ),
            self.max_t,
        );

        oamdp
    }
}
