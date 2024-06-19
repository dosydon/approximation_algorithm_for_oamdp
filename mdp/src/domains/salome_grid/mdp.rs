use std::slice::Iter;

use itertools::iproduct;

use crate::common::grid2d::Grid2D;

use super::action::{get_di, SalomeGridAction};
use super::action::{get_dj, SalomeGridAction::*};
use super::state::SalomeGridState;
use crate::mdp_traits::*;

#[derive(PartialEq, Debug, Clone)]
pub struct SalomeGridMDP {
    pub grid2d: Grid2D,
    pub goal: SalomeGridState,
    all_states: Vec<SalomeGridState>,
    all_actions: [SalomeGridAction; 4],
    pub(crate) initial_state: SalomeGridState,
}

impl SalomeGridMDP {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<SalomeGridState>,
        goal: SalomeGridState,
    ) -> SalomeGridMDP {
        let mut is_obstacled = vec![vec![false; width]; height];
        for s in obstacles.into_iter() {
            is_obstacled[s.i as usize][s.j as usize] = true;
        }
        let all_states = iproduct!((0..height), (0..width))
            .filter(|(i, j)| !is_obstacled[*i][*j])
            .map(|(i, j)| SalomeGridState {
                i: i as i32,
                j: j as i32,
            })
            .collect::<Vec<_>>();

        SalomeGridMDP {
            grid2d: Grid2D::new(height, width, is_obstacled),
            goal: goal,
            all_states: all_states,
            all_actions: [Left, Up, Right, Down],
            initial_state: SalomeGridState::new((height - 1) as i32, 0),
        }
    }

    pub fn width(&self) -> usize {
        self.grid2d.width
    }

    pub fn height(&self) -> usize {
        self.grid2d.height
    }

    pub fn is_obstacled(&self, i: usize, j: usize) -> bool {
        self.grid2d.is_obstacled[i][j]
    }

    pub fn set_initial_state(mut self, initial_state: SalomeGridState) -> SalomeGridMDP {
        self.initial_state = initial_state;
        self
    }
}

impl SalomeGridMDP {
    pub(crate) fn success(&self, st: &SalomeGridState, at: &SalomeGridAction) -> SalomeGridState {
        let di = get_di(*at);
        let dj = get_dj(*at);
        let new_i = st.i + di;
        let new_j = st.j + dj;
        if self.grid2d.is_valid_cordinate(new_i, new_j) {
            SalomeGridState { i: new_i, j: new_j }
        } else {
            *st
        }
    }
}

impl ActionAvailability for SalomeGridMDP {}

impl StatesActions for SalomeGridMDP {
    type State = SalomeGridState;
    type Action = SalomeGridAction;
}

impl IsTerminal for SalomeGridMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        *s == self.goal
    }
}

impl StateEnumerable for SalomeGridMDP {
    fn enumerate_states(&self) -> Iter<Self::State> {
        self.all_states.iter()
    }
    fn num_states(&self) -> usize {
        self.all_states.len()
    }
    fn id_to_state(&self, id: usize) -> &Self::State {
        &(self.all_states[id])
    }
}

impl ActionEnumerable for SalomeGridMDP {
    fn enumerate_actions(&self) -> Iter<Self::Action> {
        self.all_actions.iter()
    }
    fn num_actions(&self) -> usize {
        self.all_actions.len()
    }
    fn id_to_action(&self, id: usize) -> &Self::Action {
        &(self.all_actions[id])
    }
}

impl InitialState for SalomeGridMDP {
    fn initial_state(&self) -> Self::State {
        self.initial_state
    }
}

impl PMass<f32> for SalomeGridMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        let dj = get_dj(*a);
        let di = get_di(*a);

        if self.is_terminal(s) || !self.grid2d.is_valid_cordinate(s.i + di, s.j + dj) {
            vec![(*s, 1.0)]
        } else {
            vec![(self.success(s, a), 1.0)]
        }
    }
}

impl ExplicitTransition for SalomeGridMDP {
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(st) {
            if st == stt {
                1.0
            } else {
                0.0
            }
        } else {
            if *stt == self.success(st, a) {
                1.0
            } else {
                0.0
            }
        }
    }
}

impl DCost for SalomeGridMDP {
    fn d_cost(&self, st: &Self::State, a: &Self::Action, _stt: &Self::State) -> f32 {
        let dj = get_dj(*a);
        let di = get_di(*a);

        if self.is_terminal(st) {
            0.0
        } else if !self.grid2d.is_valid_cordinate(st.i + di, st.j + dj) {
            1.0
        } else {
            0.04
        }
    }
}

impl CostFromDCost for SalomeGridMDP {}

impl GetNextState for SalomeGridMDP {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        _rng: &mut rand::prelude::ThreadRng,
    ) -> Self::State {
        self.success(s, a)
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
    fn test_salome_grid_value_iteration() {
        let mdp = SalomeGridMDP::example();
        let vt = value_iteration_ssp(&mdp);
        let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);

        let grid = mdp.grid2d.clone();
        let grid_and_goals = GridAndGoals::new(grid, vec![(1, 9)], vec!["G".to_string()]);

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
