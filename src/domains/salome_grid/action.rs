use crate::into_inner::Inner;

use self::SalomeGridAction::*;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub enum SalomeGridAction {
    Left,
    Up,
    Right,
    Down,
}

impl Inner for SalomeGridAction {
    type Result = SalomeGridAction;
    fn inner(&self) -> Self::Result {
        *self
    }
}

pub fn get_dj(a: SalomeGridAction) -> i32 {
    match a {
        Left => -1,
        Up => 0,
        Right => 1,
        Down => 0,
    }
}

pub fn get_di(a: SalomeGridAction) -> i32 {
    match a {
        Left => 0,
        Up => -1,
        Right => 0,
        Down => 1,
    }
}
