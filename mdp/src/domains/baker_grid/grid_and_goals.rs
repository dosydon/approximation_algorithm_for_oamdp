use crate::{common::grid2d::Grid2D, mdp_traits::DisplayState};

use super::BakerGridState;

#[derive(Debug, Clone)]
pub struct GridAndGoals {
    grid: Grid2D,
    pub goals: Vec<(usize, usize)>,
    labels: Vec<String>,
}

impl GridAndGoals {
    pub fn new(grid: Grid2D, goals: Vec<(usize, usize)>, labels: Vec<String>) -> Self {
        Self {
            grid: grid,
            goals: goals,
            labels: labels,
        }
    }

    fn print_if_goal(&self, i: usize, j: usize) {
        for (k, (gi, gj)) in self.goals.iter().enumerate() {
            if i == *gi && j == *gj {
                print!("{}", self.labels[k]);
                return;
            }
        }
        print!(".");
    }
}

impl DisplayState<BakerGridState> for GridAndGoals {
    fn display(&self, s: &BakerGridState) {
        for i in 0..self.grid.height {
            for j in 0..self.grid.width {
                if self.grid.is_obstacled[i][j] {
                    print!("X");
                } else if s.i == i as i32 && s.j == j as i32 {
                    print!("O");
                } else {
                    self.print_if_goal(i, j);
                }
            }
            println!();
        }
    }
}

impl<const N: usize> DisplayState<[BakerGridState; N]> for GridAndGoals {
    fn display(&self, s: &[BakerGridState; N]) {
        for i in 0..self.grid.height {
            for j in 0..self.grid.width {
                if self.grid.is_obstacled[i][j] {
                    print!("X");
                } else {
                    let mut is_player = false;
                    for k in 0..N {
                        if s[k].i == i as i32 && s[k].j == j as i32 {
                            if !is_player {
                                is_player = true;
                                print!("{}", k);
                            }
                        }
                    }
                    if !is_player {
                        self.print_if_goal(i, j);
                    }
                }
            }
            println!();
        }
    }
}
