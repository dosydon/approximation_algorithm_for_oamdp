use crate::algorithms::belief_point::BeliefPoint;
use core::fmt::Debug;
use ordered_float::*;

#[derive(Copy, Clone, Debug)]
pub struct AssocBeliefPointN<A: Debug + Copy + Clone, const N: usize> {
    pub assoc: Option<A>,
    pub v: NotNan<f32>,
    pub b: [NotNan<f32>; N],
}

impl<A: Debug + Copy + Clone, const N: usize> BeliefPoint<N> for AssocBeliefPointN<A, N> {
    fn inner(&self) -> [NotNan<f32>; N] {
        self.b
    }
}

pub type AssocBeliefPoint2<A> = AssocBeliefPointN<A, 2>;
pub type AssocBeliefPoint3<A> = AssocBeliefPointN<A, 3>;
pub type AssocBeliefPoint5<A> = AssocBeliefPointN<A, 5>;

impl<A: Debug + Copy + Clone, const N: usize> AssocBeliefPointN<A, N> {
    pub(crate) fn assoc_value(&self) -> NotNan<f32> {
        self.v
    }
}

impl<A: Debug + Copy + Clone, const N: usize> AssocBeliefPointN<A, N> {
    //     pub(crate) fn assoc_action(&self) -> Option<A> {
    //         self.assoc
    //     }
    pub(crate) fn new_not_nan(a: Option<A>, v: NotNan<f32>, b: [NotNan<f32>; N]) -> Self {
        AssocBeliefPointN {
            assoc: a,
            v: v,
            b: b,
        }
    }
}
