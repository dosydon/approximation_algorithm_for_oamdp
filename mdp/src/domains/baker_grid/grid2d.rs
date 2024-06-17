use crate::common::grid2d::Grid2D;

use super::{action::BakerGridAction::*, BakerGridAction, BakerGridState};

impl Grid2D {
    pub(crate) fn success(&self, st: &BakerGridState, at: &BakerGridAction) -> BakerGridState {
        let di = at.get_di();
        let dj = at.get_dj();
        let new_i = st.i + di;
        let new_j = st.j + dj;
        if self.is_valid_cordinate(new_i, new_j) {
            BakerGridState { i: new_i, j: new_j }
        } else {
            *st
        }
    }

    pub(crate) fn veer_right(&self, s: &BakerGridState, a: &BakerGridAction) -> BakerGridState {
        self.success(
            s,
            &(match a {
                North => NorthEast,
                South => SouthWest,
                East => SouthEast,
                West => NorthWest,
                NorthEast => East,
                NorthWest => North,
                SouthEast => South,
                SouthWest => West,
                Stay => Stay,
            }),
        )
    }

    pub(crate) fn veer_left(&self, s: &BakerGridState, a: &BakerGridAction) -> BakerGridState {
        self.success(
            s,
            &(match a {
                North => NorthWest,
                South => SouthEast,
                East => NorthEast,
                West => SouthWest,
                NorthEast => North,
                NorthWest => West,
                SouthEast => East,
                SouthWest => South,
                Stay => Stay,
            }),
        )
    }
}
