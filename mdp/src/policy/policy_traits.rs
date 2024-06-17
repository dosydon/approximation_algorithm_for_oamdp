use crate::mdp_traits::StatesActions;
use rand::prelude::*;

pub trait Policy<A, M: StatesActions> {
    fn get_probability(&self, s: &M::State, a: &A, mdp: &M) -> f32;
}

pub trait PolicyMut<A, M: StatesActions> {
    fn get_probability_mut(&mut self, s: &M::State, a: &A, mdp: &mut M) -> f32;
}

pub trait PartialPolicy<M: StatesActions> {
    fn get_probability_maybe(&self, s: &M::State, a: &M::Action, mdp: &M) -> Option<f32>;
}

pub trait GetAction<S, M: StatesActions> {
    fn get_action(&self, s: &S, mdp: &M, rng: &mut ThreadRng) -> Option<M::Action>;
}

pub trait GetActionMut<S, M: StatesActions> {
    fn get_action_mut(&mut self, s: &S, mdp: &mut M, rng: &mut ThreadRng) -> Option<M::Action>;
}
