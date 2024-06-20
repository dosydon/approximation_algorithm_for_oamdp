use core::fmt::Debug;
use core::hash::Hash;
use mdp::heuristic::{HeuristicWithMDP, HeuristicWithMDPMut};
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, GetNextStateMut, InitialState, IsTerminal, PMass,
    PMassMut, State, StatesActions,
};
use mdp::policy::policy_traits::{GetAction, GetActionMut};

use mdp::value_estimator::CostEstimator;
use mdp::value_iteration::ValueTable;
use num_traits::FromPrimitive;
use ordered_float::NotNan;
use rand::prelude::*;
use std::collections::{HashSet, VecDeque};

use crate::action_selection::ActionSelection;
use crate::traits::UpperBoundWithMDPMut;

pub struct BRTDP<S: State, H, U> {
    pub lb: ValueTable<S>,
    pub ub: ValueTable<S>,
    pub h: H,
    pub u: U,
    pub is_solved: HashSet<S>,
    pub max_t: usize,
    tau: f32,
}

impl<S: State, H, U> BRTDP<S, H, U> {
    pub fn best_action_mut<M>(
        &mut self,
        s: &M::State,
        mdp: &mut M,
        action_selection: ActionSelection,
    ) -> Option<M::Action>
    where
        M: PMassMut<f32> + Cost + ActionEnumerable + ActionAvailability + StatesActions<State = S>,
        H: HeuristicWithMDPMut<M>,
        U: UpperBoundWithMDPMut<M>,
    {
        let mut action = None;
        let mut current_best = 1e+8;
        for a_id in 0..mdp.num_actions() {
            let a = *mdp.id_to_action(a_id);
            if mdp.action_available(s, &a) {
                let qsa = match action_selection {
                    ActionSelection::LB => self.get_lb_qsa_mut(s, &a, mdp),
                    ActionSelection::UB => self.get_ub_qsa_mut(s, &a, mdp),
                };
                if current_best >= qsa {
                    current_best = qsa;
                    action = Some(a);
                }
            }
        }

        return action;
    }
}

impl<S: State, H, U> BRTDP<S, H, U> {
    pub fn new(h: H, u: U) -> BRTDP<S, H, U> {
        BRTDP {
            lb: ValueTable::new(0.0),
            ub: ValueTable::new(100.0),
            h: h,
            u: u,
            is_solved: HashSet::new(),
            max_t: 1000,
            tau: 100.0,
        }
    }

    pub(crate) fn sample_next_state(
        &self,
        next_states_with_uncertainties: Vec<(S, f32)>,
        sum_next_state_uncertainties: f32,
        rng: &mut ThreadRng,
    ) -> S {
        //         println!("{:?}", next_states_with_uncertainties);
        next_states_with_uncertainties
            .choose_weighted(rng, |item| item.1 / sum_next_state_uncertainties)
            .unwrap()
            .0
    }

    pub(crate) fn next_states_with_uncertainties<M>(
        &mut self,
        s: &M::State,
        a: &M::Action,
        mdp: &mut M,
    ) -> Vec<(M::State, f32)>
    where
        M: StatesActions<State = S> + PMassMut<f32> + Cost + IsTerminal + GetNextStateMut,
        H: HeuristicWithMDPMut<M>,
        U: UpperBoundWithMDPMut<M>,
    {
        mdp.p_mass_mut(s, a)
            .into_iter()
            .map(|(ss, p)| {
                let gap = self.get_ub_mut(&ss, mdp) - self.get_lb_mut(&ss, mdp);
                (ss, p * gap)
            })
            .collect()
    }

