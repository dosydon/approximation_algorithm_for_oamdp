use crate::into_inner::Inner;
use crate::into_inner::InnerMost;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct FiniteHorizonWrapperState<S: PartialEq + Debug + Copy + Clone> {
    #[serde(bound(serialize = "S: Serialize", deserialize = "S: Deserialize<'de>"))]
    pub s: S,
    pub t: usize,
}

impl<S: PartialEq + Debug + Copy + Clone> FiniteHorizonWrapperState<S> {
    pub fn new(s: S, t: usize) -> FiniteHorizonWrapperState<S> {
        FiniteHorizonWrapperState { s: s, t: t }
    }
}

impl<S: PartialEq + Debug + Copy + Clone> Inner for FiniteHorizonWrapperState<S> {
    type Result = S;
    fn inner(&self) -> Self::Result {
        self.s
    }
}

impl<S: PartialEq + Debug + Copy + Clone> InnerMost for FiniteHorizonWrapperState<S> {
    type Result = S;
    fn inner_most(&self) -> Self::Result {
        self.s
    }
}
