use core::fmt::Debug;
use core::hash::Hash;
use mdp::heuristic::{HeuristicWithMDP, HeuristicWithMDPMut};
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, GetNextStateMut, InitialState, IsTerminal, PMass,
    PMassMut, StatesActions,
};
use mdp::policy::policy_traits::{GetAction, GetActionMut};

use mdp::value_estimator::{CostEstimator, CostEstimatorMut};
use mdp::value_iteration::ValueTable;
use rand::prelude::*;
use std::collections::HashSet;

pub struct RTDP<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> {
    pub vt: ValueTable<S>,
    pub h: H,
    pub is_solved: HashSet<S>,
    pub max_t: usize,
}

impl<M: StatesActions + PMass<f32> + Cost, H: HeuristicWithMDP<M>> CostEstimator<M>
    for RTDP<M::State, H>
{
    fn get_value_ssp(&self, s: &M::State, mdp: &M) -> f32 {
        self.vt.get_value(s).max(self.h.h_with(s, mdp))
    }

    fn get_qsa_ssp(&self, s: &M::State, a: &M::Action, mdp: &M) -> f32 {
        mdp.p_mass(s, a)
            .into_iter()
            .map(|(ss, p)| self.get_value_ssp(&ss, mdp) * p)
            .sum::<f32>()
            + mdp.cost(s, a)
    }
}

impl<M: StatesActions + PMassMut<f32> + Cost, H: HeuristicWithMDPMut<M>> CostEstimatorMut<M>
    for RTDP<M::State, H>
{
    fn get_value_ssp_mut(&mut self, s: &M::State, mdp: &mut M) -> f32 {
        self.vt.get_value(s).max(self.h.h_with_mut(s, mdp))
    }

    fn get_qsa_ssp_mut(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32 {
        mdp.p_mass_mut(s, a)
            .into_iter()
            .map(|(ss, p)| self.get_value_ssp_mut(&ss, mdp) * p)
            .sum::<f32>()
            + mdp.cost(s, a)
    }
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
    pub fn update<M>(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32
    where
        M: StatesActions<State = S> + PMassMut<f32> + Cost,
        H: HeuristicWithMDPMut<M>,
    {
        let qsa = self.get_qsa_ssp_mut(&s, &a, mdp);
        let value = self.get_value_ssp_mut(s, mdp);
        //         println!("s: {:?}, a: {:?}, qsa: {}, value: {}", s, a, qsa, value);
        let residual = (value - qsa).abs();
        self.vt.set_value(s, qsa);

        residual
    }
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, M, H> GetAction<S, M> for RTDP<S, H>
where
    M: PMass<f32> + Cost + ActionEnumerable + ActionAvailability + StatesActions<State = S>,
    H: HeuristicWithMDP<M>,
{
    fn get_action(&self, s: &S, mdp: &M, _rng: &mut ThreadRng) -> Option<M::Action> {
        self.best_action(s, mdp)
    }
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, M, H> GetActionMut<S, M> for RTDP<S, H>
where
    M: PMassMut<f32> + Cost + ActionEnumerable + ActionAvailability + StatesActions<State = S>,
    H: HeuristicWithMDPMut<M>,
{
    fn get_action_mut(&mut self, s: &S, mdp: &mut M, _rng: &mut ThreadRng) -> Option<M::Action> {
        self.best_action_mut(s, mdp)
    }
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
    pub fn best_action<M>(&self, s: &M::State, mdp: &M) -> Option<M::Action>
    where
        M: PMass<f32> + Cost + ActionEnumerable + ActionAvailability + StatesActions<State = S>,
        H: HeuristicWithMDP<M>,
    {
        let mut action = None;
        let mut current_best = 1e+8;
        for a_id in 0..mdp.num_actions() {
            let a = *mdp.id_to_action(a_id);
            if mdp.action_available(s, &a) {
                if current_best >= self.get_qsa_ssp(s, &a, mdp) {
                    current_best = self.get_qsa_ssp(s, &a, mdp);
                    action = Some(a);
                }
            }
        }

        return action;
    }
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
    pub fn best_action_mut<M>(&mut self, s: &M::State, mdp: &mut M) -> Option<M::Action>
    where
        M: PMassMut<f32> + Cost + ActionEnumerable + ActionAvailability + StatesActions<State = S>,
        H: HeuristicWithMDPMut<M>,
    {
        let mut action = None;
        let mut current_best = 1e+8;
        for a_id in 0..mdp.num_actions() {
            let a = *mdp.id_to_action(a_id);
            if mdp.action_available(s, &a) {
                if current_best >= self.get_qsa_ssp_mut(s, &a, mdp) {
                    current_best = self.get_qsa_ssp_mut(s, &a, mdp);
                    action = Some(a);
                }
            }
        }

        return action;
    }
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
    pub fn new(h: H) -> RTDP<S, H> {
        RTDP {
            vt: ValueTable::new(0.0),
            h: h,
            is_solved: HashSet::new(),
            max_t: 1000,
        }
    }

    pub fn num_states(&self) -> usize {
        self.vt.value_table.len()
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
    {
        let mut current_state = mdp.initial_state();
        let mut time_step = 0;
        let mut max_residual = 0.0;

        while !mdp.is_terminal(&current_state) && time_step < self.max_t {
            time_step += 1;
            let a = self.best_action_mut(&current_state, mdp).unwrap();
            let residual = self.update(&current_state, &a, mdp);
            if residual > max_residual {
                max_residual = residual;
            }

            current_state = mdp.get_next_state_mut(&current_state, &a, rng);
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
    {
        if num_trials > 0 {
            for _ in 0..num_trials {
                self.trial(mdp, rng);
            }
        } else {
            loop {
                self.trial(mdp, rng);
                if self.check_solved(&mdp.initial_state(), mdp, 1e-3) {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use mdp::grid_world::{GridWorldMDP, GridWorldState};
    use mdp::heuristic::ZeroHeuristic;
    use mdp::value_iteration::value_iteration_ssp;

    #[test]
    fn test_grid_world_rtdp() {
        let mut mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let mut rng = thread_rng();
        let vt = value_iteration_ssp(&mdp);

        let mut rtdp = RTDP::new(ZeroHeuristic {});
        rtdp.solve(&mut mdp, &mut rng, 5000);
        assert_approx_eq!(
            vt.get_value(&mdp.initial_state()),
            rtdp.vt.get_value(&mdp.initial_state()),
            1e-3
        );
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
