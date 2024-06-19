use super::rtdp::RTDP;
use core::fmt::Debug;
use core::hash::Hash;
use mdp::heuristic::HeuristicWithMDPMut;
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, InitialState, IsTerminal, PMassMut, StatesActions,
};
use mdp::state_queue::StateQueue;

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
    pub fn check_solved<M>(&mut self, s: &M::State, mdp: &mut M, epsilon: f32) -> bool
    where
        M: InitialState
            + StatesActions<State = S>
            + PMassMut<f32>
            + Cost
            + ActionEnumerable
            + ActionAvailability,
        H: HeuristicWithMDPMut<M>,
    {
        let mut rv = true;
        let mut open = StateQueue::new();
        let mut closed = StateQueue::new();

        open.push(*s);

        while open.len() > 0 {
            if let Some(ss) = open.pop() {
                closed.push(ss);

                let a = self.best_action_mut(&ss, mdp).unwrap();
                let qsa = self.get_qsa_ssp_mut(&ss, &a, mdp);
                let residual = (qsa - self.get_value_ssp_mut(&ss, mdp)).abs();

                if residual > epsilon {
                    rv = false;
                    continue;
                }

                for (sss, _p) in mdp.p_mass_mut(&ss, &a) {
                    if !closed.contains(&sss) && !open.contains(&sss) {
                        open.push(sss);
                    }
                }
            }
        }

        rv
    }

    pub fn check_solved_mut<M>(&mut self, s: &M::State, mdp: &mut M, epsilon: f32) -> bool
    where
        M: StatesActions<State = S>
            + PMassMut<f32>
            + Cost
            + ActionEnumerable
            + ActionAvailability
            + IsTerminal,
        H: HeuristicWithMDPMut<M>,
    {
        let mut rv = true;
        let mut open = StateQueue::new();
        let mut closed = StateQueue::new();

        if !self.is_solved.contains(s) {
            open.push(*s);
        }

        if !mdp.is_terminal(s) {
            open.push(*s);
        }

        while open.len() > 0 {
            if let Some(ss) = open.pop() {
                closed.push(ss);

                let a = self.best_action_mut(&ss, mdp).unwrap();
                let qsa = self.get_qsa_ssp_mut(&ss, &a, mdp);
                let residual = (qsa - self.get_value_ssp_mut(&ss, mdp)).abs();
                //                 println!("{:?} {:?} {:?}", a, qsa, residual);

                if residual > epsilon {
                    //                     println!("{:?} > {:?}", residual, epsilon);
                    rv = false;
                    continue;
                }

                for (sss, _p) in mdp.p_mass_mut(&ss, &a) {
                    if !self.is_solved.contains(&sss)
                        && !closed.contains(&sss)
                        && !open.contains(&sss)
                    {
                        open.push(sss);
                    }
                }
            }
        }

        if rv {
            for ss in closed {
                self.is_solved.insert(ss);
                //                 println!("{:?}", ss);
            }
        } else {
            while closed.len() > 0 {
                if let Some(ss) = closed.pop() {
                    let a = self.best_action_mut(&ss, mdp).unwrap();
                    let qsa = self.get_qsa_ssp_mut(&ss, &a, mdp);
                    self.vt.set_value(&ss, qsa);
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
    use rand::thread_rng;

    #[test]
    fn test_check_solved() {
        let mut mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let mut rng = thread_rng();
        let err = 1e-1;
        let mut rtdp = RTDP::new(ZeroHeuristic {});
        rtdp.solve(&mut mdp, &mut rng, 100);
        assert_eq!(
            rtdp.check_solved_mut(&mdp.initial_state(), &mut mdp, err),
            false
        );
        assert_eq!(
            rtdp.check_solved(&mdp.initial_state(), &mut mdp, err),
            false
        );

        rtdp.solve(&mut mdp, &mut rng, 50000);
        assert_eq!(
            rtdp.check_solved_mut(&mdp.initial_state(), &mut mdp, err),
            true
        );
        assert_eq!(rtdp.check_solved(&mdp.initial_state(), &mut mdp, err), true);
    }
}
