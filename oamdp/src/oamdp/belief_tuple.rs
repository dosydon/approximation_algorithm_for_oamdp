use ordered_float::NotNan;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct BeliefTuple<
    S: Eq + Debug + Hash + Copy,
    A: Eq + Debug + Hash + Copy,
    const N: usize,
> {
    s: S,
    a: A,
    ss: S,
    b: [NotNan<f32>; N],
}

impl<S: Eq + Debug + Hash + Copy, A: Eq + Debug + Hash + Copy, const N: usize>
    BeliefTuple<S, A, N>
{
    pub(crate) fn new(s: S, a: A, ss: S, b: [NotNan<f32>; N]) -> Self {
        Self { s, a, ss, b }
    }
}
