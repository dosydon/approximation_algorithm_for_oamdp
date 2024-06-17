use crate::traits::BeliefOverGoal;
use core::fmt::Debug;
use core::hash::Hash;

use mdp::into_inner::Inner;

use ordered_float::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct BeliefState<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> {
    #[serde(bound(serialize = "S: Serialize", deserialize = "S: Deserialize<'de>"))]
    pub(crate) s: S,
    #[serde(with = "serde_arrays")]
    pub(crate) belief_over_goal: [NotNan<f32>; N],
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> BeliefState<S, N> {
    pub fn new(s: S, p: [NotNan<f32>; N]) -> BeliefState<S, N> {
        BeliefState {
            s: s,
            belief_over_goal: p,
        }
    }
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> BeliefOverGoal<N>
    for BeliefState<S, N>
{
    fn get_belief_over_goal(&self) -> [NotNan<f32>; N] {
        self.belief_over_goal
    }
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> Inner for BeliefState<S, N> {
    type Result = S;
    fn inner(&self) -> Self::Result {
        self.s
    }
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> BeliefState<S, N> {
    pub fn sum_belief_over_goal(&self) -> NotNan<f32> {
        self.belief_over_goal.iter().sum()
    }
}
