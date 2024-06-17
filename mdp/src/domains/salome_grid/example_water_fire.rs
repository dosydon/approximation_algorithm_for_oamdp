use crate::common::coordinate2::Coordinate2;

use super::{
    mdp_water_fire::{AgentType, SalomeGridWaterFireMDP},
    state::SalomeGridState,
};

impl SalomeGridWaterFireMDP {
    pub fn example_water_fire(agent_type: AgentType) -> SalomeGridWaterFireMDP {
        let obstacles = vec![
            SalomeGridState::new(1, 3),
            SalomeGridState::new(3, 0),
            SalomeGridState::new(3, 1),
            SalomeGridState::new(3, 2),
            SalomeGridState::new(3, 4),
            SalomeGridState::new(3, 5),
            SalomeGridState::new(3, 6),
        ];
        let fires = vec![
            SalomeGridState::new(1, 1),
            SalomeGridState::new(1, 2),
            SalomeGridState::new(0, 4),
            SalomeGridState::new(0, 5),
            SalomeGridState::new(0, 6),
            SalomeGridState::new(1, 6),
            SalomeGridState::new(2, 6),
            SalomeGridState::new(2, 5),
            SalomeGridState::new(2, 4),
        ];
        let waters = vec![
            SalomeGridState::new(1, 4),
            SalomeGridState::new(1, 5),
            SalomeGridState::new(0, 2),
            SalomeGridState::new(0, 1),
            SalomeGridState::new(0, 0),
            SalomeGridState::new(1, 0),
            SalomeGridState::new(2, 0),
            SalomeGridState::new(2, 1),
            SalomeGridState::new(2, 2),
        ];
        let mdp = SalomeGridWaterFireMDP::new(
            7,
            7,
            obstacles,
            waters,
            fires,
            Coordinate2 { i: 0, j: 3 },
            agent_type,
        )
        .set_initial_state(SalomeGridState::new(6, 1));
        mdp
    }
}
