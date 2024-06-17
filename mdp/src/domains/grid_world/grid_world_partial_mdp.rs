use crate::grid_world::GridWorldState;

#[derive(PartialEq, Debug, Clone)]
pub struct GridWorldPartialMDP {
    pub(in crate::grid_world) h: i64,
    pub(in crate::grid_world) w: i64,
    pub(in crate::grid_world) initial_state: GridWorldState,
    pub(in crate::grid_world) watery_states: Vec<GridWorldState>,
    pub(in crate::grid_world) obstacled_states: Vec<GridWorldState>,
}

impl GridWorldPartialMDP {
    pub fn new(
        h: i64,
        w: i64,
        initial_state: GridWorldState,
        watery_states: Vec<GridWorldState>,
        obstacled_states: Vec<GridWorldState>,
    ) -> GridWorldPartialMDP {
        GridWorldPartialMDP {
            h: h,
            w: w,
            initial_state: initial_state,
            watery_states: watery_states,
            obstacled_states: obstacled_states,
        }
    }
}
