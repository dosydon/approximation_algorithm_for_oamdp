use crate::grid_turning::direction::Direction;
use crate::into_inner::InnerMost;
use mdp_derive::InnerMost;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, InnerMost)]
pub struct GridTurningState {
    pub i: i32,
    pub j: i32,
    pub direction: Direction,
}

impl GridTurningState {
    pub fn new(i: i32, j: i32, direction: Direction) -> GridTurningState {
        GridTurningState {
            i: i,
            j: j,
            direction: direction,
        }
    }
}
