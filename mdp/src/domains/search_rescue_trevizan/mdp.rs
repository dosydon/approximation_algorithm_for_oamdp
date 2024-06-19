use super::action::SRAction;
use super::cell_status::CellStatus;
use super::speed::Speed;
use super::state::SRState;
use crate::domains::search_rescue_trevizan::cell_status::CellStatus::*;
use crate::mdp_traits::*;
use core::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SRMDP<const N: usize> {
    initial_state: SRState<N>,
    goal_pos: (i32, i32),
    all_actions: Vec<SRAction>,
}

impl<const N: usize> SRMDP<N> {
    pub fn new(initial_state: SRState<N>, goal_pos: (i32, i32)) -> Self {
        Self {
            initial_state,
            goal_pos,
            all_actions: vec![
                SRAction::Up,
                SRAction::Down,
                SRAction::Left,
                SRAction::Right,
                SRAction::SpeedUp,
                SRAction::SpeedDown,
            ],
        }
    }

    fn within_bound(&self, i: i32, j: i32) -> bool {
        (0 <= i) && (i < N as i32) && (j >= 0) && (j < N as i32)
    }

    fn found_survivor(
        mut prev: [[CellStatus; N]; N],
        next_pos: (i32, i32),
        next_speed: Speed,
    ) -> SRState<N> {
        prev[next_pos.0 as usize][next_pos.1 as usize] = Survivor;
        SRState::new(prev, next_pos, true, next_speed)
    }

    fn not_found_survivor(
        mut prev: [[CellStatus; N]; N],
        next_pos: (i32, i32),
        boarded: bool,
        next_speed: Speed,
    ) -> SRState<N> {
        prev[next_pos.0 as usize][next_pos.1 as usize] = NoSurvivor;
        SRState::new(prev, next_pos, boarded, next_speed)
    }

    fn next_speed(&self, current: Speed, a: &SRAction) -> Speed {
        match current {
            Speed::Low => match a {
                SRAction::SpeedUp => Speed::Medium,
                _ => Speed::Low,
            },
            Speed::Medium => match a {
                SRAction::SpeedUp => Speed::High,
                SRAction::SpeedDown => Speed::Low,
                _ => Speed::Medium,
            },
            Speed::High => match a {
                SRAction::SpeedDown => Speed::Medium,
                _ => Speed::High,
            },
        }
    }
}

impl<const N: usize> StatesActions for SRMDP<N> {
    type State = SRState<N>;
    type Action = SRAction;
}

impl<const N: usize> IsTerminal for SRMDP<N> {
    fn is_terminal(&self, s: &Self::State) -> bool {
        s.boarded && s.configuration.agent_pos == self.goal_pos
    }
}

impl<const N: usize> ActionEnumerable for SRMDP<N> {
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

impl<const N: usize> ActionAvailability for SRMDP<N> {}

impl<const N: usize> InitialState for SRMDP<N> {
    fn initial_state(&self) -> Self::State {
        self.initial_state
    }
}

impl<const N: usize> PMass<f32> for SRMDP<N> {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        crate::mdp_traits::PMass::<f64>::p_mass(self, s, a)
            .into_iter()
            .map(|(s, p)| (s, p as f32))
            .collect()
    }
}

