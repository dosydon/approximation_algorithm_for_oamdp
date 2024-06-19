use crate::{
    episode_runner::{EpisodeRunner, EpisodeRunnerMut},
    mdp_traits::{
        DCost, GetNextState, GetNextStateMut, InitialState, IntoEval, IntoEvalMut, IsTerminal,
        StatesActions,
    },
    policy::{
        policy_traits::GetAction,
        private_traits::{GetActionMutFrom, Sealed},
    },
};

use super::{FiniteHorizonWrapper, FiniteHorizonWrapperState};

pub struct FiniteHorizonPolicyWrapper<P> {
    policy: P,
}

impl<P> FiniteHorizonPolicyWrapper<P> {
    pub fn new(policy: P) -> Self {
        Self { policy }
    }
}

impl<P: GetAction<M::State, M>, M: StatesActions>
    GetAction<FiniteHorizonWrapperState<M::State>, FiniteHorizonWrapper<M>>
    for FiniteHorizonPolicyWrapper<P>
{
    fn get_action(
        &self,
        s: &FiniteHorizonWrapperState<M::State>,
        mdp: &FiniteHorizonWrapper<M>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<<FiniteHorizonWrapper<M> as StatesActions>::Action> {
        self.policy.get_action(&s.s, &mdp.mdp, rng)
    }
}

impl<P> Sealed for FiniteHorizonPolicyWrapper<P> {}
impl<P: GetAction<M::State, M>, M: StatesActions>
    GetActionMutFrom<FiniteHorizonWrapperState<M::State>, FiniteHorizonWrapper<M>>
    for FiniteHorizonPolicyWrapper<P>
{
}

impl<
        M: DCost + IsTerminal + StatesActions + InitialState + GetNextState,
        P: GetAction<M::State, M>,
    > IntoEval<FiniteHorizonWrapper<M>> for FiniteHorizonPolicyWrapper<P>
{
    type Evaluator<'a> = EpisodeRunner<'a, FiniteHorizonWrapper<M>, FiniteHorizonPolicyWrapper<P>> where M: 'a, P: 'a;

    fn into_eval<'a>(
        &'a self,
        s: FiniteHorizonWrapperState<M::State>,
        mdp: &'a mut FiniteHorizonWrapper<M>,
    ) -> Self::Evaluator<'a> {
        EpisodeRunner::new(mdp, &self, s)
    }
}

impl<
        M: DCost + IsTerminal + StatesActions + InitialState + GetNextStateMut,
        P: GetAction<M::State, M>,
    > IntoEvalMut<FiniteHorizonWrapper<M>> for FiniteHorizonPolicyWrapper<P>
{
    type Evaluator<'a> = EpisodeRunnerMut<'a, FiniteHorizonWrapper<M>, FiniteHorizonPolicyWrapper<P>> where M: 'a, P: 'a;

    fn into_eval_mut<'a>(
        &'a mut self,
        s: FiniteHorizonWrapperState<M::State>,
        mdp: &'a mut FiniteHorizonWrapper<M>,
    ) -> Self::Evaluator<'a> {
        EpisodeRunnerMut::new(mdp, self, s)
    }
}
