#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Coordinate {
    pub i: i64,
    pub j: i64,
}

impl Coordinate {
    pub fn new(i: i64, j: i64) -> Coordinate {
        Coordinate { i: i, j: j }
    }
}
