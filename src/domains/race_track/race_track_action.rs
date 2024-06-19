use crate::mdp_traits::ToVarName;

use self::RaceTrackAction::*;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum RaceTrackAction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Keep,
}

impl ToVarName for RaceTrackAction {
    fn to_var_name(&self) -> String {
        match self {
            North => "N".to_string(),
            South => "S".to_string(),
            East => "E".to_string(),
            West => "W".to_string(),
            NorthEast => "NE".to_string(),
            NorthWest => "NW".to_string(),
            SouthEast => "SE".to_string(),
            SouthWest => "SW".to_string(),
            Keep => "K".to_string(),
        }
    }
}

pub fn get_ddx(a: RaceTrackAction) -> i32 {
    match a {
        North => 0,
        South => 0,
        East => 1,
        West => -1,
        NorthEast => 1,
        NorthWest => -1,
        SouthEast => 1,
        SouthWest => -1,
        Keep => 0,
    }
}

pub fn get_ddy(a: RaceTrackAction) -> i32 {
    match a {
        North => 1,
        South => -1,
        East => 0,
        West => 0,
        NorthEast => 1,
        NorthWest => 1,
        SouthEast => -1,
        SouthWest => -1,
        Keep => 0,
    }
}
