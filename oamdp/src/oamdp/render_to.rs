use mdp::{
    into_inner::Inner,
    mdp_traits::{RenderTo, StatesActions},
};

use super::oamdp::OAMDP;
use std::fmt::Debug;
use std::hash::Hash;

impl<OM, M: StatesActions + RenderTo, A: Eq + Hash + Debug + Copy, const N: usize> RenderTo
    for OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A>,
    Self::State: Inner<Result = M::State>,
{
    fn render_to(&self, s: &Self::State, path: &str) {
        self.mdp.render_to(&s.inner(), path);
    }
}
