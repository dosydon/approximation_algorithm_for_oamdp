use serde::{Deserialize, Serialize};

use super::{location::Location, RecycleMDP};

#[derive(Serialize, Deserialize)]
pub struct RecycleMDPBuilder<const K: usize> {
    #[serde(with = "serde_arrays")]
    pub(crate) initial_locs: [Location; K],
    #[serde(with = "serde_arrays")]
    pub(crate) kinds: [usize; K],
    pub(crate) actual_success_prob: f32,
}

impl<const K: usize> RecycleMDPBuilder<K> {
    pub(crate) fn build(&self, target: [Location; 3]) -> RecycleMDP<K> {
        RecycleMDP::new(
            target,
            self.initial_locs,
            self.kinds,
            self.actual_success_prob,
        )
    }
}
