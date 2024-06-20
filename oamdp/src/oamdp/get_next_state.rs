use crate::traits::BeliefOverGoal;
use core::fmt::Debug;
use core::hash::Hash;

use mdp::into_inner::Inner;
use mdp::mdp_traits::*;

use crate::traits::ProbSassGivenTheta;

use super::oamdp::OAMDP;
use super::BeliefState;

impl<
        OM,
        M: GetNextState,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy + Inner<Result = M::Action>,
        const N: usize,
    > GetNextState for OAMDP<OM, M, A, N>
where
    Self: StatesActions<State = BeliefState<M::State, N>, Action = A> + ActionEnumerable,
    for<'a> &'a OM: ProbSassGivenTheta<M::State, A>,
{
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Self::State {
        let new_s = self.mdp.get_next_state(&s.inner(), &a.inner(), rng);
        let new_belief = self.get_new_belief(&s.get_belief_over_goal(), &s.inner(), a, &new_s);
        Self::State::new(new_s, new_belief)
    }
}

impl<
        OM,
        M: GetNextStateMut,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy + Inner<Result = M::Action>,
        const N: usize,
    > GetNextStateMut for OAMDP<OM, M, A, N>
where
    Self: StatesActions<State = BeliefState<M::State, N>, Action = A> + ActionEnumerable,
    for<'a> &'a mut OM: ProbSassGivenTheta<M::State, A>,
{
    fn get_next_state_mut(
        &mut self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Self::State {
        let new_s = self.mdp.get_next_state_mut(&s.inner(), &a.inner(), rng);
        let new_belief = self.get_new_belief_mut(&s.get_belief_over_goal(), &s.inner(), a, &new_s);
        Self::State::new(new_s, new_belief)
    }
}
