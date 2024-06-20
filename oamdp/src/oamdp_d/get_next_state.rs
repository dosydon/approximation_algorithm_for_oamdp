use mdp::{
    into_inner::Inner,
    mdp_traits::{GetNextState, GetNextStateMut, InitialState, StatesActions},
};
use rand::seq::SliceRandom;

use std::fmt::Debug;
use std::hash::Hash;

use crate::{
    oamdp::{oamdp::OAMDP, BeliefState},
    traits::BeliefOverGoal,
};

use super::{VState, OAMDPD};

impl<OM, M: StatesActions, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    OAMDPD<OM, M, A, N>
{
    pub fn random_transition_to_v_state(
        &self,
        bs: &BeliefState<M::State, N>,
        rng: &mut rand::prelude::ThreadRng,
    ) -> VState<M::State, N> {
        let pairs = self
            .translator
            .get_corner_and_lambdas(&bs.get_belief_over_goal());

        if let Ok(pair) = pairs.choose_weighted(rng, |(_v, w)| *w) {
            VState::new(bs.inner(), pair.0)
        } else {
            panic!("{:?}", pairs);
        }
    }
}

impl<OM, M: StatesActions, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    GetNextState for OAMDPD<OM, M, A, N>
where
    OAMDP<OM, M, A, N>:
        GetNextState + InitialState + StatesActions<State = BeliefState<M::State, N>, Action = A>,
{
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Self::State {
        if s.is_dummy_initial_state {
            let bs = self.oamdp.initial_state();
            self.random_transition_to_v_state(&bs, rng)
        } else {
            let bs = self.to_belief_state(s);
            let bss = self.oamdp.get_next_state(&bs, a, rng);
            self.random_transition_to_v_state(&bss, rng)
        }
    }
}

impl<OM, M: StatesActions, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    GetNextStateMut for OAMDPD<OM, M, A, N>
where
    OAMDP<OM, M, A, N>: GetNextStateMut
        + InitialState
        + StatesActions<State = BeliefState<M::State, N>, Action = A>,
{
    fn get_next_state_mut(
        &mut self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Self::State {
        if s.is_dummy_initial_state {
            let bs = self.oamdp.initial_state();
            self.random_transition_to_v_state(&bs, rng)
        } else {
            let bs = self.to_belief_state(s);
            let bss = self.oamdp.get_next_state_mut(&bs, a, rng);
            self.random_transition_to_v_state(&bss, rng)
        }
    }
}
