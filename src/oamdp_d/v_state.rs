use mdp::into_inner::Inner;

use mdp::mdp_traits::ToVarName;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct VState<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> {
    #[serde(bound(serialize = "S: Serialize", deserialize = "S: Deserialize<'de>"))]
    pub(crate) s: S,
    #[serde(with = "serde_arrays")]
    pub(crate) v: [usize; N],
    pub is_dummy_initial_state: bool,
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> VState<S, N> {
    pub fn new(s: S, v: [usize; N]) -> Self {
        VState {
            s: s,
            v: v,
            is_dummy_initial_state: false,
        }
    }
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> Inner for VState<S, N> {
    type Result = S;
    fn inner(&self) -> Self::Result {
        self.s
    }
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash + ToVarName, const N: usize> ToVarName
    for VState<S, N>
{
    fn to_var_name(&self) -> String {
        match self.is_dummy_initial_state {
            true => format!("dummy_initial_state"),
            false => format!(
                "{}_{}",
                self.s.to_var_name(),
                self.v
                    .iter()
                    .fold(String::new(), |acc, x| format!("{}_{}", acc, x))
            ),
        }
    }
}
