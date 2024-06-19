use crate::into_inner::Inner;
use mdp_derive::Inner;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, Inner)]
pub enum GridTurningAction {
    TurnLeft,
    TurnRight,
    GoStraight,
}
