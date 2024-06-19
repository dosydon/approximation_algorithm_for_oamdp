use crate::common::coordinate2::Coordinate2;
use crate::mdp_traits::BuildFrom;
use crate::rescue::{ObstacleCompatibility, RescueMDP};

#[derive(PartialEq, Debug, Clone)]
pub struct RescuePartialMDP {
    pub(in crate::rescue) height: usize,
    pub(in crate::rescue) width: usize,
    pub(in crate::rescue) obstacles: Vec<(usize, usize)>,
    pub(in crate::rescue) victim_coordinate: Coordinate2,
}

impl RescuePartialMDP {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<(usize, usize)>,
        victim_coordinate: Coordinate2,
    ) -> RescuePartialMDP {
        RescuePartialMDP {
            width: width,
            height: height,
            obstacles: obstacles,
            victim_coordinate: victim_coordinate,
        }
    }
}

impl BuildFrom<ObstacleCompatibility, RescueMDP> for RescuePartialMDP {
    fn build_from(&self, parameter: ObstacleCompatibility) -> RescueMDP {
        RescueMDP::new(
            self.height,
            self.width,
            self.obstacles.clone(),
            self.victim_coordinate,
            parameter,
        )
    }
}
