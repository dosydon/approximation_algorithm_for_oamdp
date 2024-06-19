use crate::baker_grid::BakerGridState;
use crate::mdp_traits::BuildFrom;

use serde::Deserialize;
use serde::Serialize;

use super::BakerGridMDP;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BakerGridPartialMDP {
    pub(in crate::baker_grid) height: usize,
    pub(in crate::baker_grid) width: usize,
    pub(in crate::baker_grid) obstacles: Vec<BakerGridState>,
    pub(in crate::baker_grid) dangerous_coordinates: Vec<(usize, usize)>,
    pub(in crate::baker_grid) prob_veering: f32,
    pub(in crate::baker_grid) initial_state: BakerGridState,
}

impl BakerGridPartialMDP {
    pub fn new(height: usize, width: usize, coordinates: Vec<(i32, i32)>) -> BakerGridPartialMDP {
        let obstacles = coordinates
            .into_iter()
            .map(|(i, j)| BakerGridState::new(i, j))
            .collect::<Vec<_>>();
        BakerGridPartialMDP {
            width: width,
            height: height,
            obstacles: obstacles,
            dangerous_coordinates: vec![],
            prob_veering: 0.0,
            initial_state: BakerGridState::new((height - 1) as i32, 0),
        }
    }

    pub fn set_prob_veering(mut self, prob_veering: f32) -> BakerGridPartialMDP {
        self.prob_veering = prob_veering;

        self
    }

    pub fn set_initial_state(mut self, initial_state: BakerGridState) -> BakerGridPartialMDP {
        self.initial_state = initial_state;

        self
    }

    pub fn add_dangerous_coordinate(mut self, coordinate: (usize, usize)) -> BakerGridPartialMDP {
        self.dangerous_coordinates.push(coordinate);

        self
    }
}

impl<'a> BuildFrom<&'a BakerGridState, BakerGridMDP> for BakerGridPartialMDP {
    fn build_from(&self, goal: &'a BakerGridState) -> BakerGridMDP {
        let mut baker_grid =
            BakerGridMDP::new(self.height, self.width, self.obstacles.clone(), *goal);
        baker_grid.prob_veering = self.prob_veering;
        baker_grid.initial_state = self.initial_state;
        for coord in self.dangerous_coordinates.iter() {
            baker_grid.is_dangerous[coord.0][coord.1] = true;
        }

        baker_grid
    }
}
