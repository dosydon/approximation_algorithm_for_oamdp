use core::slice::Iter;
use itertools::iproduct;

use crate::common::coordinate2::Coordinate2;
use crate::grid_turning::Direction::*;
use crate::grid_turning::*;
use crate::mdp_traits::*;

#[derive(PartialEq, Debug, Clone)]
pub struct GridTurningMDP {
    pub height: usize,
    pub width: usize,
    pub(in crate::grid_turning) is_obstacled: Vec<Vec<bool>>,
    pub(in crate::grid_turning) initial_state: GridTurningState,
    pub(in crate::grid_turning) goal: Coordinate2,
    all_states: Vec<GridTurningState>,
    all_actions: [GridTurningAction; 3],
    prob_veering: f32,
}

impl GridTurningMDP {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<Coordinate2>,
        initial_state: GridTurningState,
        goal: Coordinate2,
        prob_veering: f32,
    ) -> GridTurningMDP {
        let mut is_obstacled = vec![vec![false; width]; height];
        for s in obstacles.into_iter() {
            is_obstacled[s.i as usize][s.j as usize] = true;
        }
        let directions = [
            North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest,
        ];

        GridTurningMDP {
            width: width,
            height: height,
            initial_state: initial_state,
            goal: goal,
            all_states: iproduct!((0..height), (0..width), directions.iter())
                .filter(|(i, j, _)| !is_obstacled[*i][*j])
                .map(|(i, j, d)| GridTurningState {
                    i: i as i32,
                    j: j as i32,
                    direction: *d,
                })
                .collect::<Vec<_>>(),
            all_actions: [
                GridTurningAction::TurnLeft,
                GridTurningAction::TurnRight,
                GridTurningAction::GoStraight,
            ],
            is_obstacled: is_obstacled,
            prob_veering: prob_veering,
        }
    }

    fn within_bound(&self, i: i32, j: i32) -> bool {
        (0 <= i) && (i < self.height as i32) && (j >= 0) && (j < self.width as i32)
    }

    fn is_valid_cordinate(&self, i: i32, j: i32) -> bool {
        if !self.within_bound(i, j) {
            false
        } else if self.is_obstacled[i as usize][j as usize] {
            false
        } else {
            true
        }
    }

    fn go_straight(&self, st: &GridTurningState) -> GridTurningState {
        let di = get_di(st.direction);
        let dj = get_dj(st.direction);
        let new_i = st.i + di;
        let new_j = st.j + dj;
        if self.is_valid_cordinate(new_i, new_j) {
            GridTurningState {
                i: new_i,
                j: new_j,
                direction: st.direction,
            }
        } else {
            *st
        }
    }

    pub fn is_terminal(&self, s: &GridTurningState) -> bool {
        self.goal.i == s.i && self.goal.j == s.j
    }
}

impl ActionAvailability for GridTurningMDP {}

impl StatesActions for GridTurningMDP {
    type State = GridTurningState;
    type Action = GridTurningAction;
}

impl IsTerminal for GridTurningMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        self.is_terminal(s)
    }
}

