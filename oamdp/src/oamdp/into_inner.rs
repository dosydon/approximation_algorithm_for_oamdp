use mdp::into_inner::{IntoInner, IntoInnerMost};
use mdp::mdp_traits::StatesActions;
use std::fmt::Debug;
use std::hash::Hash;

use super::oamdp::OAMDP;

impl<'a, OM, M: StatesActions, A: Eq + Debug + Hash + Copy, const N: usize> IntoInner
    for &'a OAMDP<OM, M, A, N>
{
    type IntoInnerResult = &'a M;
    fn into_inner(self) -> &'a M {
        &self.mdp
    }
}

impl<'a, OM, M: StatesActions, A: Eq + Debug + Hash + Copy, const N: usize> IntoInnerMost
    for &'a OAMDP<OM, M, A, N>
{
    type IntoInnerMostResult = &'a M;
    fn into_inner_most(self) -> &'a M {
        &self.mdp
    }
}