    pub(crate) fn trial<M>(&mut self, mdp: &mut M, rng: &mut ThreadRng) -> f32
    where
        M: InitialState
            + StatesActions<State = S>
            + PMassMut<f32>
            + Cost
            + ActionEnumerable
            + IsTerminal
            + GetNextStateMut
            + ActionAvailability,
        H: HeuristicWithMDPMut<M>,
        U: UpperBoundWithMDPMut<M>,
    {
        let mut current_state = mdp.initial_state();
        let mut time_step = 0;
        let mut max_residual = 0.0;
        let mut visited = VecDeque::new();

        while !mdp.is_terminal(&current_state) && time_step < self.max_t {
            //             println!("{:?}", current_state);
            time_step += 1;
            visited.push_front(current_state);
            let a = self
                .best_action_mut(&current_state, mdp, ActionSelection::LB)
                .unwrap();

            let residual = self.update_ub(&current_state, mdp);
            if residual > max_residual {
                max_residual = residual;
            }

            let residual = self.update_lb(&current_state, &a, mdp);
            if residual > max_residual {
                max_residual = residual;
            }

            let next_state_uncertainties =
                self.next_states_with_uncertainties(&current_state, &a, mdp);
            let sum_next_state_uncertainties =
                next_state_uncertainties.iter().map(|(_, b)| b).sum::<f32>();

            if sum_next_state_uncertainties
                < ((self.get_ub_mut(&current_state, mdp) - self.get_lb_mut(&current_state, mdp))
                    .abs()
                    / self.tau)
            {
                break;
            }

            current_state =
                self.sample_next_state(next_state_uncertainties, sum_next_state_uncertainties, rng);
        }

        while let Some(s) = visited.pop_front() {
            let residual = self.update_ub(&s, mdp);
            if residual > max_residual {
                max_residual = residual;
            }

            let a = self.best_action_mut(&s, mdp, ActionSelection::LB).unwrap();
            let residual = self.update_lb(&s, &a, mdp);
            if residual > max_residual {
                max_residual = residual;
            }
        }

        max_residual
    }

    pub fn solve<M>(&mut self, mdp: &mut M, rng: &mut ThreadRng, num_trials: usize)
    where
        M: InitialState
            + StatesActions<State = S>
            + PMassMut<f32>
            + Cost
            + IsTerminal
            + GetNextStateMut
            + ActionEnumerable
            + ActionAvailability,
        H: HeuristicWithMDPMut<M>,
        U: UpperBoundWithMDPMut<M>,
    {
        let initial_state = mdp.initial_state();
        for k in 0..num_trials {
            let gap = self.get_ub_mut(&initial_state, mdp) - self.get_lb_mut(&initial_state, mdp);
            if gap < 1e-3 {
                break;
            }
            self.trial(mdp, rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constant_upper_bound::ConstantUpperBound;

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use mdp::grid_world::{GridWorldMDP, GridWorldState};
    use mdp::heuristic::ZeroHeuristic;
    use mdp::value_iteration::value_iteration_ssp;

    #[test]
    fn test_grid_world_brtdp() {
        println!("hello world");
        let mut mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let vt = value_iteration_ssp(&mdp);

        let mut rng = thread_rng();
        let mut brtdp = BRTDP::new(ZeroHeuristic {}, ConstantUpperBound::new(100.0));
        brtdp.solve(&mut mdp, &mut rng, 1000);
        //         assert_approx_eq!(
        //             vt.get_value(&mdp.initial_state()),
        //             brtdp.lb.get_value(&mdp.initial_state()),
        //             1e-3
        //         );
    }

    //     #[test]
    //     fn test_grid_world_rtdp_hmin() {
    //         let mut mdp = GridWorldMDP::new(
    //             4,
    //             4,
    //             GridWorldState::new(0, 0),
    //             GridWorldState::new(3, 3),
    //             vec![GridWorldState::new(2, 3)],
    //             vec![],
    //         );
    //         let mut rng = thread_rng();
    //         let vt = value_iteration_ssp(&mdp);
    //
    //         let mut rtdp = RTDP::new(HminHeuristic::new());
    //         rtdp.solve(&mut mdp, &mut rng, 5000);
    //         assert_approx_eq!(
    //             vt.get_value(&mdp.initial_state()),
    //             rtdp.vt.get_value(&mdp.initial_state()),
    //             1e-3
    //         );
    //     }
}
