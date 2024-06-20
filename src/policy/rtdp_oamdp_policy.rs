use std::fmt::Debug;
use std::hash::Hash;

use mdp::episode_runner::EpisodeRunnerMut;
use mdp::heuristic::HeuristicWithMDPMut;
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, DCost, GetNextStateMut, InitialState, IntoEvalMut,
    IsTerminal, PMassMut, StatesActions,
};
use mdp::policy::policy_traits::{GetAction, GetActionMut};
use mdp::policy::random_policy::RandomPolicy;
use mdp::{finite_horizon_wrapper::FiniteHorizonWrapperState, into_inner::Inner};

use rand::rngs::ThreadRng;
use rtdp::rtdp::RTDP;

use crate::oamdp::{BeliefState, OAMDPFiniteHorizon};
pub struct RTDPOAMDPPolicy<S: Eq + Copy + Clone + Debug + Hash, H> {
    rtdp: RTDP<S, H>,
}

impl<S: Eq + Copy + Clone + Debug + Hash, H> RTDPOAMDPPolicy<S, H> {
    pub fn new(rtdp: RTDP<S, H>) -> RTDPOAMDPPolicy<S, H> {
        RTDPOAMDPPolicy { rtdp }
    }
}

impl<
        OM,
        M: StatesActions
            + ActionEnumerable
            + Cost
            + IsTerminal
            + PMassMut<f32>
            + ActionAvailability
            + GetNextStateMut,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy + From<M::Action>,
        H: HeuristicWithMDPMut<M>,
        const N: usize,
    >
    GetActionMut<
        FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        OAMDPFiniteHorizon<OM, M, A, N>,
    > for RTDPOAMDPPolicy<M::State, H>
{
    fn get_action_mut(
        &mut self,
        s: &FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        oamdp: &mut OAMDPFiniteHorizon<OM, M, A, N>,
        rng: &mut ThreadRng,
    ) -> Option<A> {
        let ss = s.inner();
        self.rtdp
            .lrtdp_inner(ss.inner(), &mut oamdp.mdp.mdp, 0, rng, 1e-3);
        if let Some(a) = self
            .rtdp
            .get_action_mut(&ss.inner(), &mut oamdp.mdp.mdp, rng)
        {
            Some(A::from(a))
        } else {
            let random_policy = RandomPolicy {};
            random_policy.get_action(s, oamdp, rng)
        }
    }
}

impl<OM, M, A: Eq + Debug + Hash + Copy, H: HeuristicWithMDPMut<M>, const N: usize>
    IntoEvalMut<OAMDPFiniteHorizon<OM, M, A, N>> for RTDPOAMDPPolicy<M::State, H>
where
    M: IsTerminal
        + GetNextStateMut
        + StatesActions
        + ActionEnumerable
        + ActionAvailability
        + Cost
        + PMassMut<f32>,
    A: From<M::Action>,
    OAMDPFiniteHorizon<OM, M, A, N>: StatesActions<Action = A, State = FiniteHorizonWrapperState<BeliefState<M::State, N>>>
        + IsTerminal
        + InitialState
        + GetNextStateMut
        + DCost,
{
    type Evaluator<'a> =
        EpisodeRunnerMut<'a, OAMDPFiniteHorizon<OM, M, A, N>, RTDPOAMDPPolicy<M::State, H>> where OM: 'a, M: 'a, A: 'a, BeliefState<M::State, N>: 'a, H: 'a;

    fn into_eval_mut<'a>(
        &'a mut self,
        s: FiniteHorizonWrapperState<BeliefState<M::State, N>>,
        mdp: &'a mut OAMDPFiniteHorizon<OM, M, A, N>,
    ) -> Self::Evaluator<'a> {
        EpisodeRunnerMut::new(mdp, self, s)
    }
}
