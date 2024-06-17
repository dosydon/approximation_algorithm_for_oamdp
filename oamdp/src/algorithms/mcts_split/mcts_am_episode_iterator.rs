use std::time::{Duration, Instant};

use log::{debug, info};
use mcts::Budget;
use mdp::{
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, GetNextStateMut, InitialState,
        IntoEvalMut, IsTerminal, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};

use crate::traits::{DomainAction, EnumerateDomainAction, EnumerateMessage, Message, Set};

use super::mcts_am::MCTSAM;

pub struct MCTSAMEpisodeIterator<'a, M: StatesActions + DomainAction + Message, P> {
    mcts: &'a mut MCTSAM<M, P>,
    node_id: usize,
    rng: &'a mut rand::rngs::ThreadRng,
}

impl<'a, M: StatesActions + InitialState + DomainAction + Message, P>
    MCTSAMEpisodeIterator<'a, M, P>
{
    pub fn from_initial_state(
        mcts: &'a mut MCTSAM<M, P>,
        rng: &'a mut rand::rngs::ThreadRng,
    ) -> Self {
        MCTSAMEpisodeIterator {
            mcts,
            node_id: 0,
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
            + DomainAction
            + Message
            + EnumerateDomainAction
            + EnumerateMessage
            + Cost
            + DCost
            + ActionEnumerable
            + InitialState,
        P: GetActionMut<M::State, M> + IntoEvalMut<M>,
    > Iterator for MCTSAMEpisodeIterator<'a, M, P>
where
    M::Action: From<(M::DomainAction, M::Message)> + Set<M::DomainAction>,
{
    type Item = (M::State, M::Action, M::State, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.mcts.arena.get_node(self.node_id).assoc;
        info!("state: {:?}", s);

        if self.mcts.mdp.is_terminal(&s) {
            return None;
        }

        let start_time = Instant::now();
        let mut num_iterations = 0;
        loop {
            match self.mcts.budget {
                Budget::TimeBudget(max_time) => {
                    if Instant::now() - start_time >= Duration::from_secs_f32(max_time) {
                        break;
                    }
                }
                Budget::NumIterations(max_iteration) => {
                    if num_iterations >= max_iteration {
                        break;
                    }
                }
            }
            num_iterations += 1;
            self.mcts.expand_recursive_state(self.node_id, self.rng);
        }
        debug!("num iterations: {:?}", num_iterations);

        let pair = self
            .mcts
            .arena
            .get_node(self.node_id)
            .best_intermediate_node_greedy_id(self.mcts.c);

        let a = if let Some((a_id, m_id)) = pair {
            let a = self.mcts.arena.get_node(self.node_id).children[a_id].assoc;
            let m = self.mcts.arena.get_node(self.node_id).children[a_id].children[m_id].a;
            M::Action::from((a, m))
        } else {
            self.mcts
                .base_line_policy
                .get_action_mut(&s, &mut self.mcts.mdp, self.rng)
                .unwrap()
        };
        info!("action: {:?}", a);
        let ss = self.mcts.mdp.get_next_state_mut(&s, &a, self.rng);

        if let Some((a_id, m_id)) = pair {
            if let Some(ss_id) = self.mcts.find_s(self.node_id, a_id, m_id, ss) {
                self.node_id = ss_id;
            } else {
                self.node_id = self.mcts.add_state_node(ss);
            }
        } else {
            self.node_id = self.mcts.add_state_node(ss);
        }

        let c = self.mcts.mdp.d_cost(&s, &a, &ss);
        info!("c: {:?}", c);

        return Some((s, a, ss, c));
    }
}
