use mdp::mdp_traits::*;

use super::oamdp::OAMDP;
use std::fmt::Debug;
use std::hash::Hash;

impl<OM, M: StatesActions, A: Eq + Hash + Debug + Copy, const N: usize> Rsa for OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A> + Cost,
{
    fn rsa(&self, s: &Self::State, a: &Self::Action) -> f32 {
        (-1.0) * self.cost(s, a)
    }
}
