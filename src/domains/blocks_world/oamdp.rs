use mdp::{
    blocks_world::{BlocksWorldAction, BlocksWorldMDPN, BlocksWorldStateN},
    heuristic::ZeroHeuristic,
    policy::softmax_policy::SoftmaxPolicy,
    state_enumerable_wrapper::StateEnumerableWrapper,
    value_iteration::ValueTable,
};
use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicy;

use crate::{oamdp::OAMDPFiniteHorizon, observer_model::ImplicitCommunicationModel};

pub type OAMDPBlocksFiniteHorizon<const NB: usize, const N: usize> = OAMDPFiniteHorizon<
    ImplicitCommunicationModel<
        RTDPSoftmaxPolicy<BlocksWorldStateN<NB>, ZeroHeuristic>,
        BlocksWorldMDPN<NB>,
        N,
    >,
    BlocksWorldMDPN<NB>,
    BlocksWorldAction,
    N,
>;

pub type OAMDPBlocksStateEnumerableFiniteHorizon<const NB: usize, const N: usize> =
    OAMDPFiniteHorizon<
        ImplicitCommunicationModel<
            SoftmaxPolicy<ValueTable<BlocksWorldStateN<NB>>>,
            StateEnumerableWrapper<BlocksWorldMDPN<NB>>,
            N,
        >,
        StateEnumerableWrapper<BlocksWorldMDPN<NB>>,
        BlocksWorldAction,
        N,
    >;
