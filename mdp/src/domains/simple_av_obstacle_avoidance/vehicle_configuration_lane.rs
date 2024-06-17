use crate::simple_av::succ::*;
use crate::simple_av_obstacle_avoidance::action::action_to_ddy;
use crate::simple_av_obstacle_avoidance::action::ObstacleAvoidanceAction;
use crate::simple_av_obstacle_avoidance::lane::Lane;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct VehicleConfigurationLane {
    pub y: usize,
    pub dy: i32,
    pub(crate) lane: Lane,
}

impl VehicleConfigurationLane {
    pub fn new(y: usize, dy: i32, lane: Lane) -> VehicleConfigurationLane {
        VehicleConfigurationLane { y, dy, lane }
    }

    pub(crate) fn next(
        self,
        a: &ObstacleAvoidanceAction,
        maxy: usize,
        mindy: i32,
        maxdy: i32,
        lane: Lane,
    ) -> VehicleConfigurationLane {
        let ddy = action_to_ddy(a);
        VehicleConfigurationLane::new(
            usize_succ_bound(self.y, self.dy, maxy),
            i32_succ_bound(self.dy, ddy, mindy, maxdy),
            match a.next_lane {
                Lane::Left => match lane {
                    Lane::Right => Lane::Center,
                    _ => Lane::Left,
                },
                Lane::Center => Lane::Center,
                Lane::Right => match lane {
                    Lane::Left => Lane::Center,
                    _ => Lane::Right,
                },
            },
        )
    }
}
