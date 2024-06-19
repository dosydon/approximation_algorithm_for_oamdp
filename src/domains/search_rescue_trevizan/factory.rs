use super::instances::*;
use super::{
    map_configuration::MapConfiguration, mdp::SRMDP, mdp_d::SRMDPD, speed::Speed, state::SRState,
};
use crate::mdp_traits::BuildFrom;
use rand::prelude::*;
pub struct SRFactory {
    n: usize,
    d: usize,
    density_r: f32,
}

impl SRFactory {
    pub fn from_name(name: &str) -> SRMDPD {
        match name {
            "tiny" => SRMDPD::SRMDP2(tiny()),
            "small" => SRMDPD::SRMDP3(small()),
            "medium" => SRMDPD::SRMDP4(medium()),
            "large" => SRMDPD::SRMDP5(large()),
            _ => panic!("Unknown domain name: {}", name),
        }
    }
    pub fn new(n: usize, d: usize, density_r: f32) -> Self {
        SRFactory { n, d, density_r }
    }
}

impl BuildFrom<&mut ThreadRng, SRMDPD> for SRFactory {
    fn build_from(&self, rng: &mut ThreadRng) -> SRMDPD {
        match self.n {
            4 => {
                let configuration =
                    MapConfiguration::<4>::random_instance(self.d, self.density_r, rng);
                let initial_state = SRState::new(
                    configuration.cells,
                    configuration.agent_pos,
                    false,
                    Speed::Medium,
                );
                let mdp = SRMDP::new(initial_state, MapConfiguration::<4>::random_position(rng));
                SRMDPD::SRMDP4(mdp)
            }
            5 => {
                let configuration =
                    MapConfiguration::<5>::random_instance(self.d, self.density_r, rng);
                let initial_state = SRState::new(
                    configuration.cells,
                    configuration.agent_pos,
                    false,
                    Speed::Medium,
                );
                let mdp = SRMDP::new(initial_state, MapConfiguration::<5>::random_position(rng));
                SRMDPD::SRMDP5(mdp)
            }
            _ => panic!("Unknown domain size: {}", self.n),
        }
    }
}
