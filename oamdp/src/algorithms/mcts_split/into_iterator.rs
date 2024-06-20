use mdp::{
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, GetNextStateMut, InitialState,
        IsTerminal, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};
use rand::rngs::ThreadRng;

use crate::traits::{DomainAction, Message};

use super::{
    mcts_am::MCTSAM, mcts_am_episode_iterator::MCTSAMEpisodeIterator,
    mcts_ma_episode_iterator::MCTSMAEpisodeIterator, MCTSMA,
};

impl<M, P> MCTSMA<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + DomainAction
        + Message
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
    ) -> MCTSMAEpisodeIterator<'b, M, P>
    where
        'a: 'b,
    {
        MCTSMAEpisodeIterator::from_initial_state(self, rng)
    }
}

impl<M, P> MCTSAM<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + DomainAction
        + Message
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
    ) -> MCTSAMEpisodeIterator<'b, M, P>
    where
        'a: 'b,
    {
        MCTSAMEpisodeIterator::from_initial_state(self, rng)
    }
}
