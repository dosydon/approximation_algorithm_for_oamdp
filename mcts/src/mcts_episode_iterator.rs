use std::time::{Duration, Instant};

use log::{debug, info};
use mdp::{
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, GetNextStateMut, InitialState,
        IntoEvalMut, IsTerminal, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};

use crate::{Budget, MCTS};

pub struct MCTSEpisodeIterator<'a, M: StatesActions, P> {
    mcts: &'a mut MCTS<M, P>,
    node_id: usize,
    budget: Budget,
    rng: &'a mut rand::rngs::ThreadRng,
}

impl<'a, M: StatesActions + InitialState, P> MCTSEpisodeIterator<'a, M, P> {
    pub fn from_initial_state(
        mcts: &'a mut MCTS<M, P>,
        budget: Budget,
        rng: &'a mut rand::rngs::ThreadRng,
    ) -> Self {
        MCTSEpisodeIterator {
            mcts,
            node_id: 0,
            budget: budget,
            rng,
        }
    }
}

impl<
        'a,
        M: StatesActions
            + IsTerminal
            + ActionAvailability
            + GetNextStateMut
            + Cost
            + DCost
            + ActionEnumerable
            + InitialState,
        P: IntoEvalMut<M> + GetActionMut<M::State, M>,
    > Iterator for MCTSEpisodeIterator<'a, M, P>
{
    type Item = (M::State, M::Action, M::State, f32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let s = self.mcts.arena.get_node(self.node_id).assoc;
            info!("state: {:?}", s);

            if self.mcts.mdp.is_terminal(&s) {
                return None;
            }

            let start_time = Instant::now();
            let mut num_iterations = 0;
            loop {
                match self.budget {
                    Budget::NumIterations(max_iter) => {
                        if num_iterations >= max_iter {
                            break;
                        }
                    }
                    Budget::TimeBudget(max_time) => {
                        if Instant::now() - start_time >= Duration::from_secs_f32(max_time) {
                            break;
                        }
                    }
                }
                num_iterations += 1;
                self.mcts.expand_recursive(self.node_id, &mut self.rng);
            }

            debug!("num iterations: {:?}", num_iterations);

            let and_node = self
                .mcts
                .arena
                .get_node(self.node_id)
                .best_and_node_greedy();

            let a = if let Some(and_node_inner) = and_node {
                and_node_inner.a
            } else {
                //                 panic!("no action found")
                self.mcts
                    .base_line_policy
                    .get_action_mut(&s, &mut self.mcts.mdp, &mut self.rng)
                    .unwrap()
            };
            info!("action: {:?}", a);

            let ss = self.mcts.mdp.get_next_state_mut(&s, &a, &mut self.rng);
            info!("ss: {:?}", ss);
            if let Some(and_node_inner) = and_node {
                if let Some(ss_id) = and_node_inner.find_s(&ss, &self.mcts.arena) {
                    self.node_id = ss_id;
                } else {
                    self.node_id = self.mcts.add_node(ss);
                }
            } else {
                self.node_id = self.mcts.add_node(ss);
            }

            let c = self.mcts.mdp.d_cost(&s, &a, &ss);
            debug!("cost: {}", c);

            return Some((s, a, ss, c));
        }
    }
}
