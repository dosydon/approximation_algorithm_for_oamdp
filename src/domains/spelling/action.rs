use crate::into_inner::Inner;

use self::SpellingAction::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum SpellingAction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Stay,
    Toggle,
}

impl SpellingAction {
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
            Toggle => 0,
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
            Toggle => 0,
        }
    }
}

impl Inner for SpellingAction {
    type Result = SpellingAction;
    fn inner(&self) -> Self::Result {
        *self
    }
}
