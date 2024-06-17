use crate::mdp_traits::{PMass, StatesActions};

pub trait Successor: StatesActions {
    fn successor(&self, s: &Self::State, a: &Self::Action) -> Vec<Self::State>;
}

pub trait PreferredSuccessor: StatesActions {
    fn preferred_successor(&self, s: &Self::State, a: &Self::Action) -> Self::State;
}

impl<M: PMass<f32>> Successor for M {
    fn successor(&self, s: &Self::State, a: &Self::Action) -> Vec<Self::State> {
        self.p_mass(s, a)
            .into_iter()
            .map(|(s, _p)| s)
            .collect::<Vec<_>>()
    }
}
