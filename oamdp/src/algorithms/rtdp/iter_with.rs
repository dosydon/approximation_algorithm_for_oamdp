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

impl<
        'a,
        'b,
        OM,
        M: StatesActions,
        A: PartialEq + Eq + Copy + Clone + Debug + Hash,
        H,
        const N: usize,
    > IntoIterWith<'b> for &'a mut RTDP_OAMDP<OM, M, A, H, N>
where
    'b: 'a,
    EpisodeIterator<'a, OM, M, A, H, N>:
        Iterator<Item = (BeliefState<M::State, N>, A, BeliefState<M::State, N>, f32)>,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>> + InitialState,
    OAMDPD<OM, M, A, N>: StatesActions<State = VState<M::State, N>> + InitialState,
{
    type Item = (BeliefState<M::State, N>, A, BeliefState<M::State, N>, f32);
    type I = EpisodeIterator<'a, OM, M, A, H, N>;

    fn into_iter_with(self, rng: &'b mut ThreadRng) -> EpisodeIterator<'a, OM, M, A, H, N> {
        let bs = self.oamdp_d.oamdp.initial_state();
        let vs = self.oamdp_d.random_transition_to_v_state(&bs, rng);
        EpisodeIterator::from_initial_state(self, vs, bs, rng)
    }
}
