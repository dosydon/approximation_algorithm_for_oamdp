use crate::common::grid2d::Grid2D;
use crate::mdp_traits::DisplayState;

use super::BakerGridMDP;
use super::BakerGridState;

impl DisplayState<BakerGridState> for Grid2D {
    fn display(&self, s: &BakerGridState) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.is_obstacled[i][j] {
                    print!("X");
                } else if s.i == i as i32 && s.j == j as i32 {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl DisplayState<BakerGridState> for BakerGridMDP {
    fn display(&self, s: &BakerGridState) {
        self.grid2d.display(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baker_grid_display() {
        let mdp = BakerGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            BakerGridState::new(4, 4),
        );

        let s = BakerGridState::new(2, 2);
        mdp.display(&s);
    }
}
