use crate::grid_world::grid_status::GridStatus;
use crate::grid_world::GridWorldAction;
use crate::grid_world::GridWorldAction::*;
use crate::grid_world::GridWorldPartialMDP;
use crate::grid_world::GridWorldState;
use crate::mdp_traits::*;
use core::slice::Iter;
use itertools::iproduct;

#[derive(Debug, Clone)]
pub struct GridWorldMDP {
    pub h: i64,
    pub w: i64,
    pub(crate) initial_state: GridWorldState,
    pub(crate) all_states: Vec<GridWorldState>,
    pub(crate) all_actions: [GridWorldAction; 4],
    pub(crate) grids: Vec<Vec<GridStatus>>,
}

fn get_dx(a: &GridWorldAction) -> i64 {
    match a {
        AttemptLeft => -1,
        AttemptRight => 1,
        _ => 0,
    }
}

fn get_dy(a: &GridWorldAction) -> i64 {
    match a {
        AttemptUp => -1,
        AttemptDown => 1,
        _ => 0,
    }
}

impl GridWorldMDP {
    pub fn new(
        h: i64,
        w: i64,
        initial_state: GridWorldState,
        terminal_state: GridWorldState,
        watery_states: Vec<GridWorldState>,
        obstacled_states: Vec<GridWorldState>,
    ) -> GridWorldMDP {
        let mut grids = vec![];
        for y in 0..h {
            let mut row = vec![];
            for x in 0..w {
                let s = GridWorldState::new(x, y);
                if s == initial_state {
                    row.push(GridStatus::Start);
                } else if s == terminal_state {
                    row.push(GridStatus::Goal);
                } else if watery_states.contains(&s) {
                    row.push(GridStatus::Watery);
                } else if obstacled_states.contains(&s) {
                    row.push(GridStatus::Wall);
                } else {
                    row.push(GridStatus::Blank);
                }
            }
            grids.push(row);
        }
        //         println!("{:?}", grids);
        let mut grid = GridWorldMDP {
            h: h,
            w: w,
            initial_state: initial_state,
            all_states: vec![],
            all_actions: [AttemptUp, AttemptRight, AttemptDown, AttemptLeft],
            grids: grids,
        };
        grid.all_states = iproduct!((0..w), (0..h))
            .filter(|(x, y)| grid.is_valid_cordinate(*x, *y))
            .map(|(x, y)| GridWorldState { x, y })
            .collect();

        grid
    }
    fn deterministic_reward(&self, st: &GridWorldState, stt: &GridWorldState) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else if self.is_terminal(stt) {
            10.0
        } else if self.is_watery(stt) {
            -10.0
        } else {
            0.0
        }
    }
    fn within_bounds(&self, x: i64, y: i64) -> bool {
        if x < 0 || x >= self.w {
            false
        } else if y < 0 || y >= self.h {
            false
        } else {
            true
        }
    }
    fn get_status(&self, s: &GridWorldState) -> GridStatus {
        self.grids[s.y as usize][s.x as usize]
    }
    pub fn is_watery(&self, s: &GridWorldState) -> bool {
        self.get_status(s) == GridStatus::Watery
    }
    fn is_obstacled(&self, s: &GridWorldState) -> bool {
        self.get_status(s) == GridStatus::Wall
        //         self.obstacled_states.contains(s)
    }
    pub(crate) fn is_valid_cordinate(&self, x: i64, y: i64) -> bool {
        if !self.within_bounds(x, y) {
            false
        } else if self.is_obstacled(&GridWorldState::new(x, y)) {
            false
        } else {
            true
        }
    }
    fn success(&self, st: &GridWorldState, at: &GridWorldAction) -> GridWorldState {
        let dx = get_dx(at);
        let dy = get_dy(at);
        let new_x = st.x + dx;
        let new_y = st.y + dy;
        if self.is_valid_cordinate(new_x, new_y) {
            GridWorldState { x: new_x, y: new_y }
        } else {
            *st
        }
    }
    fn veer_right(&self, s: &GridWorldState, a: &GridWorldAction) -> GridWorldState {
        self.success(
            s,
            &(match a {
                AttemptUp => AttemptRight,
                AttemptRight => AttemptDown,
                AttemptDown => AttemptLeft,
                AttemptLeft => AttemptUp,
            }),
        )
    }
    fn veer_left(&self, s: &GridWorldState, a: &GridWorldAction) -> GridWorldState {
        self.success(
            s,
            &(match a {
                AttemptUp => AttemptLeft,
                AttemptRight => AttemptUp,
                AttemptDown => AttemptRight,
                AttemptLeft => AttemptDown,
            }),
        )
    }
}

