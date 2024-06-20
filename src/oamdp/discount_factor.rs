use mdp::mdp_traits::*;
use std::fmt::Debug;
use std::hash::Hash;

use super::oamdp::OAMDP;

impl<OM, M: StatesActions, A: Debug + Copy + Hash + Eq, const N: usize> DiscountFactor
    for OAMDP<OM, M, A, N>
{
    fn get_discount_factor(&self) -> f32 {
        self.gamma
    }
}
