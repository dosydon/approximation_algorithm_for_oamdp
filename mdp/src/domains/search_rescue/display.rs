use crate::mdp_traits::DisplayState;

use super::{SearchRescueMDP, SearchRescueState};

impl DisplayState<SearchRescueState> for SearchRescueMDP {
    fn display(&self, state: &SearchRescueState) {
        for i in 0..self.height {
            for j in 0..self.width {
                if state.coordinate.i as usize == i && state.coordinate.j as usize == j {
                    print!("O");
                } else if self.victim_coordinate.i as usize == i
                    && self.victim_coordinate.j as usize == j
                {
                    print!("V");
                } else if self.is_obstacled[i][j].is_some() {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
