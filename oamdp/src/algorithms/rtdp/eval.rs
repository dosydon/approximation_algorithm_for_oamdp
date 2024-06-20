use mdp::mdp_traits::StatesActions;
use mdp::mdp_traits::*;
use rand::rngs::ThreadRng;
use std::fmt::Debug;
use std::hash::Hash;

use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;
use crate::oamdp_d::{VState, OAMDPD};

use super::episode_iterator::EpisodeIterator;
use super::RTDP_OAMDP;

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize> Eval
    for RTDP_OAMDP<OM, M, A, H, N>
where
    for<'a> EpisodeIterator<'a, OM, M, A, H, N>:
        Iterator<Item = (BeliefState<M::State, N>, A, BeliefState<M::State, N>, f32)>,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>> + InitialState,
    OAMDPD<OM, M, A, N>: StatesActions<State = VState<M::State, N>> + InitialState,
{
    fn eval(&mut self, rng: &mut ThreadRng) -> f32 {
        let mut cumulative_cost = 0.0;
        let max_t = self.max_t;
        for (_, _, _, c) in self.into_iter_with(rng).set_max_t(max_t) {
            cumulative_cost += c;
        }
        cumulative_cost
    }
}