impl StateEnumerable for GridTurningMDP {
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

impl ActionEnumerable for GridTurningMDP {
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

impl InitialState for GridTurningMDP {
    fn initial_state(&self) -> Self::State {
        self.initial_state
    }
}

impl PMass<f32> for GridTurningMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) || self.is_obstacled[s.i as usize][s.j as usize] {
            vec![(*s, 1.0)]
        } else {
            match *a {
                GridTurningAction::TurnLeft => {
                    if self.prob_veering > 0.0 {
                        vec![
                            (
                                GridTurningState {
                                    i: s.i,
                                    j: s.j,
                                    direction: turn_left(s.direction),
                                },
                                (1.0 - self.prob_veering),
                            ),
                            (
                                GridTurningState {
                                    i: s.i,
                                    j: s.j,
                                    direction: veer_left(s.direction),
                                },
                                (self.prob_veering),
                            ),
                        ]
                    } else {
                        vec![(
                            GridTurningState {
                                i: s.i,
                                j: s.j,
                                direction: turn_left(s.direction),
                            },
                            1.0,
                        )]
                    }
                }
                GridTurningAction::TurnRight => {
                    if self.prob_veering > 0.0 {
                        vec![
                            (
                                GridTurningState {
                                    i: s.i,
                                    j: s.j,
                                    direction: turn_right(s.direction),
                                },
                                (1.0 - self.prob_veering),
                            ),
                            (
                                GridTurningState {
                                    i: s.i,
                                    j: s.j,
                                    direction: veer_right(s.direction),
                                },
                                (self.prob_veering),
                            ),
                        ]
                    } else {
                        vec![(
                            GridTurningState {
                                i: s.i,
                                j: s.j,
                                direction: turn_right(s.direction),
                            },
                            1.0,
                        )]
                    }
                }
                GridTurningAction::GoStraight => vec![(self.go_straight(s), 1.0)],
            }
        }
    }
}

// impl PMassMut<f32> for GridTurningMDP {
//     type Distribution = Vec<(Self::State, f32)>;
//     fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
//         self.p_mass(s, a)
//     }
// }

impl ExplicitTransition for GridTurningMDP {}

fn distance(s: &GridTurningState, ss: &GridTurningState) -> f32 {
    (((s.i - ss.i) * (s.i - ss.i) + (s.j - ss.j) * (s.j - ss.j)) as f32).sqrt()
}

impl DCost for GridTurningMDP {
    fn d_cost(&self, st: &Self::State, _a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else {
            distance(st, stt).max(1.0)
        }
    }
}

impl CostFromDCost for GridTurningMDP {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_iteration::value_iteration_ssp;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_grid_turning_p_mass() {
        let initial_state = GridTurningState::new(4, 0, North);
        let goal = Coordinate2::new(4, 4);
        let mdp = GridTurningMDP::new(
            5,
            5,
            vec![Coordinate2::new(4, 2), Coordinate2::new(3, 2)],
            initial_state,
            goal,
            0.0,
        );
        let s = GridTurningState {
            i: 4,
            j: 1,
            direction: North,
        };

        let expected = GridTurningState {
            i: 3,
            j: 1,
            direction: North,
        };
        assert_eq!(
            vec![(expected, 1.0)],
            mdp.p_mass(&s, &GridTurningAction::GoStraight)
        );
    }

    #[test]
    fn test_grid_turning_value_iteration() {
        let initial_state = GridTurningState::new(2, 0, North);
        let goal = Coordinate2::new(0, 2);
        let mdp = GridTurningMDP::new(3, 3, vec![], initial_state, goal, 0.0);
        let goal = GridTurningState {
            i: 0,
            j: 2,
            direction: NorthEast,
        };
        let left_top = GridTurningState {
            i: 0,
            j: 0,
            direction: North,
        };
        let value_table = value_iteration_ssp(&mdp);
        let err = 1e-3;

        assert_approx_eq!(value_table.get_value(&goal), 0.0, err);
        assert_approx_eq!(value_table.get_value(&left_top), 4.0, err);
        //         for (id, s) in mdp.enumerate_states().enumerate() {
        //             println!("{:?} {:?}", s, value_table.get_value(s));
        //             //             assert!(value_table.get_value(s) - err <= expected[id]);
        //         }
    }

    #[test]
    fn test_grid_turning_value_iteration_stochastic() {
        let initial_state = GridTurningState::new(2, 0, North);
        let goal = Coordinate2::new(0, 2);
        let mdp = GridTurningMDP::new(3, 3, vec![], initial_state, goal, 0.8);
        let value_table = value_iteration_ssp(&mdp);

        for (_id, s) in mdp.enumerate_states().enumerate() {
            println!("{:?} {:?}", s, value_table.get_value(s));
            //             assert!(value_table.get_value(s) - err <= expected[id]);
        }
    }
}
