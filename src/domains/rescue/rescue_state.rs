use crate::common::coordinate2::Coordinate2;
use crate::into_inner::InnerMost;
use mdp_derive::InnerMost;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, EnumIter)]
pub enum ObstacleStatus {
    Removed,
    NotRemoved,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, InnerMost)]
pub struct RescueState {
    pub(in crate::rescue) coordinate: Coordinate2,
    pub(in crate::rescue) obstacles_status: [ObstacleStatus; 3],
}

impl RescueState {
    pub fn new(coordinate: Coordinate2, obstacles_status: [ObstacleStatus; 3]) -> RescueState {
        RescueState {
            coordinate: coordinate,
            obstacles_status: obstacles_status,
        }
    }
}
