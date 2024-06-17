use std::slice::Iter;

use itertools::iproduct;
use strum::IntoEnumIterator;

use crate::{
    baker_grid::{distance::distance, BakerGridAction, BakerGridState},
    common::grid2d::Grid2D,
    mdp_traits::{
        ActionAvailability, ActionEnumerable, CostFromDCost, DCost, ExplicitTransition,
        GetNextState, InitialState, IsTerminal, PMass, StateEnumerable, StatesActions,
    },
};

#[derive(PartialEq, Debug, Clone)]
pub struct BakerMultiGridMDP<const N: usize> {
    pub grid2d: Grid2D,
    pub(in crate::baker_grid::baker_multi_grid) is_dangerous: Vec<Vec<bool>>,
    pub goal: [BakerGridState; N],
    all_states: Vec<[BakerGridState; N]>,
    all_actions: Vec<[BakerGridAction; N]>,
    pub(in crate::baker_grid::baker_multi_grid) initial_state: [BakerGridState; N],
    pub cost_upon_dangerous: f32,
}

impl BakerMultiGridMDP<3> {
    //     #[new]
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<BakerGridState>,
        initial_state: [BakerGridState; 3],
        goal: [BakerGridState; 3],
    ) -> BakerMultiGridMDP<3> {
        let mut is_obstacled = vec![vec![false; width]; height];
        let is_dangerous = vec![vec![false; width]; height];
        for s in obstacles.into_iter() {
            is_obstacled[s.i as usize][s.j as usize] = true;
        }
        let all_states = iproduct!(
            (0..height),
            (0..width),
            (0..height),
            (0..width),
            (0..height),
            (0..width)
        )
        .filter(|(i0, j0, i1, j1, i2, j2)| {
            !is_obstacled[*i0][*j0] || !is_obstacled[*i1][*j1] || !is_obstacled[*i2][*j2]
        })
        .map(|(i0, j0, i1, j1, i2, j2)| {
            [
                BakerGridState {
                    i: i0 as i32,
                    j: j0 as i32,
                },
                BakerGridState {
                    i: i1 as i32,
                    j: j1 as i32,
                },
                BakerGridState {
                    i: i2 as i32,
                    j: j2 as i32,
                },
            ]
        })
        .collect::<Vec<_>>();

        let all_actions = iproduct!(
            BakerGridAction::iter(),
            BakerGridAction::iter(),
            BakerGridAction::iter()
        )
        .map(|(a0, a1, a2)| [a0, a1, a2])
        .collect::<Vec<_>>();

        BakerMultiGridMDP {
            grid2d: Grid2D::new(height, width, is_obstacled),
            goal: goal,
            all_states: all_states,
            all_actions: all_actions,
            is_dangerous: is_dangerous,
            initial_state: initial_state,
            cost_upon_dangerous: 10.0,
        }
    }
}

impl<const N: usize> StatesActions for BakerMultiGridMDP<N> {
    type State = [BakerGridState; N];
    type Action = [BakerGridAction; N];
}

impl<const N: usize> IsTerminal for BakerMultiGridMDP<N> {
    fn is_terminal(&self, s: &Self::State) -> bool {
        *s == self.goal
    }
}

impl<const N: usize> StateEnumerable for BakerMultiGridMDP<N> {
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

impl<const N: usize> ActionEnumerable for BakerMultiGridMDP<N> {
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

impl<const N: usize> ActionAvailability for BakerMultiGridMDP<N> {
    fn action_available(&self, _s: &Self::State, _a: &Self::Action) -> bool {
        true
    }
}

impl<const N: usize> InitialState for BakerMultiGridMDP<N> {
    fn initial_state(&self) -> Self::State {
        self.initial_state
    }
}

impl<const N: usize> PMass<f32> for BakerMultiGridMDP<N> {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) {
            vec![(*s, 1.0)]
        } else {
            let mut next_state = *s;
            for i in 0..N {
                next_state[i] = self.grid2d.success(&s[i], &a[i]);
            }

            vec![(next_state, 1.0)]
        }
    }
}

impl<const N: usize> ExplicitTransition for BakerMultiGridMDP<N> {
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(st) {
            if st == stt {
                1.0
            } else {
                0.0
            }
        } else {
            let mut next_state = *st;
            for i in 0..N {
                next_state[i] = self.grid2d.success(&st[i], &a[i]);
            }
            if next_state == *stt {
                1.0
            } else {
                0.0
            }
        }
    }
}

fn distance_n<const N: usize>(s0: &[BakerGridState; N], s1: &[BakerGridState; N]) -> f32 {
    let mut r = 0.0;
    for i in 0..N {
        r += distance(&s0[i], &s1[i])
    }
    r
}

impl<const N: usize> DCost for BakerMultiGridMDP<N> {
    fn d_cost(&self, s: &Self::State, _a: &Self::Action, ss: &Self::State) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            distance_n(s, ss).max(1.0)
        }
    }
}

impl<const N: usize> CostFromDCost for BakerMultiGridMDP<N> {}

impl<const N: usize> GetNextState for BakerMultiGridMDP<N> {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        _rng: &mut rand::prelude::ThreadRng,
    ) -> Self::State {
        let mut next_state = *s;
        for i in 0..N {
            next_state[i] = self.grid2d.success(&s[i], &a[i]);
        }
        next_state
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::{
        episode_runner::EpisodeRunner, policy::tabular_policy::TabularPolicy,
        value_iteration::value_iteration_ssp,
    };

    use super::*;

    #[test]
    fn test_baker_multi_grid_p_mass() {
        let mdp = BakerMultiGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            [
                BakerGridState::new(4, 0),
                BakerGridState::new(3, 0),
                BakerGridState::new(2, 0),
            ],
            [
                BakerGridState::new(4, 4),
                BakerGridState::new(3, 4),
                BakerGridState::new(2, 4),
            ],
        );

        for s in mdp.all_states.iter() {
            println!("{:?}", s);
        }

        //         assert_eq!(
        //             vec![(BakerGridState::new(4, 1), 1.0)],
        //             mdp.p_mass(&BakerGridState::new(4, 1), &East)
        //         );
    }

    #[test]
    fn test_baker_multi_grid_value_iteration() {
        let mdp = BakerMultiGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            [
                BakerGridState::new(4, 0),
                BakerGridState::new(3, 0),
                BakerGridState::new(2, 0),
            ],
            [
                BakerGridState::new(4, 4),
                BakerGridState::new(3, 4),
                BakerGridState::new(2, 4),
            ],
        );

        let vt = value_iteration_ssp(&mdp);
        let mut rng = thread_rng();
        let policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        let mut runner = EpisodeRunner::new(&mdp, &policy, mdp.initial_state());

        for (s, _a, _ss, _c) in runner.into_iter_with(&mut rng) {
            println!("{:?}", s);
        }
    }
}
