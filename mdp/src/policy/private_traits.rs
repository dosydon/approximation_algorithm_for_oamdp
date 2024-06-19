use rand::rngs::ThreadRng;

use crate::mdp_traits::StatesActions;

use super::policy_traits::{GetAction, GetActionMut};

pub(crate) trait Sealed {}

pub(crate) trait GetActionMutFrom<S, M: StatesActions>: GetAction<S, M> {
    fn get_action_mut_from(
        &mut self,
        s: &S,
        mdp: &mut M,
        rng: &mut ThreadRng,
    ) -> Option<M::Action> {
        self.get_action(s, mdp, rng)
    }
}

impl<M: StatesActions, P, S> GetActionMut<S, M> for P
where
    P: GetActionMutFrom<S, M> + Sealed,
{
    fn get_action_mut(
        &mut self,
        s: &S,
        mdp: &mut M,
        rng: &mut ThreadRng,
    ) -> Option<<M as StatesActions>::Action> {
        self.get_action_mut_from(s, mdp, rng)
    }
}
