use crate::simple_av::succ::*;
use crate::simple_av_lane_change::lane::Lane;
use crate::simple_av_lane_change::action::action_to_ddy;
use crate::simple_av_lane_change::action::{
    SimpleAVLaneChangeAction, Steering,
};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct VehicleConfigurationLane {
    pub y: usize,
    pub dy: i32,
    pub(in crate) lane: Lane,
}

impl VehicleConfigurationLane {
    pub fn new(y: usize, dy: i32, lane: Lane) -> VehicleConfigurationLane {
        VehicleConfigurationLane { y, dy, lane }
    }

    pub(in crate) fn next(
        self,
        a: &SimpleAVLaneChangeAction,
        maxy: usize,
        mindy: i32,
        maxdy: i32,
        lane: Lane,
    ) -> VehicleConfigurationLane {
        let ddy = action_to_ddy(a);
        VehicleConfigurationLane::new(
            usize_succ_bound(self.y, self.dy, maxy),
            i32_succ_bound(self.dy, ddy, mindy, maxdy),
            match a.steering {
                Steering::Left => match lane {
                    Lane::LeftLeft => Lane::LeftLeft,
                    Lane::LeftCenter => Lane::LeftLeft,
                    Lane::LeftRight => Lane::LeftCenter,
                    Lane::RightLeft => Lane::LeftRight,
                    Lane::RightCenter => Lane::RightLeft,
                    Lane::RightRight => Lane::RightCenter,
                },
                Steering::Center => lane,
                Steering::Right => match lane {
                    Lane::LeftLeft => Lane::LeftCenter,
                    Lane::LeftCenter => Lane::LeftRight,
                    Lane::LeftRight => Lane::RightLeft,
                    Lane::RightLeft => Lane::RightCenter,
                    Lane::RightCenter => Lane::RightRight,
                    Lane::RightRight => Lane::RightRight,
                },
            },
        )
    }
}
