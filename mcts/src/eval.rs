use mdp::{
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, Eval, GetNextStateMut, InitialState,
        IntoEvalMut, IsTerminal, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};
use rand::rngs::ThreadRng;

use crate::MCTS;

impl<M, P> Eval for MCTS<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + GetNextStateMut
        + Cost
        + DCost
        + InitialState
        + ActionEnumerable,
    P: IntoEvalMut<M> + GetActionMut<M::State, M>,
{
    fn eval(&mut self, rng: &mut ThreadRng) -> f32 {
        let mut sum = 0.0;
        for (_, _, _, c) in self.into_iter_with(rng) {
            sum += c;
        }
        self.clear();
        sum
    }
}
