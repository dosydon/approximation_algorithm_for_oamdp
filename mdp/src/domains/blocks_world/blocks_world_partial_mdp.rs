pub use crate::blocks_world::Location;

pub type BlocksWorldPartialMDP = BlocksWorldPartialMDPN<4>;

#[derive(PartialEq, Debug, Clone)]
pub struct BlocksWorldPartialMDPN<const N: usize> {
    pub(in crate::blocks_world) start: [Location; N],
    pub(in crate::blocks_world) epsilon: f32,
    pub(in crate::blocks_world) letters: [char; N],
}

impl<const N: usize> BlocksWorldPartialMDPN<N> {
    pub fn new(
        start: [Location; N],
        epsilon: f32,
        letters: [char; N],
    ) -> BlocksWorldPartialMDPN<N> {
        BlocksWorldPartialMDPN {
            start: start,
            epsilon: epsilon,
            letters: letters,
        }
    }
}
