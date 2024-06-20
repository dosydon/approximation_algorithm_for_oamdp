use mdp::episode_runner::EpisodeRunnerMut;
use mdp::finite_horizon_wrapper::FiniteHorizonWrapperState;
use mdp::mdp_traits::{
    ActionEnumerable, DCost, GetNextStateMut, InitialState, IntoEvalMut, IsTerminal,
};
use mdp::policy::policy_traits::{GetAction, GetActionMut};
use mdp::{into_inner::Inner, mdp_traits::StatesActions, policy::tabular_policy::TabularPolicy};
use rand::rngs::ThreadRng;

use crate::oamdp::{BeliefState, OAMDPFiniteHorizon};

use std::fmt::Debug;
use std::hash::Hash;

pub struct TabularOAMDPPolicy<M: StatesActions> {
    tabular_policy: TabularPolicy<M::State, M::Action>,
}

impl<M: StatesActions> TabularOAMDPPolicy<M> {
    pub fn new(tabular_policy: TabularPolicy<M::State, M::Action>) -> TabularOAMDPPolicy<M> {
        TabularOAMDPPolicy {
            tabular_policy: tabular_policy,
        }
    }
}

impl<
        OM,
        M: StatesActions,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy + From<M::Action>,
        const N: usize,
    >
    GetActionMut<
        FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        OAMDPFiniteHorizon<OM, M, A, N>,
    > for TabularOAMDPPolicy<M>
{
    fn get_action_mut(
        &mut self,
        s: &FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        oamdp: &mut OAMDPFiniteHorizon<OM, M, A, N>,
        rng: &mut ThreadRng,
    ) -> Option<A> {
        let ss = s.inner();
        if let Some(a) = self
            .tabular_policy
            .get_action(&ss.inner(), &oamdp.mdp.mdp, rng)
        {
            Some(A::from(a))
        } else {
            None
        }
    }
}

impl<OM, M, A: Eq + Debug + Hash + Copy, const N: usize>
    IntoEvalMut<OAMDPFiniteHorizon<OM, M, A, N>> for TabularOAMDPPolicy<M>
where
    M: StatesActions + ActionEnumerable,
    A: From<M::Action>,
    OAMDPFiniteHorizon<OM, M, A, N>: StatesActions<Action = A, State = FiniteHorizonWrapperState<BeliefState<M::State, N>>>
        + IsTerminal
        + InitialState
        + GetNextStateMut
        + DCost,
{
    type Evaluator<'a> =
        EpisodeRunnerMut<'a, OAMDPFiniteHorizon<OM, M, A, N>, TabularOAMDPPolicy<M>> where OM: 'a, M: 'a, A: 'a, BeliefState<M::State, N>: 'a;

    fn into_eval_mut<'a>(
        &'a mut self,
        s: FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        mdp: &'a mut OAMDPFiniteHorizon<OM, M, A, N>,
    ) -> Self::Evaluator<'a> {
        EpisodeRunnerMut::new(mdp, self, s)
    }
}
