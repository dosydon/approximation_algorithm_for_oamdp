use crate::{baker_grid::BakerGridState, mdp_traits::BuildFrom};

use super::mdp::BakerMultiGridMDP;

#[derive(PartialEq, Debug, Clone)]
pub struct BakerMultiGridPartialMDP<const N: usize> {
    pub(in crate::baker_grid) height: usize,
    pub(in crate::baker_grid) width: usize,
    pub(in crate::baker_grid) obstacles: Vec<BakerGridState>,
    pub(in crate::baker_grid) dangerous_coordinates: Vec<(usize, usize)>,
    pub(in crate::baker_grid) initial_state: [BakerGridState; N],
}

impl<const N: usize> BakerMultiGridPartialMDP<N> {
    pub fn new(
        height: usize,
        width: usize,
        coordinates: Vec<(i32, i32)>,
        initial_state: [BakerGridState; N],
    ) -> BakerMultiGridPartialMDP<N> {
        let obstacles = coordinates
            .into_iter()
            .map(|(i, j)| BakerGridState::new(i, j))
            .collect::<Vec<_>>();
        BakerMultiGridPartialMDP {
            width: width,
            height: height,
            obstacles: obstacles,
            dangerous_coordinates: vec![],
            initial_state: initial_state,
        }
    }
}

impl BuildFrom<[BakerGridState; 3], BakerMultiGridMDP<3>> for BakerMultiGridPartialMDP<3> {
    fn build_from(&self, goal: [BakerGridState; 3]) -> BakerMultiGridMDP<3> {
        let mut baker_grid = BakerMultiGridMDP::new(
            self.height,
            self.width,
            self.obstacles.clone(),
            self.initial_state,
            goal,
        );
        baker_grid.initial_state = self.initial_state;
        for coord in self.dangerous_coordinates.iter() {
            baker_grid.is_dangerous[coord.0][coord.1] = true;
        }

        baker_grid
    }
}
