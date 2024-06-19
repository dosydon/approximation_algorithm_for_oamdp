use crate::{common::av1d_map::AV1dRange, mdp_traits::DisplayState};

use super::{ObstacleAvoidanceMDP, ObstacleAvoidanceState};

fn to_char(s: &ObstacleAvoidanceState) -> char {
    match s.ego_vehicle.lane {
        crate::simple_av_obstacle_avoidance::lane::Lane::Left => 'L',
        crate::simple_av_obstacle_avoidance::lane::Lane::Center => 'C',
        crate::simple_av_obstacle_avoidance::lane::Lane::Right => 'R',
    }
}

impl DisplayState<ObstacleAvoidanceState> for AV1dRange {
    fn display(&self, s: &ObstacleAvoidanceState) {
        println!("{:?}", s);
        for i in (0..self.maxy).rev() {
            if s.ego_vehicle.y == i && s.other_vehicle.y == (self.maxy - i - 1) {
                println!("O {}", to_char(s));
            } else if s.other_vehicle.y == (self.maxy - i - 1) {
                println!("O");
            } else if s.ego_vehicle.y == i {
                println!("  {}", to_char(s));
            } else {
                println!();
            }
        }
    }
}

impl DisplayState<ObstacleAvoidanceState> for ObstacleAvoidanceMDP {
    fn display(&self, s: &ObstacleAvoidanceState) {
        self.range.display(s)
    }
}