impl<const N: usize> PMass<f64> for SRMDP<N> {
    type Distribution = Vec<(Self::State, f64)>;
    fn p_mass(&self, s: &SRState<N>, a: &SRAction) -> Vec<(SRState<N>, f64)> {
        if self.is_terminal(s) {
            return vec![(*s, 1.0)];
        }
        let i = s.configuration.agent_pos.0 + a.di();
        let j = s.configuration.agent_pos.1 + a.dj();
        let next_pos = if self.within_bound(i, j) {
            (i, j)
        } else {
            s.configuration.agent_pos
        };
        let next_speed = self.next_speed(s.speed, a);
        match s.configuration.cells[next_pos.0 as usize][next_pos.1 as usize] {
            NoSurvivor => vec![(
                SRState::new(s.configuration.cells, next_pos, s.boarded, next_speed),
                1.0,
            )],
            Survivor => vec![(
                SRState::new(s.configuration.cells, next_pos, true, next_speed),
                1.0,
            )],
            ProbLow => vec![
                (
                    Self::found_survivor(s.configuration.cells, next_pos, next_speed),
                    0.05,
                ),
                (
                    Self::not_found_survivor(
                        s.configuration.cells,
                        next_pos,
                        s.boarded,
                        next_speed,
                    ),
                    0.95,
                ),
            ],
            ProbMedium => vec![
                (
                    Self::found_survivor(s.configuration.cells, next_pos, next_speed),
                    0.1,
                ),
                (
                    Self::not_found_survivor(
                        s.configuration.cells,
                        next_pos,
                        s.boarded,
                        next_speed,
                    ),
                    0.9,
                ),
            ],
            ProbHigh => vec![
                (
                    Self::found_survivor(s.configuration.cells, next_pos, next_speed),
                    0.2,
                ),
                (
                    Self::not_found_survivor(
                        s.configuration.cells,
                        next_pos,
                        s.boarded,
                        next_speed,
                    ),
                    0.8,
                ),
            ],
        }
    }
}

impl<const N: usize> PMassMutFrom<f32> for SRMDP<N> {}
impl<const N: usize> PMassMutFrom<f64> for SRMDP<N> {}
impl<const N: usize> GetNextStateFromPMass for SRMDP<N> {}
impl<const N: usize> GetNextStateMutFromImmut for SRMDP<N> {}

impl<const N: usize> ExplicitTransition for SRMDP<N> {}

impl<const N: usize> ExplicitTransitionMutFrom for SRMDP<N> {}

impl<const N: usize> Cost for SRMDP<N> {
    fn cost(&self, s: &Self::State, _a: &Self::Action) -> f32 {
        match s.speed {
            Speed::Low => 3.0,
            Speed::Medium => 2.0,
            Speed::High => 1.0,
        }
    }
}

impl<const N: usize> DCost for SRMDP<N> {
    fn d_cost(&self, s: &Self::State, _a: &Self::Action, _ss: &Self::State) -> f32 {
        match s.speed {
            Speed::Low => 3.0,
            Speed::Medium => 2.0,
            Speed::High => 1.0,
        }
    }
}

