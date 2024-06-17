use rand::rngs::ThreadRng;

use super::StatesActions;

pub trait Eval {
    fn eval(&mut self, rng: &mut ThreadRng) -> f32;
}

pub trait IntoIterWith<'b> {
    type Item;
    type I: Iterator<Item = Self::Item>;
    fn into_iter_with(self, rng: &'b mut ThreadRng) -> Self::I;
}

pub trait SetMaxHorizon {
    fn set_max_horizon(self, max_horizon: Option<usize>) -> Self;
}

pub trait IntoEval<M: StatesActions> {
    type Evaluator<'a>: Eval + SetMaxHorizon
    where
        Self: 'a,
        M: 'a;
    fn into_eval<'a>(&'a self, s: M::State, mdp: &'a mut M) -> Self::Evaluator<'a>;
}

pub trait IntoEvalMut<M: StatesActions> {
    type Evaluator<'a>: Eval + SetMaxHorizon
    where
        Self: 'a,
        M: 'a;
    fn into_eval_mut<'a>(&'a mut self, s: M::State, mdp: &'a mut M) -> Self::Evaluator<'a>;
}
