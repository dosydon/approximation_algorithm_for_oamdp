use serde::{Deserialize, Serialize};

use crate::{
    baker_grid::{BakerGridPartialMDP, BakerGridState},
    common::coordinate2::Coordinate2,
    mdp_traits::BuildFrom,
};

use super::BakerGridResetMDP;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BakerGridResetBuilder {
    builder: BakerGridPartialMDP,
    reset_prob: f32,
    reset_states: Vec<Coordinate2>,
}

impl BakerGridResetBuilder {
    pub fn new(
        builder: BakerGridPartialMDP,
        reset_prob: f32,
        reset_states: Vec<Coordinate2>,
    ) -> Self {
        BakerGridResetBuilder {
            builder: builder,
            reset_prob: reset_prob,
            reset_states: reset_states,
        }
    }
}

impl<'a> BuildFrom<&'a BakerGridState, BakerGridResetMDP> for BakerGridResetBuilder {
    fn build_from(&self, s: &'a Coordinate2) -> BakerGridResetMDP {
        let mdp = self.builder.build_from(&s);
        BakerGridResetMDP::new(mdp, self.reset_prob, self.reset_states.clone())
    }
}
