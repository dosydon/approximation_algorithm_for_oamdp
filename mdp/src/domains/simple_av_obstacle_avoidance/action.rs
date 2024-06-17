use crate::into_inner::Inner;
use crate::simple_av::SimpleAVAction;
use crate::simple_av::SimpleAVAction::*;
use crate::simple_av_obstacle_avoidance::lane::Lane;
use mdp_derive::Inner;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, Inner)]
pub struct ObstacleAvoidanceAction {
    pub(crate) acceleration: SimpleAVAction,
    pub(crate) next_lane: Lane,
}

impl ObstacleAvoidanceAction {
    pub fn new(acceleration: SimpleAVAction, next_lane: Lane) -> ObstacleAvoidanceAction {
        ObstacleAvoidanceAction {
            acceleration: acceleration,
            next_lane: next_lane,
        }
    }
}

pub(in crate::domains::simple_av_obstacle_avoidance) fn action_to_ddy(
    a: &ObstacleAvoidanceAction,
) -> i32 {
    match a.acceleration {
        Accelerate => 1,
        Decelerate => -1,
        Keep => 0,
        Stop => -2,
        Accelerate2 => 2,
    }
}

// impl IntoInner for (ObstacleAvoidanceAction, HeadLight) {
//     type Result = ObstacleAvoidanceAction;
//     fn into_inner(&self) -> Self::Result {
//         self.0
//     }
// }

// impl EnumerateWithMDP<SimpleAVObstacleAvoidanceMDP> for (ObstacleAvoidanceAction, HeadLight) {
//     fn enumerate_with_mdp(mdp: &SimpleAVObstacleAvoidanceMDP) -> Vec<(ObstacleAvoidanceAction, HeadLight)> {
//         iproduct!(mdp.enumerate_actions(), HeadLight::iter())
//             .map(|(a, h)| (*a, h))
//             .collect()
//     }
// }
