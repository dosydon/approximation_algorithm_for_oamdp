use crate::mdp_traits::ExplicitTransition;

use super::SpellingMDP;

impl<const N: usize> ExplicitTransition for SpellingMDP<N> {}
