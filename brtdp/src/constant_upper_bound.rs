use mdp::mdp_traits::{IsTerminal, StatesActions};

use crate::traits::UpperBoundWithMDPMut;

pub struct ConstantUpperBound {
    value: f32,
}

impl ConstantUpperBound {
    pub fn new(value: f32) -> ConstantUpperBound {
        ConstantUpperBound { value }
    }
}

impl<M: StatesActions + IsTerminal> UpperBoundWithMDPMut<M> for ConstantUpperBound {
    fn u_with_mut(&mut self, s: &M::State, mdp: &mut M) -> f32 {
        if mdp.is_terminal(s) {
            0.0
        } else {
            self.value
        }
    }
}
