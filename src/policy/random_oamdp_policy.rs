use mdp::{
    episode_runner::EpisodeRunnerMut,
    finite_horizon_wrapper::FiniteHorizonWrapperState,
    into_inner::Inner,
    mdp_traits::{
        ActionEnumerable, DCost, GetNextStateMut, InitialState, IntoEvalMut, IsTerminal,
        StatesActions,
    },
    policy::{
        policy_traits::{GetAction, GetActionMut},
        random_policy::RandomPolicy,
    },
};
use std::fmt::Debug;
use std::hash::Hash;

use crate::oamdp::{BeliefState, OAMDPFiniteHorizon};
pub struct RandomOAMDPPolicy {
    random_policy: RandomPolicy,
}

impl RandomOAMDPPolicy {
    pub fn new() -> RandomOAMDPPolicy {
        RandomOAMDPPolicy {
            random_policy: RandomPolicy {},
        }
    }
}

impl<OM, M, A: Eq + Debug + Hash + Copy, const N: usize>
    IntoEvalMut<OAMDPFiniteHorizon<OM, M, A, N>> for RandomOAMDPPolicy
where
    M: StatesActions + ActionEnumerable,
    A: From<M::Action>,
    OAMDPFiniteHorizon<OM, M, A, N>: StatesActions<State = FiniteHorizonWrapperState<BeliefState<M::State, N>>>
        + IsTerminal
        + InitialState
        + GetNextStateMut
        + DCost,
{
    type Evaluator<'a> =
        EpisodeRunnerMut<'a, OAMDPFiniteHorizon<OM, M, A, N>, RandomOAMDPPolicy> where OM: 'a, M: 'a, A: 'a, BeliefState<M::State, N>: 'a;

    fn into_eval_mut<'a>(
        &'a mut self,
        s: FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        mdp: &'a mut OAMDPFiniteHorizon<OM, M, A, N>,
    ) -> Self::Evaluator<'a> {
        EpisodeRunnerMut::new(mdp, self, s)
    }
}

impl<OM, M: StatesActions + ActionEnumerable, A: Eq + Copy + Debug + Hash, const N: usize>
    GetActionMut<
        FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        OAMDPFiniteHorizon<OM, M, A, N>,
    > for RandomOAMDPPolicy
where
    A: From<M::Action>,
{
    fn get_action_mut(
        &mut self,
        s: &FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        mdp: &mut OAMDPFiniteHorizon<OM, M, A, N>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<A> {
        if let Some(a) = self
            .random_policy
            .get_action(&s.inner().inner(), &mdp.mdp.mdp, rng)
        {
            Some(A::from(a))
        } else {
            None
        }
    }
}
