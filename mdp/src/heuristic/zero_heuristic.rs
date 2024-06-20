use crate::heuristic::{HeuristicWithMDP, HeuristicWithMDPMut};
use crate::mdp_traits::StatesActions;

#[derive(Copy, Clone)]
pub struct ZeroHeuristic {}

impl<M: StatesActions> HeuristicWithMDP<M> for ZeroHeuristic {
    fn h_with(&self, _s: &M::State, _mdp: &M) -> f32 {
        0.0
    }
}

impl<M: StatesActions> HeuristicWithMDPMut<M> for ZeroHeuristic {
    fn h_with_mut(&mut self, _s: &M::State, _mdp: &mut M) -> f32 {
        0.0
    }
}
