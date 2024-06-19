use crate::mdp_traits::BuildFrom;
use crate::simple_av::vehicle_configuration::VehicleConfiguration;
use crate::simple_av_lane_change::lane::Lane;
use crate::simple_av_lane_change::mdp::SimpleAVLaneChangeMDP;
use crate::simple_av_lane_change::parameter::SimpleAVLaneChangeParameter;
use crate::simple_av_lane_change::state::SimpleAVLaneChangeState;
use crate::simple_av_lane_change::vehicle_configuration_lane::VehicleConfigurationLane;

pub struct SimpleAVLaneChangePartialMDP {
    pub(in crate::domains::simple_av_lane_change) maxy: usize,
    pub(in crate::domains::simple_av_lane_change) mindy: i32,
    pub(in crate::domains::simple_av_lane_change) maxdy: i32,
    pub(in crate::domains::simple_av_lane_change) start: SimpleAVLaneChangeState,
}

impl SimpleAVLaneChangePartialMDP {
    pub fn new(maxy: usize, maxdy: i32) -> SimpleAVLaneChangePartialMDP {
        let start = SimpleAVLaneChangeState::new(
            VehicleConfigurationLane::new(0, 2, Lane::RightCenter),
            VehicleConfiguration::new(0, 2),
        );

        SimpleAVLaneChangePartialMDP {
            maxy: maxy,
            mindy: 0,
            maxdy: maxdy,
            start: start,
        }
    }
}

impl BuildFrom<SimpleAVLaneChangeParameter, SimpleAVLaneChangeMDP>
    for SimpleAVLaneChangePartialMDP
{
    fn build_from(&self, parameter: SimpleAVLaneChangeParameter) -> SimpleAVLaneChangeMDP {
        SimpleAVLaneChangeMDP::new(self.maxy, self.mindy, self.maxdy, self.start, parameter)
    }
}
