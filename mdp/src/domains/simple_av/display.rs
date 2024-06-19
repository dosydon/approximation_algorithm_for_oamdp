use crate::{common::av1d_map::AV1dRange, mdp_traits::DisplayState};

use super::{SimpleAVVehicleInFrontMDP, SimpleAVVehicleInFrontState};

impl DisplayState<SimpleAVVehicleInFrontState> for AV1dRange {
    fn display(&self, s: &SimpleAVVehicleInFrontState) {
        for i in (0..self.maxy).rev() {
            if s.ego_vehicle.y == i {
                println!("E");
            } else if s.vehicle_in_front.y == i {
                println!("F");
            } else {
                println!();
            }
        }
    }
}

impl DisplayState<SimpleAVVehicleInFrontState> for SimpleAVVehicleInFrontMDP {
    fn display(&self, s: &SimpleAVVehicleInFrontState) {
        self.range.display(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simple_av::SimpleAVParameter::NonYield;
    use crate::simple_av::VehicleConfiguration;

    #[test]
    fn test_display() {
        let start = SimpleAVVehicleInFrontState::new(
            VehicleConfiguration::new(0, 2),
            VehicleConfiguration::new(3, 2),
        );
        let problem = SimpleAVVehicleInFrontMDP::new(0, 10, -3, 3, start, NonYield(8, 2, 3));
        problem.display(&start);
    }
}
