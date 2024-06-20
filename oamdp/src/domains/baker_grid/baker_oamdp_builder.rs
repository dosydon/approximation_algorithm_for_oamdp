use std::fs;

use mdp::{
    baker_grid::{BakerGridAction, BakerGridMDP, BakerGridPartialMDP, BakerGridState},
    finite_horizon_wrapper::FiniteHorizonWrapper,
    mdp_traits::Build,
    policy::softmax_policy::SoftmaxPolicyBuilder,
};
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    belief_cost_function::{BeliefCostType, Objective},
    belief_update_type::ObserveabilityAssumption,
    oamdp::{oamdp::OAMDP, OAMDPFiniteHorizon},
    observer_model::SoftmaxModel,
};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct BakerOAMDPBuilder<const N: usize> {
    pub(crate) beta: f32,
    pub(crate) gamma: f32,
    pub(crate) observability_assumption: ObserveabilityAssumption,
    pub(crate) belief_cost_type: BeliefCostType,
    pub(crate) horizon: usize,
    pub(crate) random: Option<usize>,
    pub(crate) prob_veering: f32,
    pub(crate) partial_mdp: BakerGridPartialMDP,
    #[serde_as(as = "[_; N]")]
    pub possible_goals: [BakerGridState; N],
    pub(crate) true_goal: usize,
    #[serde_as(as = "[_; N]")]
    pub(crate) initial_belief: [NotNan<f32>; N],
    pub(crate) objective: Objective,
}

impl<const N: usize> BakerOAMDPBuilder<N> {
    pub fn new(instance_id: usize) -> Self {
        let path = format!(
            "{}/src/domains/baker_grid/oamdp_instances/{}.yaml",
            env!("CARGO_MANIFEST_DIR"),
            instance_id
        );
        let data = fs::read_to_string(&path).expect("Unable to read file");
        serde_yaml::from_str(&data).expect("Invalid yaml")
    }

    pub fn set_observabaility_assumption(
        mut self,
        observability_assumption: ObserveabilityAssumption,
    ) -> BakerOAMDPBuilder<N> {
        self.observability_assumption = observability_assumption;

        self
    }
    pub fn set_gamma(mut self, gamma: f32) -> Self {
        self.gamma = gamma;

        self
    }

    pub fn set_horizon(mut self, horizon: usize) -> Self {
        self.horizon = horizon;

        self
    }

    pub fn set_distance_measure(mut self, distance_measure: BeliefCostType) -> Self {
        self.belief_cost_type = distance_measure;

        self
    }

    pub fn set_random(mut self, random: Option<usize>) -> Self {
        self.random = random;

        self
    }

    pub fn set_prob_veering(mut self, prob_veering: f32) -> Self {
        self.prob_veering = prob_veering;

        self
    }

    pub fn set_belief_cost_type(mut self, belief_cost_type: BeliefCostType) -> Self {
        self.belief_cost_type = belief_cost_type;
        self
    }

    pub fn set_objective(mut self, objective: Objective) -> Self {
        self.objective = objective;
        self
    }
}

impl<const N: usize>
    Build<OAMDPFiniteHorizon<SoftmaxModel<BakerGridMDP, N>, BakerGridMDP, BakerGridAction, N>>
    for BakerOAMDPBuilder<N>
{
    fn build(
        self,
    ) -> OAMDPFiniteHorizon<SoftmaxModel<BakerGridMDP, N>, BakerGridMDP, BakerGridAction, N> {
        FiniteHorizonWrapper::new(self.build_oamdp(), self.horizon)
    }
}

impl<const N: usize> BakerOAMDPBuilder<N> {
    pub fn build_oamdp(
        &self,
    ) -> OAMDP<SoftmaxModel<BakerGridMDP, N>, BakerGridMDP, BakerGridAction, N> {
        let partial_mdp = self.partial_mdp.clone().set_prob_veering(self.prob_veering);
        let softmax_policy = SoftmaxPolicyBuilder::new(self.beta);
        let possible_goals = self.possible_goals;

        let mut oamdp = OAMDP::new_with_initial_belief(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            self.true_goal,
            self.belief_cost_type.clone(),
            self.objective,
            self.observability_assumption,
            self.initial_belief,
        );
        oamdp.gamma = self.gamma;
        if let Some(i) = self.random {
            oamdp.assumed_model.assumed_policy[i].beta = 0.0;
        }
        oamdp
    }
}
