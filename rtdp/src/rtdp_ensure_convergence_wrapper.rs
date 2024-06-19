use crate::rtdp::RTDP;
use mdp::heuristic::HeuristicWithMDPMut;
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, GetNextStateMut, IsTerminal, PMassMut,
    StatesActions,
};
use mdp::policy::policy_traits::GetActionMut;
use rand::rngs::ThreadRng;
use std::fmt::Debug;
use std::hash::Hash;

pub struct RTDPEnsureConvergenceWrapper<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> {
    pub rtdp: RTDP<S, H>,
    epsilon: f32,
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDPEnsureConvergenceWrapper<S, H> {
    pub fn new(rtdp: RTDP<S, H>, epsilon: f32) -> RTDPEnsureConvergenceWrapper<S, H> {
        RTDPEnsureConvergenceWrapper { rtdp, epsilon }
    }
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, M, H> GetActionMut<S, M>
    for RTDPEnsureConvergenceWrapper<S, H>
where
    M: IsTerminal
        + GetNextStateMut
        + PMassMut<f32>
        + Cost
        + ActionEnumerable
        + ActionAvailability
        + StatesActions<State = S>,
    H: HeuristicWithMDPMut<M>,
{
    fn get_action_mut(&mut self, s: &S, mdp: &mut M, rng: &mut ThreadRng) -> Option<M::Action> {
        self.rtdp.lrtdp_inner(*s, mdp, 0, rng, self.epsilon);
        self.rtdp.best_action_mut(s, mdp)
    }
}