impl ActionAvailability for GridWorldMDP {}

impl DCost for GridWorldMDP {
    fn d_cost(&self, st: &Self::State, _a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else if self.is_watery(stt) {
            10.0
        } else {
            1.0
        }
    }
}

impl StatesActions for GridWorldMDP {
    type State = GridWorldState;
    type Action = GridWorldAction;
}

impl IsTerminal for GridWorldMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        self.get_status(s) == GridStatus::Goal
    }
}

fn add_outcome(outcomes: &mut Vec<(GridWorldState, f32)>, outcome: GridWorldState, outcome_p: f32) {
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

impl GridWorldMDP {
    fn p_mass_non_terminal(
        &self,
        s: &GridWorldState,
        a: &GridWorldAction,
    ) -> Vec<(GridWorldState, f32)> {
        let mut res = Vec::new();
        add_outcome(&mut res, self.success(s, a), 0.8);
        add_outcome(&mut res, self.veer_left(s, a), 0.05);
        add_outcome(&mut res, self.veer_right(s, a), 0.05);
        add_outcome(&mut res, *s, 0.1);

        res
    }
}

impl InitialState for GridWorldMDP {
    fn initial_state(&self) -> Self::State {
        self.initial_state
    }
}

impl PMass<f32> for GridWorldMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) || self.is_obstacled(s) {
            vec![(*s, 1.0)]
        } else {
            self.p_mass_non_terminal(s, a)
        }
    }
}

impl PMass<f64> for GridWorldMDP {
    type Distribution = Vec<(Self::State, f64)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f64)> {
        if self.is_terminal(s) || self.is_obstacled(s) {
            vec![(*s, 1.0)]
        } else {
            self.p_mass_non_terminal(s, a)
                .into_iter()
                .map(|(s, p)| (s, p as f64))
                .collect()
        }
    }
}

impl GetNextStateFromPMass for GridWorldMDP {}
impl GetNextStateMutFromImmut for GridWorldMDP {}

impl PMassMutFrom<f32> for GridWorldMDP {}
impl PMassMutFrom<f64> for GridWorldMDP {}

