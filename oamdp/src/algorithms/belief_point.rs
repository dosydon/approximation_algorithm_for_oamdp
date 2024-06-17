use ordered_float::*;

pub trait BeliefPoint<const N: usize> {
    fn inner(&self) -> [NotNan<f32>; N];
}
