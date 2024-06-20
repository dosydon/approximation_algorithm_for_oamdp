use mdp::{
    blocks_world::BlocksWorldPartialMDPN, finite_horizon_wrapper::FiniteHorizonWrapper,
    mdp_traits::Build, policy::softmax_policy::SoftmaxPolicyBuilder,
};
use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicyBuilder;

use crate::{
    belief_cost_function::{BeliefCostType, Objective},
    belief_update_type::ObserveabilityAssumption,
    oamdp::oamdp::OAMDP,
};

use super::{oamdp::OAMDPBlocksStateEnumerableFiniteHorizon, OAMDPBlocksFiniteHorizon};

pub struct BlocksOAMDPBuilder<PB, const NB: usize, const N: usize> {
    pub(crate) policy_builder: PB,
    pub(crate) observability_assumption: ObserveabilityAssumption,
    pub(crate) belief_cost_type: BeliefCostType,
    pub(crate) horizon: usize,
    pub(crate) partial_mdp: BlocksWorldPartialMDPN<NB>,
    pub possible_goals: [[char; NB]; N],
    pub(crate) true_goal: usize,
    pub(crate) objective: Objective,
}

impl<PB, const NB: usize, const N: usize> BlocksOAMDPBuilder<PB, NB, N> {
    pub fn set_horizon(mut self, horizon: usize) -> Self {
        self.horizon = horizon;
        self
    }
}

impl<const NB: usize, const N: usize> Build<OAMDPBlocksFiniteHorizon<NB, N>>
    for BlocksOAMDPBuilder<RTDPSoftmaxPolicyBuilder, NB, N>
{
    fn build(self) -> OAMDPBlocksFiniteHorizon<NB, N> {
        let oamdp = OAMDP::new_implicit_model(
            &self.partial_mdp,
            &self.policy_builder,
            self.possible_goals,
            self.true_goal,
            self.belief_cost_type,
            self.objective,
            self.observability_assumption,
        );

        FiniteHorizonWrapper::new(oamdp, self.horizon)
    }
}

impl<const NB: usize, const N: usize> Build<OAMDPBlocksStateEnumerableFiniteHorizon<NB, N>>
    for BlocksOAMDPBuilder<SoftmaxPolicyBuilder, NB, N>
{
    fn build(self) -> OAMDPBlocksStateEnumerableFiniteHorizon<NB, N> {
        let oamdp = OAMDP::new_implicit_model(
            &self.partial_mdp,
            &self.policy_builder,
            self.possible_goals,
            self.true_goal,
            self.belief_cost_type,
            self.objective,
            self.observability_assumption,
        );

        FiniteHorizonWrapper::new(oamdp, self.horizon)
    }
}
