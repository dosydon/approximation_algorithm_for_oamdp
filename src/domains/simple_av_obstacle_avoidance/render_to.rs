use crate::mdp_traits::RenderTo;

use super::{ObstacleAvoidanceState, SimpleAVObstacleAvoidanceMDP};

impl RenderTo for SimpleAVObstacleAvoidanceMDP {
    fn render_to(&self, _s: &ObstacleAvoidanceState, _path: &str) {}
}
