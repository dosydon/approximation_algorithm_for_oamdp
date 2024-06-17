
use super::Location;

pub type BlocksWorldState = BlocksWorldStateN<4>;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct BlocksWorldStateN<const N: usize> {
    pub locations: [Location; N],
}

impl<const N: usize> BlocksWorldStateN<N> {
    pub fn new(locations: [Location; N]) -> BlocksWorldStateN<N> {
        BlocksWorldStateN {
            locations: locations,
        }
    }
}