// impl<const N: usize> ShowProgress<SRState<N>> for SRMDP<N> {
//     fn show_progress(&self, _states: &HashSet<SRState<N>>) -> String {
//         "".to_string()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{
        mdp_traits::{InitialState, StateEnumerable},
        state_enumerable_wrapper::StateEnumerableWrapper,
        value_iteration::value_iteration_ssp,
    };

    use crate::domains::search_rescue_trevizan::instances::small;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_reserch_rescue_value_iteration() {
        let mdp = small();
        let wrapper = StateEnumerableWrapper::new(mdp);
        println!("{}", wrapper.num_states());

        let vt = value_iteration_ssp(&wrapper);
        assert_approx_eq!(vt.get_value(&wrapper.initial_state()), 8.636);

        //         let policy = TabularPolicy::from_value_table_ssp(&wrapper, &vt);
        //         let mut runner = CostEpisodeRunner::new(&wrapper.mdp, &policy, wrapper.mdp.initial_state());
        //         let mut rng = thread_rng();
        //         for (s, a, c) in runner.into_iter_with(&mut rng) {
        //             println!("{:?}", s);
        //         }
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mdp::heuristic::ZeroHeuristic;
//     use mdp::state_explicating_wrapper::StateExplicatingWrapper;
//     use mdp::value_iteration::value_iteration_ssp;
//     use assert_approx_eq::assert_approx_eq;
//     use good_lp::default_solver;
//     use crate::Idual;
//     use crate::cmap_good_lp;
//     use crate::retrieve_multi_costs::retrieve_multi_costs;
//     use crate::good_lp::{dual_good_lp, dual_good_lp_cmdp};
//     use crate::lp::dual_lp;
//     use crate::good_lp::solve_good_lp;
//     use crate::lp::solve_lp;
//     use crate::domains::search_rescue_trevizan::instances::{small, tiny};
//     use crate::h_min_heuristic_n::HminHeuristicN;
//
//     fn init() {
//         let _ = env_logger::builder().is_test(true).try_init();
//     }
//
//     #[test]
//     fn test_reserch_rescue_p_mass() {
//         let mdp = small();
//         assert_eq!(mdp.p_mass(&mdp.initial_state(), &SRAction::Right).len(), 2);
//     }
//
//     #[test]
//     fn test_dual_good_lp_cmdp_search_rescue_small() {
//         init();
//         let mut mdp = StateExplicatingWrapper::new(small());
//         println!("num states: {}", mdp.num_states());
//         let (lp, var_map, objective) = dual_good_lp_cmdp(&mut mdp, 0, [None, Some(20.0)], default_solver);
//         let result = solve_good_lp(lp,&var_map, &objective, &mdp, mdp.enumerate_states());
//         let costs = retrieve_multi_costs(&mdp, &result.1);
//         println!("{:?}", costs);
//     }
//
//     #[test]
//     fn test_dual_good_lp_search_rescue_tiny() {
//         init();
//         let mut mdp = StateExplicatingWrapper::new(tiny());
//         println!("num states: {}", mdp.num_states());
//         let (lp, var_map, objective) = dual_good_lp(&mut mdp, default_solver);
//         let result = solve_good_lp(lp,&var_map, &objective, &mdp, mdp.enumerate_states());
//         let costs = retrieve_multi_costs(&mdp, &result.1);
//         println!("{:?}", costs);
//     }
//
//     #[test]
//     fn test_dual_lp_search_rescue() {
//         init();
//         let mut mdp = StateExplicatingWrapper::new(tiny());
//         println!("num states: {}", mdp.num_states());
//         let lp = dual_lp(&mut mdp);
//         let result = solve_lp(&lp, &mdp, mdp.enumerate_states());
//         let costs = retrieve_multi_costs(&mdp, &result.1);
//         println!("{:?}", costs);
//     }
//
// //     #[test]
// //     fn test_idual_search_rescue_good_lp() {
// //         init();
// //         let mut mdp = small();
// //         let mut idual = Idual::new(mdp, HminHeuristicN::new2()).set_lp_solver(LpSolver::GoodLp);
// //         let costs = idual.idual(0, [None, Some(20.0)]);
// //         println!("{:?}", costs);
// //         println!("{:?}", idual.num_generated());
// //     }
//
//     #[test]
//     fn test_idual_search_rescue() {
//         init();
//         let mdp = small();
//         let mut idual = Idual::new(mdp, HminHeuristicN::new2());
//         let costs = idual.idual(0, [None, Some(20.0)]);
//         println!("{:?}", costs);
//         println!("{:?}", idual.num_generated());
//     }
//
// //     #[test]
// //     fn test_idual_lexicographic_search_rescue_good_lp() {
// //         init();
// //         let mut mdp = small();
// //         let mut idual = Idual::new(mdp, HminHeuristicN::new2()).set_lp_solver(LpSolver::GoodLp);
// //         let costs = idual.idual_lexicographic([1.0, 0.0]);
// //         println!("{:?}", costs);
// //         println!("{:?}", idual.num_generated());
// //     }
//
//     #[test]
//     fn test_idual_lexicographic_search_rescue() {
//         init();
//         let mdp = tiny();
//         let mut idual = Idual::new(mdp, ZeroHeuristic{});
//         let costs = idual.idual_lexicographic([1.0, 0.0]);
//         println!("{:?}", costs);
//         println!("{:?}", idual.num_generated());
//     }
//
//     #[test]
//     fn test_cmap_search_rescue() {
//         init();
//         let mut mdp = StateExplicatingWrapper::new(tiny());
//         let costs = cmap_good_lp(&mut mdp, [1.0, 0.0]);
//         println!("{:?}", costs);
//     }
// }
