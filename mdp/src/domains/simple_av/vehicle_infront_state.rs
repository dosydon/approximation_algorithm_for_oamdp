use crate::into_inner::InnerMost;
use crate::simple_av::vehicle_configuration::VehicleConfiguration;
use mdp_derive::InnerMost;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, InnerMost)]
pub struct SimpleAVVehicleInFrontState {
    pub(in crate::simple_av) ego_vehicle: VehicleConfiguration,
    pub(in crate::simple_av) vehicle_in_front: VehicleConfiguration,
}

impl SimpleAVVehicleInFrontState {
    pub fn new(
        ego_vehicle: VehicleConfiguration,
        vehicle_in_front: VehicleConfiguration,
    ) -> SimpleAVVehicleInFrontState {
        SimpleAVVehicleInFrontState {
            ego_vehicle,
            vehicle_in_front,
        }
    }
}
