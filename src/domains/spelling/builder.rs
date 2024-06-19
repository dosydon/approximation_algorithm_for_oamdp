use crate::state_enumerable_wrapper::StateEnumerableWrapper;
use crate::{common::coordinate2::Coordinate2, mdp_traits::BuildFrom};

use super::letter::Letter;
use super::{mdp::SpellingMDP, state::SpellingState};

pub struct SpellingMDPBuilder<const NL: usize> {
    pub height: usize,
    pub width: usize,
    pub(in crate::domains::spelling) obstacles: Vec<Coordinate2>,
    pub(crate) letter_locs: [(usize, usize); NL],
    pub prob_veering: f32,
    initial_state: SpellingState<NL>,
}

impl<const N: usize> SpellingMDPBuilder<N> {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<Coordinate2>,
        letter_locs: [(usize, usize); N],
        initial_state: SpellingState<N>,
    ) -> SpellingMDPBuilder<N> {
        SpellingMDPBuilder {
            width: width,
            height: height,
            letter_locs: letter_locs,
            obstacles: obstacles,
            prob_veering: 0.0,
            initial_state: initial_state,
        }
    }
}

impl<'a, const N: usize> BuildFrom<&'a [Letter; N], SpellingMDP<N>> for SpellingMDPBuilder<N> {
    fn build_from(&self, from: &'a [Letter; N]) -> SpellingMDP<N> {
        SpellingMDP::new(
            self.height,
            self.width,
            self.obstacles.clone(),
            *from,
            self.letter_locs,
            self.initial_state,
        )
    }
}

impl<'a, const N: usize> BuildFrom<&'a [Letter; N], StateEnumerableWrapper<SpellingMDP<N>>>
    for SpellingMDPBuilder<N>
{
    fn build_from(&self, from: &'a [Letter; N]) -> StateEnumerableWrapper<SpellingMDP<N>> {
        StateEnumerableWrapper::new(SpellingMDP::new(
            self.height,
            self.width,
            self.obstacles.clone(),
            *from,
            self.letter_locs,
            self.initial_state,
        ))
    }
}
