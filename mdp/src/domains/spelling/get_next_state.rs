use crate::mdp_traits::{GetNextStateFromPMass, GetNextStateMutFromImmut};

use super::mdp::SpellingMDP;

impl<const N: usize> GetNextStateFromPMass for SpellingMDP<N> {}

impl<const N: usize> GetNextStateMutFromImmut for SpellingMDP<N> {}
