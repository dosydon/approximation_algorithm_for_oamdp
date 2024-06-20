use mcts::RunEpisode;
use mdp::{
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, DisplayState, GetNextStateMut,
        InitialState, IntoEvalMut, IsTerminal, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};
use rand::rngs::ThreadRng;

use crate::traits::{DomainAction, EnumerateDomainAction, EnumerateMessage, Message, Set};

use super::{mcts_am::MCTSAM, MCTSMA};

impl<M, P> RunEpisode for MCTSMA<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + DomainAction
        + Message
        + GetNextStateMut
        + Cost
        + EnumerateDomainAction
        + EnumerateMessage
        + DCost
        + InitialState
        + ActionEnumerable
        + DisplayState<M::State>,
    P: IntoEvalMut<M> + GetActionMut<M::State, M>,
    M::Action: From<(M::DomainAction, M::Message)> + Set<M::Message>,
{
    fn run_episode(&mut self, rng: &mut ThreadRng) -> f32 {
        let mut sum = 0.0;
        unsafe {
            let self_p = self as *const Self;
            for (_s, a, ss, c) in self.into_iter_with(rng) {
                sum += c;
                println!("{:?}", a);
                (*self_p).display(&ss);
            }
        }
        sum
    }
}

impl<M, P> RunEpisode for MCTSAM<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + DomainAction
        + Message
        + GetNextStateMut
        + EnumerateDomainAction
        + EnumerateMessage
        + Cost
        + DCost
        + InitialState
        + ActionEnumerable
        + DisplayState<M::State>,
    P: IntoEvalMut<M> + GetActionMut<M::State, M>,
    M::Action: From<(M::DomainAction, M::Message)> + Set<M::DomainAction>,
{
    fn run_episode(&mut self, rng: &mut ThreadRng) -> f32 {
        let mut sum = 0.0;
        unsafe {
            let self_p = self as *const Self;
            for (_s, a, ss, c) in self.into_iter_with(rng) {
                sum += c;
                println!("{:?}", a);
                (*self_p).display(&ss);
            }
        }
        sum
    }
}
