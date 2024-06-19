#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SimpleAVParameter {
    Stopping(usize, usize),
    NonYield(usize, i32, i32),
    YouHaveLightOff(usize, i32, i32),
}
