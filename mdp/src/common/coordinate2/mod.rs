use crate::{into_inner::InnerMost, mdp_traits::ToVarName};
use mdp_derive::InnerMost;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, InnerMost)]
pub struct Coordinate2 {
    pub i: i32,
    pub j: i32,
}

impl Coordinate2 {
    pub fn new(i: i32, j: i32) -> Self {
        Coordinate2 { i: i, j: j }
    }
}

impl ToVarName for Coordinate2 {
    fn to_var_name(&self) -> String {
        format!("c{}_{}", self.i, self.j)
    }
}
