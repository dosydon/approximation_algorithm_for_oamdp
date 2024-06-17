use super::StatesActions;

pub trait PMass<F>: StatesActions {
    type Distribution: IntoIterator<Item = (Self::State, F)>
        + GetProbability<Self::State, F>
        + Clone;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Self::Distribution;
}

pub trait PMassMutFrom<F>: StatesActions + PMass<F> {
    fn p_mass_mut_from(&mut self, s: &Self::State, a: &Self::Action) -> Self::Distribution {
        self.p_mass(s, a)
    }
}

impl<M: StatesActions + PMassMutFrom<F>, F> PMassMut<F> for M {
    type Distribution = M::Distribution;
    fn p_mass_mut(&mut self, s: &M::State, a: &M::Action) -> M::Distribution {
        self.p_mass_mut_from(s, a)
    }
}

pub trait PMassMut<F>: StatesActions {
    type Distribution: IntoIterator<Item = (Self::State, F)>
        + GetProbability<Self::State, F>
        + Clone;
    fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Self::Distribution;
}

pub trait GetProbability<S, F> {
    fn get_probability(&self, s: &S) -> F;
}

impl<S: PartialEq, F: Copy> GetProbability<S, F> for Vec<(S, F)> {
    fn get_probability(&self, s: &S) -> F {
        for (s_, p) in self {
            if s == s_ {
                return *p;
            }
        }
        unreachable!("State not found in distribution")
    }
}
