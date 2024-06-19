use std::slice::Iter;

use super::StatesActions;

pub trait StateEnumerable: StatesActions {
    fn enumerate_states(&self) -> Iter<Self::State>;
    fn num_states(&self) -> usize;
    fn id_to_state(&self, id: usize) -> &Self::State;
}
