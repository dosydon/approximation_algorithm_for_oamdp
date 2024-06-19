use crate::common::coordinate2::Coordinate2;
use crate::grid_turning::{GridTurningMDP, GridTurningState};
use crate::mdp_traits::*;

#[derive(PartialEq, Debug, Clone)]
pub struct GridTurningPartialMDP {
    pub(in crate::grid_turning) height: usize,
    pub(in crate::grid_turning) width: usize,
    pub(in crate::grid_turning) initial_state: GridTurningState,
    pub(in crate::grid_turning) obstacles: Vec<Coordinate2>,
    pub(in crate::grid_turning) prob_veering: f32,
}

impl GridTurningPartialMDP {
    pub fn new(
        height: usize,
        width: usize,
        initial_state: GridTurningState,
        coordinates: Vec<(i32, i32)>,
        prob_veering: f32,
    ) -> GridTurningPartialMDP {
        let obstacles = coordinates
            .into_iter()
            .map(|(i, j)| Coordinate2::new(i, j))
            .collect::<Vec<_>>();
        GridTurningPartialMDP {
            width: width,
            height: height,
            initial_state: initial_state,
            obstacles: obstacles,
            prob_veering: prob_veering,
        }
    }
}

impl BuildFrom<Coordinate2, GridTurningMDP> for GridTurningPartialMDP {
    fn build_from(&self, goal: Coordinate2) -> GridTurningMDP {
        let mdp = GridTurningMDP::new(
            self.height,
            self.width,
            self.obstacles.clone(),
            self.initial_state,
            goal,
            self.prob_veering,
        );

        mdp
    }
}
