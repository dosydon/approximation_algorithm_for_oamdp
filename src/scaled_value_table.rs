use mdp::heuristic::{HeuristicWithMDP, HeuristicWithMDPMut};
use mdp::into_inner::Inner;
use mdp::mdp_traits::StatesActions;
use mdp::value_iteration::ValueTable;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct ScaledValueTable<S: Eq + PartialEq + Debug + Clone + Hash> {
    alpha: f32,
    vt: ValueTable<S>,
}

impl<S: Eq + PartialEq + Debug + Clone + Hash + Copy> ScaledValueTable<S> {
    pub fn new(alpha: f32, vt: ValueTable<S>) -> ScaledValueTable<S> {
        ScaledValueTable {
            alpha: alpha,
            vt: vt,
        }
    }
}

impl<S: Eq + PartialEq + Debug + Clone + Hash + Copy, M: StatesActions> HeuristicWithMDP<M>
    for ScaledValueTable<S>
where
    M::State: Inner<Result = S>,
{
    fn h_with(&self, s: &<M as StatesActions>::State, _mdp: &M) -> f32 {
        self.alpha * self.vt.get_value(&s.inner())
    }
}

impl<S: Eq + PartialEq + Debug + Clone + Hash + Copy, M: StatesActions> HeuristicWithMDPMut<M>
    for ScaledValueTable<S>
where
    M::State: Inner<Result = S>,
{
}

// impl<S: Eq + PartialEq + Debug + Clone + Hash + Copy> ScaledValueTable<S> {
//     pub fn new(alpha: f32, vt: ValueTable<S>) -> ScaledValueTable<S> {
//         ScaledValueTable {
//             alpha: alpha,
//             vt: vt,
//         }
//     }
// }
