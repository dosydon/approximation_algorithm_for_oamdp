use crate::into_inner::Inner;
use crate::simple_av::SimpleAVAction;
use crate::simple_av::SimpleAVAction::*;
use mdp_derive::Inner;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, EnumIter)]
pub enum Steering {
    Left,
    Center,
    Right,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, Inner)]
pub struct SimpleAVLaneChangeAction {
    pub(crate) acceleration: SimpleAVAction,
    pub(crate) steering: Steering,
}

impl SimpleAVLaneChangeAction {
    pub fn new(acceleration: SimpleAVAction, steering: Steering) -> SimpleAVLaneChangeAction {
        SimpleAVLaneChangeAction {
            acceleration: acceleration,
            steering: steering,
        }
    }
}

pub(crate) fn action_to_ddy(a: &SimpleAVLaneChangeAction) -> i32 {
    match a.acceleration {
        Accelerate => 1,
        Decelerate => -1,
        Keep => 0,
        Stop => -2,
        Accelerate2 => 2,
    }
}
