use self::Direction::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub enum Direction {
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    North,
    NorthEast,
}

pub(in crate::domains::grid_turning) fn turn_right(direction: Direction) -> Direction {
    match direction {
        North => NorthEast,
        South => SouthWest,
        East => SouthEast,
        West => NorthWest,
        NorthEast => East,
        NorthWest => North,
        SouthEast => South,
        SouthWest => West,
    }
}

pub(in crate::domains::grid_turning) fn veer_right(direction: Direction) -> Direction {
    turn_right(turn_right(direction))
}

pub(in crate::domains::grid_turning) fn turn_left(direction: Direction) -> Direction {
    match direction {
        North => NorthWest,
        South => SouthEast,
        East => NorthEast,
        West => SouthWest,
        NorthEast => North,
        NorthWest => West,
        SouthEast => East,
        SouthWest => South,
    }
}

pub(in crate::domains::grid_turning) fn veer_left(direction: Direction) -> Direction {
    turn_left(turn_left(direction))
}

pub(in crate::domains::grid_turning) fn get_dj(d: Direction) -> i32 {
    match d {
        North => 0,
        South => 0,
        East => 1,
        West => -1,
        NorthEast => 1,
        NorthWest => -1,
        SouthEast => 1,
        SouthWest => -1,
    }
}

pub(in crate::domains::grid_turning) fn get_di(d: Direction) -> i32 {
    match d {
        North => -1,
        South => 1,
        East => 0,
        West => 0,
        NorthEast => -1,
        NorthWest => -1,
        SouthEast => 1,
        SouthWest => 1,
    }
}
