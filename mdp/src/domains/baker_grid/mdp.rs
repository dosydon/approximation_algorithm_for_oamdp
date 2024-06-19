use crate::baker_grid::BakerGridAction::*;
use crate::baker_grid::{BakerGridAction, BakerGridState};
use crate::common::grid2d::Grid2D;
use crate::mdp_traits::*;
use core::slice::Iter;
use itertools::iproduct;

use super::distance;

#[derive(PartialEq, Debug, Clone)]
pub struct BakerGridMDP {
    pub grid2d: Grid2D,
    pub(in crate::baker_grid) is_dangerous: Vec<Vec<bool>>,
    pub goal: BakerGridState,
    all_states: Vec<BakerGridState>,
    all_actions: [BakerGridAction; 9],
    pub prob_veering: f32,
    pub(crate) initial_state: BakerGridState,
    pub cost_upon_dangerous: f32,
}

impl BakerGridMDP {
    //     #[new]
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<BakerGridState>,
        goal: BakerGridState,
    ) -> BakerGridMDP {
        let mut is_obstacled = vec![vec![false; width]; height];
        let is_dangerous = vec![vec![false; width]; height];
        for s in obstacles.into_iter() {
            is_obstacled[s.i as usize][s.j as usize] = true;
        }
        let all_states = iproduct!((0..height), (0..width))
            .filter(|(i, j)| !is_obstacled[*i][*j])
            .map(|(i, j)| BakerGridState {
                i: i as i32,
                j: j as i32,
            })
            .collect::<Vec<_>>();

        BakerGridMDP {
            grid2d: Grid2D::new(height, width, is_obstacled),
            goal: goal,
            all_states: all_states,
            all_actions: [
                North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest, Stay,
            ],
            is_dangerous: is_dangerous,
            prob_veering: 0.0,
            initial_state: BakerGridState::new((height - 1) as i32, 0),
            cost_upon_dangerous: 10.0,
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

    pub fn set_initial_state(mut self, initial_state: BakerGridState) -> BakerGridMDP {
        self.initial_state = initial_state;
        self
    }

    pub fn set_prob_veering(mut self, prob_veering: f32) -> BakerGridMDP {
        self.prob_veering = prob_veering;
        self
    }

    pub fn add_dangerous_state(&mut self, i: usize, j: usize) {
        self.is_dangerous[i][j] = true;
    }
}

impl ActionAvailability for BakerGridMDP {}

impl StatesActions for BakerGridMDP {
    type State = BakerGridState;
    type Action = BakerGridAction;
}

impl IsTerminal for BakerGridMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        *s == self.goal
    }
}

impl StateEnumerable for BakerGridMDP {
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

impl ActionEnumerable for BakerGridMDP {
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

pub(crate) fn add_outcome(
    outcomes: &mut Vec<(BakerGridState, f32)>,
    outcome: BakerGridState,
    outcome_p: f32,
) {
    let mut flag = true;
    for (s, p) in outcomes.iter_mut() {
        if *s == outcome {
            *p += outcome_p;
            flag = false;
        }
    }

    if flag {
        outcomes.push((outcome, outcome_p));
    }
}

impl InitialState for BakerGridMDP {
    fn initial_state(&self) -> Self::State {
        self.initial_state
    }
}

impl PMass<f32> for BakerGridMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        let dj = a.get_dj();
        let di = a.get_di();

        if self.is_terminal(s) || !self.grid2d.is_valid_cordinate(s.i + di, s.j + dj) {
            vec![(*s, 1.0)]
        } else {
            let mut res = Vec::new();
            if self.prob_veering > 0.0 {
                add_outcome(&mut res, self.grid2d.success(s, a), 1.0 - self.prob_veering);
                add_outcome(
                    &mut res,
                    self.grid2d.veer_left(s, a),
                    self.prob_veering / 2.0,
                );
                add_outcome(
                    &mut res,
                    self.grid2d.veer_right(s, a),
                    self.prob_veering / 2.0,
                );
            } else {
                add_outcome(&mut res, self.grid2d.success(s, a), 1.0);
            }

            res
        }
    }
}

impl PMassMutFrom<f32> for BakerGridMDP {}
//     type Distribution = Vec<(Self::State, f32)>;
//     fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
//         self.p_mass(s, a)
//     }
// }

impl ExplicitTransition for BakerGridMDP {
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        let dj = a.get_dj();
        let di = a.get_di();

        if self.is_terminal(st) || !self.grid2d.is_valid_cordinate(st.i + di, st.j + dj) {
            if *stt == *st {
                0.0
            } else {
                0.0
            }
        } else {
            let mut p = 0.0;
            if self.prob_veering > 0.0 {
                if *stt == self.grid2d.success(st, a) {
                    p += 1.0 - self.prob_veering;
                }
                if *stt == self.grid2d.veer_left(st, a) {
                    p += self.prob_veering / 2.0;
                }
                if *stt == self.grid2d.veer_right(st, a) {
                    p += self.prob_veering / 2.0;
                }
            } else {
                if *stt == self.grid2d.success(st, a) {
                    p = 1.0;
                }
            }

            p
        }
    }
}

