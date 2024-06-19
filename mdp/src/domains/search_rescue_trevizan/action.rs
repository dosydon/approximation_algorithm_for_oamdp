use serde::{Deserialize, Serialize};

use crate::mdp_traits::ToVarName;
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub enum SRAction {
    Up,
    Down,
    Left,
    Right,
    SpeedUp,
    SpeedDown,
}

impl ToVarName for SRAction {
    fn to_var_name(&self) -> String {
        match self {
            SRAction::Up => "U".to_string(),
            SRAction::Down => "D".to_string(),
            SRAction::Left => "L".to_string(),
            SRAction::Right => "R".to_string(),
            SRAction::SpeedUp => "SU".to_string(),
            SRAction::SpeedDown => "SD".to_string(),
        }
    }
}

impl SRAction {
    pub(crate) fn di(&self) -> i32 {
        match self {
            SRAction::Up => -1,
            SRAction::Down => 1,
            SRAction::Left => 0,
            SRAction::Right => 0,
            SRAction::SpeedUp => 0,
            SRAction::SpeedDown => 0,
        }
    }

    pub(crate) fn dj(&self) -> i32 {
        match self {
            SRAction::Up => 0,
            SRAction::Down => 0,
            SRAction::Left => -1,
            SRAction::Right => 1,
            SRAction::SpeedUp => 0,
            SRAction::SpeedDown => 0,
        }
    }
}
