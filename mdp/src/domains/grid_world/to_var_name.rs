use crate::mdp_traits::ToVarName;
use super::GridWorldState;

impl ToVarName for GridWorldState {
    fn to_var_name(&self) -> String {
        format!("x{}y{}", self.x(), self.y())
    }
}