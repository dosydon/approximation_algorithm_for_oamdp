use crate::mdp_traits::StatesActions;
pub trait ValueEstimator<S, A, M: StatesActions> {
    fn get_qsa(&self, s: &S, a: &A, mdp: &M) -> f32;
    fn get_max_qsa(&self, s: &S, mdp: &M) -> f32;
}
pub trait UpdateValue<M: StatesActions> {
    fn update(&mut self, s: &M::State, a: &M::Action, mdp: &mut M);
}

pub trait CostEstimator<M: StatesActions> {
    fn get_qsa_ssp(&self, s: &M::State, a: &M::Action, mdp: &M) -> f32;
    fn get_value_ssp(&self, s: &M::State, mdp: &M) -> f32;
}

pub trait CostEstimatorMut<M: StatesActions> {
    fn get_qsa_ssp_mut(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32;
    fn get_value_ssp_mut(&mut self, s: &M::State, mdp: &mut M) -> f32;
}
