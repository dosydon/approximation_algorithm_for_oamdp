use crate::mdp_traits::RenderTo;

use super::{SimpleAVVehicleInFrontMDP, SimpleAVVehicleInFrontState};

impl RenderTo for SimpleAVVehicleInFrontMDP {
    fn render_to(&self, _s: &SimpleAVVehicleInFrontState, _path: &str) {}
}
