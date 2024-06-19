use rand::rngs::ThreadRng;

use super::StatesActions;

pub trait InitialState: StatesActions {
    fn initial_state(&self) -> Self::State;
}

pub trait SampleInitialState: StatesActions {
    fn sample_initial_state(&self, rng: &mut ThreadRng) -> Self::State;
}

pub trait ProbInitialState: StatesActions {
    fn prob_initial_state(&self, s: &Self::State) -> f32;
}

impl<M: InitialState> ProbInitialState for M {
    fn prob_initial_state(&self, s: &Self::State) -> f32 {
        if *s == self.initial_state() {
            1.0
        } else {
            0.0
        }
    }
}
