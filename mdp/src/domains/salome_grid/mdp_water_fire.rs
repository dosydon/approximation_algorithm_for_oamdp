use super::{
    action::{get_di, get_dj},
    SalomeGridAction, SalomeGridMDP, SalomeGridState,
};
use crate::mdp_traits::*;
use std::slice::Iter;

#[derive(PartialEq, Debug, Clone)]
pub enum AgentType {
    WaterResistent,
    FireResistent,
}

#[derive(PartialEq, Debug, Clone)]
enum GridStatus {
    Water,
    Fire,
    Normal,
    Obstacled,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SalomeGridWaterFireMDP {
    pub mdp: SalomeGridMDP,
    grid_status: Vec<Vec<GridStatus>>,
    agent_type: AgentType,
}

impl SalomeGridWaterFireMDP {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<SalomeGridState>,
        water_cells: Vec<SalomeGridState>,
        fire_cells: Vec<SalomeGridState>,
        goal: SalomeGridState,
        agent_type: AgentType,
    ) -> SalomeGridWaterFireMDP {
        let mut grid_status = vec![vec![GridStatus::Normal; width]; height];
        for s in obstacles.iter() {
            grid_status[s.i as usize][s.j as usize] = GridStatus::Obstacled;
        }
        for s in water_cells.iter() {
            grid_status[s.i as usize][s.j as usize] = GridStatus::Water;
        }
        for s in fire_cells.iter() {
            grid_status[s.i as usize][s.j as usize] = GridStatus::Fire;
        }
        SalomeGridWaterFireMDP {
            mdp: SalomeGridMDP::new(height, width, obstacles, goal),
            grid_status: grid_status,
            agent_type: agent_type,
        }
    }

    pub fn width(&self) -> usize {
        self.mdp.width()
    }

    pub fn height(&self) -> usize {
        self.mdp.height()
    }

    pub fn is_obstacled(&self, i: usize, j: usize) -> bool {
        self.mdp.is_obstacled(i, j)
    }

    pub fn set_initial_state(mut self, initial_state: SalomeGridState) -> SalomeGridWaterFireMDP {
        self.mdp.initial_state = initial_state;
        self
    }
}

impl ActionAvailability for SalomeGridWaterFireMDP {}

impl StatesActions for SalomeGridWaterFireMDP {
    type State = SalomeGridState;
    type Action = SalomeGridAction;
}

impl IsTerminal for SalomeGridWaterFireMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        self.mdp.is_terminal(s)
    }
}

impl StateEnumerable for SalomeGridWaterFireMDP {
    fn enumerate_states(&self) -> Iter<Self::State> {
        self.mdp.enumerate_states()
    }
    fn num_states(&self) -> usize {
        self.mdp.num_states()
    }
    fn id_to_state(&self, id: usize) -> &Self::State {
        self.mdp.id_to_state(id)
    }
}

impl ActionEnumerable for SalomeGridWaterFireMDP {
    fn enumerate_actions(&self) -> Iter<Self::Action> {
        self.mdp.enumerate_actions()
    }
    fn num_actions(&self) -> usize {
        self.mdp.num_actions()
    }
    fn id_to_action(&self, id: usize) -> &Self::Action {
        self.mdp.id_to_action(id)
    }
}

impl InitialState for SalomeGridWaterFireMDP {
    fn initial_state(&self) -> Self::State {
        self.mdp.initial_state
    }
}

impl PMass<f32> for SalomeGridWaterFireMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        self.mdp.p_mass(s, a)
    }
}

impl ExplicitTransition for SalomeGridWaterFireMDP {
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        self.mdp.p(st, a, stt)
    }
}

impl DCost for SalomeGridWaterFireMDP {
    fn d_cost(&self, st: &Self::State, a: &Self::Action, _stt: &Self::State) -> f32 {
        let dj = get_dj(*a);
        let di = get_di(*a);

        if self.is_terminal(st) {
            0.0
        } else if !self.mdp.grid2d.is_valid_cordinate(st.i + di, st.j + dj) {
            1.0
        } else {
            match self.grid_status[(st.i + di) as usize][(st.j + dj) as usize] {
                GridStatus::Fire => match self.agent_type {
                    AgentType::FireResistent => 0.04,
                    AgentType::WaterResistent => 1.0,
                },
                GridStatus::Water => match self.agent_type {
                    AgentType::FireResistent => 1.0,
                    AgentType::WaterResistent => 0.04,
                },
                GridStatus::Obstacled => {
                    panic!("should not be reachable")
                }
                GridStatus::Normal => 0.04,
            }
        }
    }
}

impl Cost for SalomeGridWaterFireMDP {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            self.p_mass(s, a)
                .into_iter()
                .map(|(stt, prob)| prob * self.d_cost(s, a, &stt))
                .sum()
        }
    }
}

impl GetNextState for SalomeGridWaterFireMDP {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        _rng: &mut rand::prelude::ThreadRng,
    ) -> Self::State {
        self.mdp.success(s, a)
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::{
        baker_grid::GridAndGoals, episode_runner::EpisodeRunner,
        policy::tabular_policy::TabularPolicy, value_iteration::value_iteration_ssp,
    };

    use super::*;

    #[test]
    fn test_salome_grid_water_fire_value_iteration() {
        let mdp = SalomeGridWaterFireMDP::example_water_fire(AgentType::WaterResistent);
        let grid = mdp.mdp.grid2d.clone();
        let grid_and_goals = GridAndGoals::new(grid, vec![(0, 3)], vec!["G".to_string()]);
        let s = mdp.initial_state();
        grid_and_goals.display(&s);
        let vt = value_iteration_ssp(&mdp);
        for s in mdp.enumerate_states() {
            println!("{:?} {}", s, vt.get_value(s));
        }
        let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);

        let mut rng = thread_rng();
        let mut runner = EpisodeRunner::new(&mdp, &tabular_policy, mdp.initial_state());

        for (s, a, _, _) in runner.into_iter_with(&mut rng) {
            println!("{:?}", s);
            println!("{:?}", a);
            grid_and_goals.display(&s);
            println!("");
        }
    }
}
