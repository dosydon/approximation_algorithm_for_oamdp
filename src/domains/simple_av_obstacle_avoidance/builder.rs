use crate::mdp_traits::BuildFrom;
use crate::simple_av::vehicle_configuration::VehicleConfiguration;
use crate::simple_av_obstacle_avoidance::lane::Lane;
use crate::simple_av_obstacle_avoidance::mdp::ObstacleAvoidanceMDP;
use crate::simple_av_obstacle_avoidance::parameter::ObstacleAvoidanceParameter;
use crate::simple_av_obstacle_avoidance::state::ObstacleAvoidanceState;
use crate::simple_av_obstacle_avoidance::vehicle_configuration_lane::VehicleConfigurationLane;

pub struct ObstacleAvoidanceBuilder {
    pub(in crate::domains::simple_av_obstacle_avoidance) maxy: usize,
    pub(in crate::domains::simple_av_obstacle_avoidance) mindy: i32,
    pub(in crate::domains::simple_av_obstacle_avoidance) maxdy: i32,
    pub(in crate::domains::simple_av_obstacle_avoidance) start: ObstacleAvoidanceState,
    collision_zone_lb: usize,
    collision_zone_ub: usize,
}

impl ObstacleAvoidanceBuilder {
    pub fn new(maxy: usize, maxdy: i32) -> ObstacleAvoidanceBuilder {
        let start = ObstacleAvoidanceState::new(
            VehicleConfigurationLane::new(0, 3, Lane::Center),
            VehicleConfiguration::new(0, 3),
        );

        ObstacleAvoidanceBuilder {
            maxy: maxy,
            mindy: 0,
            maxdy: maxdy,
            start: start,
            collision_zone_lb: 11,
            collision_zone_ub: 19,
        }
    }

    pub fn set_collision_zone(mut self, lb: usize, ub: usize) -> Self {
        self.collision_zone_lb = lb;
        self.collision_zone_ub = ub;

        self
    }

    pub fn set_start_state(mut self, start: ObstacleAvoidanceState) -> Self {
        self.start = start;

        self
    }
}

impl<'a> BuildFrom<&'a ObstacleAvoidanceParameter, ObstacleAvoidanceMDP>
    for ObstacleAvoidanceBuilder
{
    fn build_from(&self, parameter: &'a ObstacleAvoidanceParameter) -> ObstacleAvoidanceMDP {
        let mut mdp =
            ObstacleAvoidanceMDP::new(self.maxy, self.mindy, self.maxdy, self.start, *parameter);
        mdp.collision_zone_lb = self.collision_zone_lb;
        mdp.collision_zone_ub = self.collision_zone_ub;

        mdp
    }
}
