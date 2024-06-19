use self::BakerGridAction::*;
use crate::into_inner::Inner;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, EnumIter)]
pub enum BakerGridAction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Stay,
}

impl Inner for BakerGridAction {
    type Result = BakerGridAction;

    fn inner(&self) -> Self::Result {
        *self
    }
}

impl<const N: usize> Inner for [BakerGridAction; N] {
    type Result = [BakerGridAction; N];
    fn inner(&self) -> Self::Result {
        *self
    }
}

impl BakerGridAction {
    pub fn get_dj(&self) -> i32 {
        match self {
            North => 0,
            South => 0,
            East => 1,
            West => -1,
            NorthEast => 1,
            NorthWest => -1,
            SouthEast => 1,
            SouthWest => -1,
            Stay => 0,
        }
    }

    pub fn get_di(&self) -> i32 {
        match self {
            North => -1,
            South => 1,
            East => 0,
            West => 0,
            NorthEast => -1,
            NorthWest => -1,
            SouthEast => 1,
            SouthWest => 1,
            Stay => 0,
        }
    }
}

impl fmt::Display for BakerGridAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            North => "↑",
            South => "↓",
            East => "→",
            West => "←",
            NorthEast => "↗",
            NorthWest => "↖",
            SouthEast => "↘",
            SouthWest => "↙",
            Stay => "s",
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serde_grid_action() {
        let action = BakerGridAction::South;
        let serialized = serde_json::to_string(&action).unwrap();
        let deserialized: BakerGridAction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(action, deserialized);
    }
}
