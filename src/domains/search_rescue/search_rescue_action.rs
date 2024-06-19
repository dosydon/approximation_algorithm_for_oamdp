use self::SearchRescueAction::*;
use crate::into_inner::Inner;
use mdp_derive::Inner;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Inner)]
pub enum SearchRescueAction {
    North = 0,
    South,
    East,
    West,
}

pub(in crate::domains::search_rescue) fn get_dj(a: &SearchRescueAction) -> i64 {
    match a {
        North => 0,
        South => 0,
        East => 1,
        West => -1,
    }
}

pub(in crate::domains::search_rescue) fn get_di(a: &SearchRescueAction) -> i64 {
    match a {
        North => -1,
        South => 1,
        East => 0,
        West => 0,
    }
}

// impl Inner for SearchRescueAction {
//     type Result = SearchRescueAction;
//     fn inner(&self) -> Self::Result {
//         *self
//     }
// }
