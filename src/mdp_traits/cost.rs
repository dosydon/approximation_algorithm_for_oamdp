use super::{PMass, StatesActions};

pub trait CostFromDCost: DCost + PMass<f32> {
    fn cost_from_d_cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        self.p_mass(s, a)
            .into_iter()
            .map(|(ss, p)| p * self.d_cost(s, a, &ss))
            .sum()
    }
}

impl<M: CostFromDCost> Cost for M {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        self.cost_from_d_cost(s, a)
    }
}

pub trait Cost: StatesActions {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32;
}

pub trait DCost: StatesActions {
    fn d_cost(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32;
}
