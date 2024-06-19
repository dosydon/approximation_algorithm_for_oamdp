#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum RaceTrackState {
    Dummy,
    Wrapper(RaceTrackStateInner),
}

impl RaceTrackState {
    pub fn new(x: usize, y: usize, dx: i32, dy: i32) -> RaceTrackState {
        RaceTrackState::Wrapper(RaceTrackStateInner::new(x, y, dx, dy))
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct RaceTrackStateInner {
    pub x: usize,
    pub y: usize,
    pub dx: i32,
    pub dy: i32,
}

impl RaceTrackStateInner {
    pub fn new(x: usize, y: usize, dx: i32, dy: i32) -> RaceTrackStateInner {
        RaceTrackStateInner {
            x: x,
            y: y,
            dx: dx,
            dy: dy,
        }
    }
}
