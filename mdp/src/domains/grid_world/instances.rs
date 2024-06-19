use crate::grid_world::GridWorldMDP;
use crate::grid_world::GridWorldState;

impl GridWorldMDP {
    pub fn mid() -> GridWorldMDP {
        GridWorldMDP::new(
            15,
            15,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(0, 2), GridWorldState::new(1, 2)],
            vec![],
        )
    }
    pub fn watery() -> GridWorldMDP {
        GridWorldMDP::new(
            5,
            5,
            GridWorldState::new(0, 0),
            GridWorldState::new(4, 4),
            vec![GridWorldState::new(2, 2), GridWorldState::new(2, 4)],
            vec![GridWorldState::new(2, 3)],
        )
    }
    pub fn default() -> GridWorldMDP {
        GridWorldMDP::new(
            5,
            5,
            GridWorldState::new(0, 0),
            GridWorldState::new(4, 4),
            vec![GridWorldState::new(2, 4)],
            vec![GridWorldState::new(2, 2), GridWorldState::new(2, 3)],
        )
    }
}
