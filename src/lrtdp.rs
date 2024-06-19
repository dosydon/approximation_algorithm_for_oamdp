use super::rtdp::RTDP;
use core::fmt::Debug;
use core::hash::Hash;
use mdp::heuristic::HeuristicWithMDPMut;
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, GetNextStateMut, InitialState, IsTerminal,
    PMassMut, StatesActions,
};
use mdp::value_estimator::CostEstimatorMut;
use rand::prelude::*;

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
    pub fn lrtdp<M>(&mut self, mdp: &mut M, num_trials: usize, rng: &mut ThreadRng, epsilon: f32)
    where
        M: InitialState
            + StatesActions<State = S>
            + PMassMut<f32>
            + IsTerminal
            + Cost
            + GetNextStateMut
            + ActionEnumerable
            + ActionAvailability,
        H: HeuristicWithMDPMut<M>,
    {
        self.lrtdp_inner(mdp.initial_state(), mdp, num_trials, rng, epsilon)
    }

    pub fn lrtdp_inner<M>(
        &mut self,
        s: M::State,
        mdp: &mut M,
        num_trials: usize,
        rng: &mut ThreadRng,
        epsilon: f32,
    ) where
        M: StatesActions<State = S>
            + PMassMut<f32>
            + IsTerminal
            + Cost
            + GetNextStateMut
            + ActionEnumerable
            + ActionAvailability,
        H: HeuristicWithMDPMut<M>,
    {
        let mut trial = 0;
        while !self.is_solved.contains(&s) && (num_trials <= 0 || trial < num_trials) {
            let mut visited = Vec::<M::State>::new();
            let mut current_state = s;

            while !mdp.is_terminal(&current_state) && !self.is_solved.contains(&current_state) {
                visited.push(current_state);

                let a = self.best_action_mut(&current_state, mdp).unwrap();
                let qsa = self.get_qsa_ssp_mut(&current_state, &a, mdp);
                //             let residual = (qsa - value_table.get_value(&current_state)).abs();
                self.vt.set_value(&current_state, qsa);

                current_state = mdp.get_next_state_mut(&current_state, &a, rng);
            }

            while visited.len() > 0 {
                if let Some(ss) = visited.pop() {
                    if !self.check_solved_lrtdp(&ss, mdp, epsilon) {
                        break;
                    }
                }
            }

            trial += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;
    use mdp::blocks_world::BlocksWorldPartialMDP;
    use mdp::blocks_world::Location::*;
    use mdp::blocks_world::*;
    use mdp::grid_world::{GridWorldMDP, GridWorldState};
    use mdp::heuristic::HminHeuristic;
    use mdp::mdp_traits::BuildFrom;
    use mdp::mdp_traits::StateEnumerable;
    use mdp::state_enumerable_wrapper::StateEnumerableWrapper;
    use mdp::value_iteration::value_iteration_ssp;

    #[test]
    fn test_grid_world_lrtdp() {
        let mut mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let mut rng = thread_rng();
        let err = 1e-3;
        let mut lrtdp = RTDP::new(HminHeuristic::new());
        lrtdp.lrtdp(&mut mdp, 0, &mut rng, err);
        let vt = value_iteration_ssp(&mdp);

        assert_approx_eq!(
            vt.get_value(&mdp.initial_state()),
            lrtdp.vt.get_value(&mdp.initial_state()),
            1e-1
        );
    }

    #[test]
    fn test_blocks_world_lrtdp() {
        let partial_mdp = BlocksWorldPartialMDP::new(
            [OnTable, OnTable, On(Block::new(1)), OnTable],
            0.1,
            ['A', 'M', 'S', 'R'],
        );
        let goal = [
            On(Block::new(3)),
            On(Block::new(2)),
            OnTable,
            On(Block::new(1)),
        ];
        let mut mdp = partial_mdp.build_from(&goal);
        let mut rng = thread_rng();
        let err = 1e-3;
        let mut lrtdp = RTDP::new(HminHeuristic::new());
        lrtdp.lrtdp(&mut mdp, 0, &mut rng, err);

        let mdp = StateEnumerableWrapper::new(mdp);
        println!("{:?}", mdp.num_states());
        let vt = value_iteration_ssp(&mdp);

        assert_approx_eq!(
            vt.get_value(&mdp.initial_state()),
            lrtdp.vt.get_value(&mdp.initial_state()),
            1e-1
        );
    }
}
