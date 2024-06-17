use super::blocks_world_mdp::BlocksWorldMDPN;
use crate::mdp_traits::{ExplicitTransition};

impl<const N: usize> ExplicitTransition for BlocksWorldMDPN<N> {}
