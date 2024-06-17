use super::Block;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Location {
    On(Block),
    OnTable,
    OnHold,
}
