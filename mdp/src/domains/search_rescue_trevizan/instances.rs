use crate::domains::search_rescue_trevizan::cell_status::CellStatus::*;
use super::mdp::SRMDP;
use super::state::SRState;
use super::speed::Speed;

pub fn tiny() -> SRMDP<2> {
    let cells = [
        [NoSurvivor, ProbLow],
        [ProbLow, Survivor],
    ];
    let s = SRState::new(
        cells,
        (0, 0),
        false,
        Speed::Medium
    );

    SRMDP::new(s, (0, 0))
}

pub fn small() -> SRMDP<3> {
    let cells = [
        [NoSurvivor, ProbLow, ProbHigh],
        [ProbLow, ProbLow, ProbHigh],
        [NoSurvivor, ProbLow, Survivor],
    ];
    let s = SRState::new(cells, (0, 0), false, Speed::Medium);
    SRMDP::new(s, (0, 0))
}

pub fn medium() -> SRMDP<4> {
    let cells = [
        [NoSurvivor, ProbLow, ProbHigh, ProbHigh],
        [ProbLow, ProbLow, ProbHigh, NoSurvivor],
        [NoSurvivor, ProbLow, Survivor, NoSurvivor],
        [NoSurvivor, ProbLow, Survivor, NoSurvivor],
    ];
    let s = SRState::new(cells, (0, 0), false, Speed::Medium);
    SRMDP::new(s, (0, 3))
}

pub fn large() -> SRMDP<5> {
    let cells = [
        [NoSurvivor, ProbLow, ProbHigh, ProbHigh, ProbLow],
        [ProbLow, ProbLow, ProbHigh, NoSurvivor, ProbLow],
        [NoSurvivor, ProbLow, ProbHigh, NoSurvivor, ProbLow],
        [NoSurvivor, ProbLow, Survivor, NoSurvivor, ProbLow],
        [NoSurvivor, ProbLow, Survivor, NoSurvivor, ProbLow],
    ];
    let s = SRState::new(cells, (0, 0), false, Speed::Medium);
    SRMDP::new(s, (0, 0))
}