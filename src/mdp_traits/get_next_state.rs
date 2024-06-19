use rand::{rngs::ThreadRng, seq::SliceRandom};

use super::{PMass, StatesActions};

pub trait GetNextState: StatesActions {
    fn get_next_state(&self, s: &Self::State, a: &Self::Action, rng: &mut ThreadRng)
        -> Self::State;
}

pub trait GetNextStateMut: StatesActions {
    fn get_next_state_mut(
        &mut self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut ThreadRng,
    ) -> Self::State;
}

pub trait GetNextStateFromPMass:
    PMass<f32, Distribution = Vec<(<Self as StatesActions>::State, f32)>>
{
    fn get_next_state_p_mass(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut ThreadRng,
    ) -> Self::State {
        self.p_mass(s, a)
            .choose_weighted(rng, |item| item.1)
            .unwrap()
            .0
    }
}

impl<M: GetNextStateFromPMass> GetNextState for M {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut ThreadRng,
    ) -> Self::State {
        self.get_next_state_p_mass(s, a, rng)
    }
}

pub trait GetNextStateMutFromImmut: StatesActions + GetNextState {
    fn get_next_state_mut_from(
        &mut self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut ThreadRng,
    ) -> Self::State {
        self.get_next_state(s, a, rng)
    }
}

impl<M: GetNextStateMutFromImmut> GetNextStateMut for M {
    fn get_next_state_mut(
        &mut self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut ThreadRng,
    ) -> Self::State {
        self.get_next_state_mut_from(s, a, rng)
    }
}
