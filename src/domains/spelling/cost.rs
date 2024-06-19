use crate::{
    common::coordinate2::Coordinate2,
    mdp_traits::{Cost, DCost, DiscountFactor, IsTerminal, PMass, Rsa},
};

use super::mdp::SpellingMDP;

impl<const N: usize> Rsa for SpellingMDP<N> {
    fn rsa(&self, s: &Self::State, a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            (-1.0) * self.cost(s, a)
        }
    }
}

impl<const N: usize> DiscountFactor for SpellingMDP<N> {
    fn get_discount_factor(&self) -> f32 {
        0.9
    }
}

fn distance(s: &Coordinate2, ss: &Coordinate2) -> f32 {
    (((s.i - ss.i) * (s.i - ss.i) + (s.j - ss.j) * (s.j - ss.j)) as f32).sqrt()
}

impl<const N: usize> DCost for SpellingMDP<N> {
    fn d_cost(&self, st: &Self::State, _a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else if self.is_terminal(stt) {
            //             println!("{:?} {:?}", st, a);
            0.0
        } else {
            distance(&st.coord, &stt.coord).max(1.0)
        }
    }
}

impl<const N: usize> Cost for SpellingMDP<N> {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            self.p_mass(s, a)
                .into_iter()
                .map(|(stt, prob)| prob * self.d_cost(s, a, &stt))
                .sum()
        }
    }
}