impl ActionEnumerable for GridWorldMDP {
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

impl StateEnumerable for GridWorldMDP {
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

impl ExplicitTransition for GridWorldMDP {}

impl ExplicitTransitionMutFrom for GridWorldMDP {}

impl Rsas for GridWorldMDP {
    fn rsas(&self, st: &GridWorldState, _at: &GridWorldAction, stt: &GridWorldState) -> f32 {
        self.deterministic_reward(st, stt)
    }
}

impl Rsa for GridWorldMDP {
    fn rsa(&self, st: &Self::State, at: &Self::Action) -> f32 {
        PMass::<f32>::p_mass(self, st, at)
            .into_iter()
            .map(|(stt, prob)| prob * self.deterministic_reward(st, &stt))
            .sum()
    }
}

impl DiscountFactor for GridWorldMDP {
    fn get_discount_factor(&self) -> f32 {
        0.9
    }
}

impl CostFromDCost for GridWorldMDP {}

impl BuildFrom<GridWorldState, GridWorldMDP> for GridWorldPartialMDP {
    fn build_from(&self, goal: GridWorldState) -> GridWorldMDP {
        GridWorldMDP::new(
            self.h,
            self.w,
            self.initial_state,
            goal,
            self.watery_states.clone(),
            self.obstacled_states.clone(),
        )
    }
}

impl PreferredSuccessor for GridWorldMDP {
    fn preferred_successor(&self, s: &Self::State, a: &Self::Action) -> Self::State {
        if self.is_terminal(s) {
            *s
        } else {
            self.success(s, a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_grid_world_is_terminal() {
        let mdp = GridWorldMDP::default();
        assert_eq!(true, mdp.is_terminal(&GridWorldState { x: 4, y: 4 }));
        assert_eq!(false, mdp.is_terminal(&GridWorldState { x: 3, y: 4 }));
    }

    #[test]
    fn test_grid_world_is_valid_coordinate() {
        let mdp = GridWorldMDP::default();
        assert_eq!(true, mdp.is_valid_cordinate(0, 0));
    }

    #[test]
    fn test_grid_world_is_watery() {
        let mdp = GridWorldMDP::default();
        assert_eq!(true, mdp.is_watery(&GridWorldState::new(2, 4)));
        assert_eq!(false, mdp.is_watery(&GridWorldState::new(3, 4)));
    }

    #[test]
    fn test_grid_world_num_states() {
        let mdp = GridWorldMDP::default();
        assert_eq!(23, mdp.num_states());
    }

    #[test]
    fn test_grid_world_attempt_success() {
        let mdp = GridWorldMDP::default();
        let s = GridWorldState::new(0, 1);
        assert_eq!(GridWorldState::new(0, 0), mdp.success(&s, &AttemptUp));
        assert_eq!(GridWorldState::new(0, 2), mdp.success(&s, &AttemptDown));
        assert_eq!(
            GridWorldState::new(1, 0),
            mdp.success(&GridWorldState::new(0, 0), &AttemptRight)
        );
    }

    #[test]
    fn test_grid_world_attempt_veer_right() {
        let mdp = GridWorldMDP::default();
        assert_eq!(
            GridWorldState::new(1, 1),
            mdp.veer_right(&GridWorldState::new(0, 1), &AttemptUp)
        );
        assert_eq!(
            GridWorldState::new(0, 1),
            mdp.veer_right(&GridWorldState::new(0, 1), &AttemptDown)
        );
        assert_eq!(
            GridWorldState::new(2, 1),
            mdp.veer_right(&GridWorldState::new(1, 1), &AttemptUp)
        );
    }
    #[test]
    fn test_grid_world_p() {
        let err = 1e-3;
        let mdp = GridWorldMDP::default();
        assert_approx_eq!(
            0.15,
            mdp.p(
                &GridWorldState::new(0, 0),
                &AttemptRight,
                &GridWorldState::new(0, 0)
            ),
            err
        );
        assert_approx_eq!(
            0.8,
            mdp.p(
                &GridWorldState::new(0, 0),
                &AttemptRight,
                &GridWorldState::new(1, 0)
            ),
            err
        );
        assert_approx_eq!(
            0.05,
            mdp.p(
                &GridWorldState::new(0, 0),
                &AttemptRight,
                &GridWorldState::new(0, 1)
            ),
            err
        );

        assert_approx_eq!(
            0.05,
            mdp.p(
                &GridWorldState::new(2, 4),
                &AttemptUp,
                &GridWorldState::new(1, 4)
            ),
            err
        );
        assert_approx_eq!(
            0.9,
            mdp.p(
                &GridWorldState::new(2, 4),
                &AttemptUp,
                &GridWorldState::new(2, 4)
            ),
            err
        );
        assert_approx_eq!(
            0.05,
            mdp.p(
                &GridWorldState::new(2, 4),
                &AttemptUp,
                &GridWorldState::new(3, 4)
            ),
            err
        );
        assert_approx_eq!(
            0.8,
            mdp.p(
                &GridWorldState::new(2, 4),
                &AttemptRight,
                &GridWorldState::new(3, 4)
            ),
            err
        );
        assert_approx_eq!(
            0.2,
            mdp.p(
                &GridWorldState::new(2, 4),
                &AttemptRight,
                &GridWorldState::new(2, 4)
            ),
            err
        );
    }
    #[test]
    fn test_grid_world_rsa() {
        let err = 1e-3;
        let mdp = GridWorldMDP::default();
        assert_approx_eq!(-9.0, mdp.rsa(&GridWorldState::new(2, 4), &AttemptUp), err);
    }
}
