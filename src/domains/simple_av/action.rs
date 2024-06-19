use self::SimpleAVAction::*;
use crate::into_inner::Inner;
use mdp_derive::Inner;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, EnumIter, Inner)]
pub enum SimpleAVAction {
    Accelerate = 0,
    Decelerate = 1,
    Keep = 2,
    Stop = 3,
    Accelerate2 = 4,
}

pub(in crate::domains::simple_av) fn action_to_ddy(a: &SimpleAVAction) -> i32 {
    match a {
        Accelerate => 1,
        Decelerate => -1,
        Keep => 0,
        Stop => -2,
        Accelerate2 => 2,
    }
}