impl Rsa for BakerGridMDP {
    fn rsa(&self, s: &Self::State, a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            (-1.0) * self.cost(s, a)
        }
    }
}

impl Rsas for BakerGridMDP {
    fn rsas(&self, s: &Self::State, a: &Self::Action, _ss: &Self::State) -> f32 {
        self.rsa(s, a)
    }
}

impl DiscountFactor for BakerGridMDP {
    fn get_discount_factor(&self) -> f32 {
        0.9
    }
}

impl DCost for BakerGridMDP {
    fn d_cost(&self, st: &Self::State, _a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else if self.is_terminal(stt) {
            //             println!("{:?} {:?}", st, a);
            0.0
        } else {
            if self.is_dangerous[stt.i as usize][stt.j as usize] {
                distance::distance(st, stt).max(self.cost_upon_dangerous)
            } else {
                distance::distance(st, stt).max(1.0)
            }
        }
    }
}

impl CostFromDCost for BakerGridMDP {}

// impl Builder<(BakerGridState, f32), BakerGridMDP> for BakerGridPartialMDP {
//     fn build(&self, pair: (BakerGridState, f32)) -> BakerGridMDP {
//         let mut baker_grid =
//             BakerGridMDP::new(self.height, self.width, self.obstacles.clone(), pair.0);
//         baker_grid.prob_veering = self.prob_veering;
//         baker_grid.initial_state = self.initial_state;
//         baker_grid.cost_upon_dangerous = pair.1;
//         for coord in self.dangerous_coordinates.iter() {
//             baker_grid.is_dangerous[coord.0][coord.1] = true;
//         }
//
//         baker_grid
//     }
// }

impl PreferredSuccessor for BakerGridMDP {
    fn preferred_successor(&self, s: &Self::State, a: &Self::Action) -> Self::State {
        self.grid2d.success(s, a)
    }
}

#[cfg(test)]
mod tests {
    use crate::baker_grid::BakerGridPartialMDP;

    use super::*;

    #[test]
    fn test_baker_grid_p_mass() {
        let mdp = BakerGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            BakerGridState::new(4, 4),
        );

        assert_eq!(
            vec![(BakerGridState::new(4, 1), 1.0)],
            mdp.p_mass(&BakerGridState::new(4, 1), &East)
        );
    }

    #[test]
    fn test_baker_grid_cost() {
        let mdp = BakerGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            BakerGridState::new(4, 4),
        );

        assert_eq!(
            (2.0 as f32).sqrt(),
            mdp.cost(&BakerGridState::new(4, 1), &NorthWest)
        );
    }

    #[test]
    fn test_baker_grid_cost_upon_obstacle() {
        let mut mdp = BakerGridMDP::new(5, 5, vec![], BakerGridState::new(4, 4));

        mdp.cost_upon_dangerous = 10.0;
        mdp.is_dangerous[3][0] = true;

        assert_eq!(10.0, mdp.cost(&BakerGridState::new(4, 0), &North));
    }

    #[test]
    fn test_baker_grid_success() {
        let mdp = BakerGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            BakerGridState::new(4, 4),
        );

        assert_eq!(
            BakerGridState::new(3, 1),
            mdp.grid2d.success(&BakerGridState::new(4, 1), &North)
        );
    }

    #[test]
    fn test_baker_grid_p_mass_obstacle() {
        let width = 17;
        let height = 9;
        let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

        let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles);
        let mdp: BakerGridMDP = partial_mdp.build_from(&BakerGridState::new(0, 16));

        assert_eq!(
            vec![(BakerGridState::new(5, 7), 1.0)],
            mdp.p_mass(&BakerGridState::new(5, 7), &East)
        );
        assert_eq!(
            vec![(BakerGridState::new(4, 7), 1.0)],
            mdp.p_mass(&BakerGridState::new(4, 7), &SouthEast)
        );
    }

    #[test]
    fn test_baker_grid_p_mass_veering() {
        let width = 17;
        let height = 9;
        let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

        let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles).set_prob_veering(0.5);
        let mdp: BakerGridMDP = partial_mdp.build_from(&BakerGridState::new(0, 16));

        assert_eq!(
            vec![
                (BakerGridState::new(5, 7), 0.5),
                (BakerGridState::new(4, 7), 0.25),
                (BakerGridState::new(6, 7), 0.25),
            ],
            mdp.p_mass(&BakerGridState::new(5, 6), &East)
        );

        assert_eq!(
            mdp.p(
                &BakerGridState::new(5, 6),
                &East,
                &BakerGridState::new(5, 7)
            ),
            0.5,
        );

        assert_eq!(
            mdp.p(
                &BakerGridState::new(5, 6),
                &East,
                &BakerGridState::new(4, 7)
            ),
            0.25,
        );
    }
}
