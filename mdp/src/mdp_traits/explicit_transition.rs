use super::{PMass, StatesActions};

pub trait ExplicitTransition: StatesActions + PMass<f32> {
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        self.p_mass(st, a)
            .into_iter()
            .filter(|&(next_state, _p)| next_state == *stt)
            .map(|(_next_state, p)| p)
            .sum()
    }
}

pub(crate) trait ExplicitTransitionMutFrom: StatesActions + ExplicitTransition {
    fn p_mut_from(&mut self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        self.p(st, a, stt)
    }
}

impl<T: ExplicitTransitionMutFrom> ExplicitTransitionMut for T {
    fn p_mut(&mut self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        self.p_mut_from(st, a, stt)
    }
}

pub trait ExplicitTransitionMut: StatesActions {
    fn p_mut(&mut self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32;
}
