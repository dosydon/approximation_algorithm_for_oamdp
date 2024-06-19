use crate::into_inner::InnerMost;
use crate::simple_av::vehicle_configuration::VehicleConfiguration;
use crate::simple_av_lane_change::vehicle_configuration_lane::VehicleConfigurationLane;
use mdp_derive::InnerMost;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, InnerMost)]
pub struct SimpleAVLaneChangeState {
    pub ego_vehicle: VehicleConfigurationLane,
    pub other_vehicle: VehicleConfiguration,
}

impl SimpleAVLaneChangeState {
    pub fn new(
        ego_vehicle: VehicleConfigurationLane,
        other_vehicle: VehicleConfiguration,
    ) -> SimpleAVLaneChangeState {
        SimpleAVLaneChangeState {
            ego_vehicle: ego_vehicle,
            other_vehicle: other_vehicle,
        }
    }
}
