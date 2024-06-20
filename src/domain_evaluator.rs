use std::fmt::Debug;
use std::hash::Hash;

use mdp::episode_runner::EpisodeRunnerMut;
use mdp::into_inner::Inner;
use mdp::mdp_traits::{DCost, GetNextStateMut, InitialState, IntoEvalMut, IsTerminal};
use mdp::policy::policy_traits::GetActionMut;

use mdp::{finite_horizon_wrapper::FiniteHorizonWrapperState, mdp_traits::StatesActions};

use crate::oamdp::{BeliefState, OAMDPFiniteHorizon};
pub struct DomainEvaluator<P> {
    policy: P,
}

impl<P> DomainEvaluator<P> {
    pub fn new(policy: P) -> Self {
        Self { policy }
    }
}

impl<P, OM, M: StatesActions, A: Eq + PartialEq + Debug + Hash + Copy + Clone, const N: usize>
    GetActionMut<
        FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        OAMDPFiniteHorizon<OM, M, A, N>,
    > for DomainEvaluator<P>
where
    A: From<M::Action>,
    P: GetActionMut<M::State, M>,
{
    fn get_action_mut(
        &mut self,
        s: &FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        mdp: &mut OAMDPFiniteHorizon<OM, M, A, N>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<A> {
        if let Some(a) = self
            .policy
            .get_action_mut(&s.inner().inner(), &mut mdp.mdp.mdp, rng)
        {
            Some(A::from(a))
        } else {
            None
        }
    }
}

impl<
        P,
        OM,
        M: StatesActions + IsTerminal + InitialState + GetNextStateMut + DCost,
        A: Eq + PartialEq + Debug + Hash + Copy + Clone,
        const N: usize,
    > IntoEvalMut<OAMDPFiniteHorizon<OM, M, A, N>> for DomainEvaluator<P>
where
    P: GetActionMut<M::State, M>,
{
    type Evaluator<'a> = EpisodeRunnerMut<'a, M, P> where OM: 'a, M: 'a, M::State: 'a, M::Action: 'a, A: 'a, P: 'a;

    fn into_eval_mut<'a>(
        &'a mut self,
        s: FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        mdp: &'a mut OAMDPFiniteHorizon<OM, M, A, N>,
    ) -> Self::Evaluator<'a> {
        EpisodeRunnerMut::new(&mut mdp.mdp.mdp, &mut self.policy, s.inner().inner())
    }
}
