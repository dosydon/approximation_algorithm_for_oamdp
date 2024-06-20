use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Shape {
    BlueSquare,
    BlueCircle,
    GreenSquare,
    GreenCircle,
}

impl Shape {
    pub(crate) fn random(rng: &mut ThreadRng) -> Shape {
        *[
            Shape::BlueSquare,
            Shape::BlueCircle,
            Shape::GreenSquare,
            Shape::GreenCircle,
        ]
        .choose(rng)
        .unwrap()
    }
}
