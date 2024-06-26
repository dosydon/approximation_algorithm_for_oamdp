use crate::mdp_traits::StatesActions;

pub trait HeuristicWithMDP<M: StatesActions> {
    fn h_with(&self, s: &M::State, mdp: &M) -> f32;
}
pub trait HeuristicWithMDPMut<M: StatesActions> {
    fn h_with_mut(&mut self, s: &M::State, mdp: &mut M) -> f32;
}
