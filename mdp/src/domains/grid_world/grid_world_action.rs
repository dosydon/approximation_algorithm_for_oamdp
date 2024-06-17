#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum GridWorldAction {
    AttemptUp = 0,
    AttemptRight,
    AttemptDown,
    AttemptLeft,
}

impl AsRef<GridWorldAction> for GridWorldAction {
    fn as_ref(&self) -> &GridWorldAction {
        self
    }
}