use crate::into_inner::InnerMost;
use crate::search_rescue::Coordinate;
use crate::search_rescue::VictimStatus;
use mdp_derive::InnerMost;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, EnumIter)]
pub enum ObstacleStatus {
    Removed,
    NotRemoved,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, InnerMost)]
pub struct SearchRescueState {
    pub(in crate::search_rescue) coordinate: Coordinate,
    pub(in crate::search_rescue) victim_status: VictimStatus,
    pub(in crate::search_rescue) obstacles_status: [ObstacleStatus; 4],
}

impl SearchRescueState {
    pub fn new(
        i: i64,
        j: i64,
        victim_status: VictimStatus,
        obstacles_status: [ObstacleStatus; 4],
    ) -> SearchRescueState {
        SearchRescueState {
            coordinate: Coordinate::new(i, j),
            victim_status: victim_status,
            obstacles_status: obstacles_status,
        }
    }
    pub fn from_coordinate(
        coordinate: Coordinate,
        victim_status: VictimStatus,
        obstacles_status: [ObstacleStatus; 4],
    ) -> SearchRescueState {
        SearchRescueState {
            coordinate: coordinate,
            victim_status: victim_status,
            obstacles_status: obstacles_status,
        }
    }
}
