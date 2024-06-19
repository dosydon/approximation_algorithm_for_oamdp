use crate::mdp_traits::DisplayState;

use super::{mdp::SpellingMDP, state::SpellingState, SpellingEnv};

impl<const N: usize> DisplayState<SpellingState<N>> for SpellingEnv<N> {
    fn display(&self, s: &SpellingState<N>) {
        for i in 0..self.grid2d.height {
            for j in 0..self.grid2d.width {
                if self.grid2d.is_obstacled[i][j] {
                    print!("X");
                } else if s.coord.i == i as i32 && s.coord.j == j as i32 {
                    print!("O");
                } else {
                    let mut is_letter = false;
                    for k in 0..N {
                        if self.letter_locs[k].0 == i && self.letter_locs[k].1 == j {
                            is_letter = true;
                            print!("{}", s.letters[k].to_char());
                        }
                    }
                    if !is_letter {
                        print!(".");
                    }
                }
            }
            println!();
        }
    }
}

impl<const N: usize> DisplayState<SpellingState<N>> for SpellingMDP<N> {
    fn display(&self, s: &SpellingState<N>) {
        self.env.display(s)
    }
}
