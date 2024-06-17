#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Block {
    pub(in crate) id: usize
}

impl Block {
    pub fn new(id: usize) -> Block {
        Block {
            id: id
        }
    }
}
