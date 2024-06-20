use mdp::{
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, Eval, GetNextStateMut, InitialState,
        IntoEvalMut, IsTerminal, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};
use rand::rngs::ThreadRng;

use crate::traits::{DomainAction, EnumerateDomainAction, EnumerateMessage, Message, Set};

use super::{MCTSAM, MCTSMA};

impl<M, P> Eval for MCTSMA<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + GetNextStateMut
        + DomainAction
        + Message
        + EnumerateDomainAction
        + EnumerateMessage
        + Cost
        + DCost
        + InitialState
        + ActionEnumerable,
    P: GetActionMut<M::State, M> + IntoEvalMut<M>,
    M::Action: From<(M::DomainAction, M::Message)> + Set<M::Message>,
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

impl<M, P> Eval for MCTSAM<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + GetNextStateMut
        + DomainAction
        + Message
        + EnumerateDomainAction
        + EnumerateMessage
        + Cost
        + DCost
        + InitialState
        + ActionEnumerable,
    P: GetActionMut<M::State, M> + IntoEvalMut<M>,
    M::Action: From<(M::DomainAction, M::Message)> + Set<M::DomainAction>,
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
