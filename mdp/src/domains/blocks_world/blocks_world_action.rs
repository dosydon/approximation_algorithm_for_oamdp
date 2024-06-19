use crate::blocks_world::block::Block;
use crate::blocks_world::Location;
use crate::into_inner::Inner;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum BlocksWorldAction {
    PickUp(Block),
    PutDown(Block, Location),
}

impl Inner for BlocksWorldAction {
    type Result = Self;
    fn inner(&self) -> Self::Result {
        *self
    }
}
