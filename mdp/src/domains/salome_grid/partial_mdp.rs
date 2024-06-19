use crate::{common::coordinate2::Coordinate2, mdp_traits::BuildFrom};

use super::{
    mdp_water_fire::{AgentType, SalomeGridWaterFireMDP},
    SalomeGridState,
};

#[derive(PartialEq, Debug, Clone)]
pub struct SalomeGridWaterFirePartialMDP {
    height: usize,
    width: usize,
    obstacles: Vec<SalomeGridState>,
    goal: SalomeGridState,
    water_cells: Vec<SalomeGridState>,
    fire_cells: Vec<SalomeGridState>,
    initial_state: SalomeGridState,
}

impl SalomeGridWaterFirePartialMDP {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<SalomeGridState>,
        goal: SalomeGridState,
        water_cells: Vec<SalomeGridState>,
        fire_cells: Vec<SalomeGridState>,
        initial_state: SalomeGridState,
    ) -> SalomeGridWaterFirePartialMDP {
        SalomeGridWaterFirePartialMDP {
            height: height,
            width: width,
            obstacles: obstacles,
            goal: goal,
            water_cells: water_cells,
            fire_cells: fire_cells,
            initial_state: initial_state,
        }
    }

    pub fn example_water_fire() -> SalomeGridWaterFirePartialMDP {
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
            SalomeGridState::new(6, 2),
            SalomeGridState::new(5, 2),
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
            SalomeGridState::new(6, 4),
            SalomeGridState::new(5, 4),
        ];
        let mdp = SalomeGridWaterFirePartialMDP::new(
            7,
            7,
            obstacles,
            Coordinate2 { i: 0, j: 3 },
            waters,
            fires,
            SalomeGridState::new(6, 3),
        );

        mdp
    }
}

impl BuildFrom<AgentType, SalomeGridWaterFireMDP> for SalomeGridWaterFirePartialMDP {
    fn build_from(&self, from: AgentType) -> SalomeGridWaterFireMDP {
        SalomeGridWaterFireMDP::new(
            self.height,
            self.width,
            self.obstacles.clone(),
            self.water_cells.clone(),
            self.fire_cells.clone(),
            self.goal,
            from,
        )
        .set_initial_state(self.initial_state)
    }
}
