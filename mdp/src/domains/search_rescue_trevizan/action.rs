use serde::{Deserialize, Serialize};
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub enum SRAction {
    Up,
    Down,
    Left,
    Right,
    SpeedUp,
    SpeedDown,
}

impl SRAction {
    pub(in crate) fn di(&self) -> i32 {
        match self {
            SRAction::Up => -1,
            SRAction::Down => 1,
            SRAction::Left => 0,
            SRAction::Right => 0,
            SRAction::SpeedUp => 0,
            SRAction::SpeedDown => 0,
        }
    }

    pub(in crate) fn dj(&self) -> i32 {
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