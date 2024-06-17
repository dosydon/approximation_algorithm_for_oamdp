use mdp::{
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, GetNextStateMut, InitialState,
        IsTerminal, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};
use rand::rngs::ThreadRng;

use crate::{MCTSEpisodeIterator, MCTS};

impl<M, P> MCTS<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + GetNextStateMut
        + Cost
        + DCost
        + InitialState
        + ActionEnumerable,
    P: GetActionMut<M::State, M>,
{
    pub fn into_iter_with<'a, 'b>(
        &'a mut self,
        rng: &'a mut ThreadRng,
    ) -> MCTSEpisodeIterator<'b, M, P>
    where
        'a: 'b,
    {
        MCTSEpisodeIterator::from_initial_state(self, self.budget, rng)
    }
}
