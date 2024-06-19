use self::RescueAction::*;
use crate::into_inner::Inner;
use mdp_derive::Inner;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Inner)]
pub enum RescueAction {
    North = 0,
    South,
    East,
    West,
    RemoveObstacle,
}

pub(in crate::domains::rescue) fn get_dj(a: &RescueAction) -> i32 {
    match a {
        North => 0,
        South => 0,
        East => 1,
        West => -1,
        RemoveObstacle => 0,
    }
}

pub(in crate::domains::rescue) fn get_di(a: &RescueAction) -> i32 {
    match a {
        North => -1,
        South => 1,
        East => 0,
        West => 0,
        RemoveObstacle => 0,
    }
}
