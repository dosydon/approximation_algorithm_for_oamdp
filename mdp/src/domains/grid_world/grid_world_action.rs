use crate::mdp_traits::ToVarName;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum GridWorldAction {
    AttemptUp = 0,
    AttemptRight,
    AttemptDown,
    AttemptLeft,
}

impl ToVarName for GridWorldAction {
    fn to_var_name(&self) -> String {
        match self {
            GridWorldAction::AttemptUp => "U".to_string(),
            GridWorldAction::AttemptRight => "R".to_string(),
            GridWorldAction::AttemptDown => "D".to_string(),
            GridWorldAction::AttemptLeft => "L".to_string(),
        }
    }
}

impl AsRef<GridWorldAction> for GridWorldAction {
    fn as_ref(&self) -> &GridWorldAction {
        self
    }
}
