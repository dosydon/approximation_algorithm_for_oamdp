use crate::search_rescue::Coordinate;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, EnumIter)]
pub enum ObstacleCompatibility {
    Low,
    High,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SearchRescueParameter {
    pub(in crate::search_rescue) base_coordinate: Coordinate,
    pub(in crate::search_rescue) obstacle_compatibility: ObstacleCompatibility,
}

impl SearchRescueParameter {
    pub fn new(
        base_coordinate: Coordinate,
        obstacle_compatibility: ObstacleCompatibility,
    ) -> SearchRescueParameter {
        SearchRescueParameter {
            base_coordinate: base_coordinate,
            obstacle_compatibility: obstacle_compatibility,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SearchRescuePartialMDP {
    pub(in crate::search_rescue) height: usize,
    pub(in crate::search_rescue) width: usize,
    pub(in crate::search_rescue) obstacles: Vec<(usize, usize)>,
    pub(in crate::search_rescue) victim_coordinate: Coordinate,
}

impl SearchRescuePartialMDP {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<(usize, usize)>,
        victim_coordinate: Coordinate,
    ) -> SearchRescuePartialMDP {
        SearchRescuePartialMDP {
            width: width,
            height: height,
            obstacles: obstacles,
            victim_coordinate: victim_coordinate,
        }
    }
}
