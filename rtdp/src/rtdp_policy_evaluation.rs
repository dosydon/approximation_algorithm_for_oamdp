use core::fmt::Debug;
use core::hash::Hash;
use mdp::heuristic::HeuristicWithMDPMut;
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, GetNextStateMut, InitialState, IsTerminal,
    PMassMut, StatesActions,
};
use mdp::policy::policy_traits::{GetAction, GetActionMut, GetActionProbabilityMut};

use mdp::state_queue::StateQueue;
use mdp::value_estimator::CostEstimatorMut;
use rand::prelude::*;

use crate::rtdp::RTDP;

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
    pub fn update_with_policy<M, P>(&mut self, s: &M::State, mdp: &mut M, policy: &mut P) -> f32
    where
        M: StatesActions<State = S> + PMassMut<f32> + Cost + ActionAvailability + ActionEnumerable,
        H: HeuristicWithMDPMut<M>,
        P: GetActionProbabilityMut<M::Action, M>,
    {
        let mut new_value = 0.0;
        for a_id in 0..mdp.num_actions() {
            let a = *mdp.id_to_action(a_id);
            if mdp.action_available(s, &a) {
                let qsa = self.get_qsa_ssp_mut(s, &a, mdp);
                new_value += policy.get_action_probability_mut(s, &a, mdp) * qsa;
            }
        }
        let residual = (self.get_value_ssp_mut(s, mdp) - new_value).abs();
        self.vt.set_value(s, new_value);

        residual
    }

    pub(crate) fn trial_with_policy<M, P>(
        &mut self,
        mdp: &mut M,
        policy: &mut P,
        rng: &mut ThreadRng,
    ) -> f32
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
        P: GetActionMut<M::State, M> + GetActionProbabilityMut<M::Action, M>,
    {
        let mut current_state = mdp.initial_state();
        let mut time_step = 0;
        let mut max_residual = 0.0;

        while !mdp.is_terminal(&current_state) && time_step < self.max_t {
            time_step += 1;
            let a = policy.get_action_mut(&current_state, mdp, rng).unwrap();
            let residual = self.update_with_policy(&current_state, mdp, policy);
            if residual > max_residual {
                max_residual = residual;
            }

            current_state = mdp.get_next_state_mut(&current_state, &a, rng);
        }

        max_residual
    }

    pub fn policy_evaluation<M, P>(&mut self, mdp: &mut M, policy: &mut P, rng: &mut ThreadRng)
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
        P: GetActionProbabilityMut<M::Action, M> + GetAction<M::State, M>,
    {
        loop {
            self.trial(mdp, rng);
            if self.check_solved_policy(&mdp.initial_state(), mdp, policy, 1e-3) {
                break;
            }
        }
    }
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
    pub fn check_solved_policy<M, P>(
        &mut self,
        s: &M::State,
        mdp: &mut M,
        policy: &mut P,
        epsilon: f32,
    ) -> bool
    where
        M: InitialState
            + StatesActions<State = S>
            + PMassMut<f32>
            + Cost
            + ActionEnumerable
            + ActionAvailability,
        H: HeuristicWithMDPMut<M>,
        P: GetActionProbabilityMut<M::Action, M> + GetAction<M::State, M>,
    {
        let mut rv = true;
        let mut open = StateQueue::new();
        let mut closed = StateQueue::new();

        open.push(*s);

        while open.len() > 0 {
            if let Some(ss) = open.pop() {
                closed.push(ss);

                let mut new_value = 0.0;
                for a_id in 0..mdp.num_actions() {
                    let a = *mdp.id_to_action(a_id);
                    if mdp.action_available(s, &a) {
                        let qsa = self.get_qsa_ssp_mut(s, &a, mdp);
                        new_value += policy.get_action_probability_mut(s, &a, mdp) * qsa;
                    }
                }
                let residual = (self.get_value_ssp_mut(s, mdp) - new_value).abs();

                if residual > epsilon {
                    rv = false;
                    continue;
                }

                for a_id in 0..mdp.num_actions() {
                    let a = *mdp.id_to_action(a_id);
                    if mdp.action_available(s, &a) {
                        if policy.get_action_probability_mut(s, &a, mdp) > 0.0 {
                            for (sss, _p) in mdp.p_mass_mut(&ss, &a) {
                                if !closed.contains(&sss) && !open.contains(&sss) {
                                    open.push(sss);
                                }
                            }
                        }
                    }
                }
            }
        }

        rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mdp::grid_world::{GridWorldMDP, GridWorldState};
    use mdp::heuristic::ZeroHeuristic;
    use mdp::policy::tabular_policy::TabularPolicy;
    use mdp::value_iteration::value_iteration_ssp;
    use rand::thread_rng;

    #[test]
    fn test_policy_evaluation() {
        let mut mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let vt = value_iteration_ssp(&mdp);
        println!("{:?}", vt.get_value(&mdp.initial_state()));
        let mut policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        let mut rng = thread_rng();
        let mut rtdp = RTDP::new(ZeroHeuristic {});
        rtdp.policy_evaluation(&mut mdp, &mut policy, &mut rng);
        println!("{:?}", rtdp.vt.get_value(&mdp.initial_state()));
        //         assert_eq!(
        //             rtdp.check_solved_mut(&mdp.initial_state(), &mut mdp, err),
        //             false
        //         );
        //         assert_eq!(
        //             rtdp.check_solved(&mdp.initial_state(), &mut mdp, err),
        //             false
        //         );
        //
        //         rtdp.solve(&mut mdp, &mut rng, 50000);
        //         assert_eq!(
        //             rtdp.check_solved_mut(&mdp.initial_state(), &mut mdp, err),
        //             true
        //         );
        //         assert_eq!(rtdp.check_solved(&mdp.initial_state(), &mut mdp, err), true);
    }
}
