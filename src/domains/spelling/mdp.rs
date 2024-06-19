use crate::common::coordinate2::Coordinate2;
use crate::common::grid2d::Grid2D;
use crate::mdp_traits::*;
use crate::state_enumerable_wrapper::StateEnumerableWrapper;
use core::slice::Iter;

use super::action::SpellingAction;
use super::action::SpellingAction::*;
use super::letter::Letter;
use super::state::SpellingState;

#[derive(Debug, Clone)]
pub struct SpellingEnv<const N: usize> {
    pub(crate) grid2d: Grid2D,
    pub(crate) letter_locs: [(usize, usize); N],
}

pub struct SpellingMDP<const N: usize> {
    pub env: SpellingEnv<N>,
    pub goal: [Letter; N],
    all_actions: [SpellingAction; 10],
    fail_prob: f32,
    initial_state: SpellingState<N>,
}

pub type SpellingMDPE<const NL: usize> = StateEnumerableWrapper<SpellingMDP<NL>>;

impl<const N: usize> SpellingMDP<N> {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<Coordinate2>,
        goal: [Letter; N],
        letter_locs: [(usize, usize); N],
        initial_state: SpellingState<N>,
    ) -> SpellingMDP<N> {
        let mut is_obstacled = vec![vec![false; width]; height];
        for s in obstacles.into_iter() {
            is_obstacled[s.i as usize][s.j as usize] = true;
        }

        SpellingMDP {
            env: SpellingEnv {
                grid2d: Grid2D::new(height, width, is_obstacled),
                letter_locs: letter_locs,
            },
            goal: goal,
            all_actions: [
                North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest, Stay, Toggle,
            ],
            fail_prob: 0.1,
            initial_state: initial_state,
            //             initial_state: Coordinate2::new((height - 1) as i32, 0),
        }
    }
}

impl<const N: usize> StatesActions for SpellingMDP<N> {
    type State = SpellingState<N>;
    type Action = SpellingAction;
}

impl<const N: usize> ActionEnumerable for SpellingMDP<N> {
    fn id_to_action(&self, id: usize) -> &Self::Action {
        &self.all_actions[id]
    }

    fn enumerate_actions(&self) -> Iter<Self::Action> {
        self.all_actions.iter()
    }

    fn num_actions(&self) -> usize {
        self.all_actions.len()
    }
}

impl<const N: usize> SpellingMDP<N> {
    pub(crate) fn success(&self, st: &Coordinate2, at: &SpellingAction) -> Coordinate2 {
        let di = at.get_di();
        let dj = at.get_dj();
        let new_i = st.i + di;
        let new_j = st.j + dj;
        if self.env.grid2d.is_valid_cordinate(new_i, new_j) {
            Coordinate2 { i: new_i, j: new_j }
        } else {
            *st
        }
    }

    pub(crate) fn next_letters(&self, s: &SpellingState<N>, a: &SpellingAction) -> [Letter; N] {
        let mut letters = s.letters;
        match a {
            Toggle => {
                for i in 0..N {
                    if self.env.letter_locs[i] == (s.coord.i as usize, s.coord.j as usize) {
                        letters[i] = letters[i].toggle();
                    }
                }
                letters
            }
            _ => return letters,
        }
    }

    pub(crate) fn next_letters_toggle_too_much(
        &self,
        s: &SpellingState<N>,
        a: &SpellingAction,
    ) -> [Letter; N] {
        let mut letters = s.letters;
        match a {
            Toggle => {
                for i in 0..N {
                    if self.env.letter_locs[i] == (s.coord.i as usize, s.coord.j as usize) {
                        letters[i] = letters[i].toggle().toggle();
                    }
                }
                letters
            }
            _ => return letters,
        }
    }
}

impl<const N: usize> ActionAvailability for SpellingMDP<N> {}

impl<const N: usize> IsTerminal for SpellingMDP<N> {
    fn is_terminal(&self, s: &Self::State) -> bool {
        s.letters == self.goal
    }
}

impl<const N: usize> InitialState for SpellingMDP<N> {
    fn initial_state(&self) -> Self::State {
        self.initial_state
    }
}

impl<const N: usize> PMassMutFrom<f32> for SpellingMDP<N> {}

impl<const N: usize> PMass<f32> for SpellingMDP<N> {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        let dj = a.get_dj();
        let di = a.get_di();

        if self.is_terminal(s)
            || !self.env.grid2d.within_bound(s.coord.i + di, s.coord.j + dj)
            || self.env.grid2d.is_obstacled[(s.coord.i + di) as usize][(s.coord.j + dj) as usize]
        {
            vec![(*s, 1.0)]
        } else {
            match a {
                Toggle => {
                    if self.fail_prob > 0.0 {
                        vec![
                            (
                                SpellingState::new(
                                    self.success(&s.coord, a),
                                    self.next_letters(s, a),
                                ),
                                1.0 - self.fail_prob,
                            ),
                            (
                                SpellingState::new(
                                    self.success(&s.coord, a),
                                    self.next_letters_toggle_too_much(s, a),
                                ),
                                self.fail_prob,
                            ),
                        ]
                    } else {
                        vec![(
                            SpellingState::new(self.success(&s.coord, a), self.next_letters(s, a)),
                            1.0,
                        )]
                    }
                }
                _ => {
                    vec![(
                        SpellingState::new(self.success(&s.coord, a), s.letters),
                        1.0,
                    )]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::*;
    use crate::{
        domains::spelling::letter::Letter::*, episode_runner::CostEpisodeIterator,
        policy::tabular_policy::TabularPolicy, state_enumerable_wrapper::StateEnumerableWrapper,
        value_iteration::value_iteration_ssp,
    };
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_spelling_p_mass() {
        let mdp = SpellingMDP::new(
            5,
            5,
            vec![],
            [A, R, M, S],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]),
        );
        let s = SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]);

        assert_eq!(
            vec![(
                SpellingState::new(Coordinate2::new(0, 1), [A, A, A, A]),
                1.0
            )],
            mdp.p_mass(&s, &East)
        );
    }

    #[test]
    fn test_spelling_value_iteration() {
        let mdp = SpellingMDP::new(
            5,
            5,
            vec![],
            [A, R, M, S],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]),
        );
        let mdp = StateEnumerableWrapper::new(mdp);

        let mut rng = thread_rng();
        let vt = value_iteration_ssp(&mdp);
        assert_approx_eq!(vt.get_value(&mdp.initial_state()), 18.627785);
        let policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        for (s, _a, _, _) in
            CostEpisodeIterator::new(&mdp, &policy, mdp.initial_state(), &mut rng, None)
        {
            mdp.display(&s);
            //             println!("{:?} {:?}", s, a);
        }
    }
}
