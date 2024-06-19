
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct GridWorldState {
    pub(in crate::grid_world) x: i64,
    pub(in crate::grid_world) y: i64,
}

impl AsRef<GridWorldState> for GridWorldState {
    fn as_ref(&self) -> &GridWorldState {
        self
    }
}

impl GridWorldState {
    pub fn new(x: i64, y: i64) -> GridWorldState {
        GridWorldState { x: x, y: y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }
}
