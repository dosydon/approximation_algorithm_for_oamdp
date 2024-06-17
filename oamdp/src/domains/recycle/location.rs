use self::Location::*;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Serialize, Deserialize)]
pub enum Location {
    Compost,
    Recycle,
    Trash,
    InHand,
}

impl Location {
    pub(crate) fn random_location(rng: &mut ThreadRng) -> Location {
        *[Compost, Recycle, Trash].choose(rng).unwrap()
    }
}
