use mcts::{BackupOperator, Budget, MCTSTrait, RunEpisode, SetMCTSParams};
use mdp::{
    arena::Arena,
    episode_runner::monte_carlo_evaluation,
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, DisplayState, Eval, GetNextStateMut,
        InitialState, IntoEvalMut, IsTerminal, SetMaxHorizon, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};
use rand::rngs::ThreadRng;

use crate::traits::{DomainAction, EnumerateDomainAction, EnumerateMessage, Message, Set};

use super::{intermediate_node::IntermediateNode, state_node::StateNode};

pub struct MCTSAM<M: StatesActions + DomainAction + Message, P> {
    pub mdp: M,
    pub(crate) base_line_policy: P,
    pub(crate) arena: Arena<StateNode<M::State, M::Message, M::DomainAction>>,
    pub(crate) c: f32,
    pub(crate) num_rollouts: usize,
    pub(crate) budget: Budget,
    pub(crate) backup_operator: BackupOperator,
    pub(crate) lookahead: Option<usize>,
}

impl<M: StatesActions + DomainAction + Message, P> SetMCTSParams for MCTSAM<M, P> {
    fn set_budget(&mut self, budget: Budget) {
        self.budget = budget;
    }

    fn set_c(&mut self, c: f32) {
        self.c = c;
    }

    fn set_num_rollouts(&mut self, num_rollouts: usize) {
        self.num_rollouts = num_rollouts;
    }

    fn set_lookahead(&mut self, horizon: Option<usize>) {
        self.lookahead = horizon;
    }
}

impl<M: StatesActions + DomainAction + Message, P> MCTSTrait for MCTSAM<M, P> where
    Self: Eval + RunEpisode
{
}

impl<M: DisplayState<M::State> + StatesActions + DomainAction + Message, P> DisplayState<M::State>
    for MCTSAM<M, P>
{
    fn display(&self, s: &M::State) {
        self.mdp.display(s);
    }
}

impl<M: StatesActions + InitialState + DomainAction + Message, P> MCTSAM<M, P> {
    pub fn new(mdp: M, base_line_policy: P) -> MCTSAM<M, P> {
        let mut mcts = MCTSAM {
            mdp: mdp,
            base_line_policy: base_line_policy,
            arena: Arena::new(),
            c: 0.5,
            num_rollouts: 10,
            budget: Budget::NumIterations(1000),
            backup_operator: BackupOperator::MonteCarlo,
            lookahead: None,
        };

        mcts.add_state_node(mcts.mdp.initial_state());
        mcts
    }

    pub fn set_c(mut self, c: f32) -> MCTSAM<M, P> {
        self.c = c;
        self
    }

    pub fn set_backup_operator(mut self, b: BackupOperator) -> MCTSAM<M, P> {
        self.backup_operator = b;
        self
    }

    pub fn set_budget(mut self, budget: Budget) -> MCTSAM<M, P> {
        self.budget = budget;
        self
    }

    pub fn set_num_rollouts(mut self, num_rollouts: usize) -> MCTSAM<M, P> {
        self.num_rollouts = num_rollouts;
        self
    }

    pub(crate) fn add_state_node(&mut self, s: M::State) -> usize {
        let next_id = self.arena.next_id();

        self.arena.add_node(StateNode::new(s, next_id));
        next_id
    }

    pub fn dump(&self) {
        for (i, vec) in self.arena.nodes.iter().enumerate() {
            println!("{}: {:?}", i, vec);
        }
        println!("");
    }

    pub fn node_count(&self) -> usize {
        self.arena.next_id()
    }

    pub fn clear(&mut self) {
        self.arena.clear();
        self.add_state_node(self.mdp.initial_state());
    }
}

impl<
        M: ActionAvailability
            + ActionEnumerable
            + DomainAction
            + Message
            + EnumerateMessage
            + EnumerateDomainAction
            + InitialState
            + IsTerminal
            + StatesActions
            + GetNextStateMut
            + InitialState
            + Cost
            + DCost,
        P: GetActionMut<M::State, M> + IntoEvalMut<M>,
    > MCTSAM<M, P>
where
    M::Action: From<(M::DomainAction, M::Message)> + Set<M::DomainAction>,
{
    fn expand_state_node(&mut self, id: usize) {
        for (a_id, a) in self.mdp.enumerate_domain_actions().enumerate() {
            let child = IntermediateNode::new(*a, a_id, id);
            self.arena.get_node_mut(id).add_child(child);
        }
    }

    fn expand_action_node(&mut self, s_id: usize, a_id: usize) {
        for m in self.mdp.enumerate_message() {
            self.arena.get_node_mut(s_id).children[a_id].add_child(*m, 0.0);
        }
    }

    pub(crate) fn expand_recursive_state(&mut self, s_id: usize, rng: &mut ThreadRng) -> f32 {
        if self.mdp.is_terminal(&self.arena.get_node(s_id).assoc) {
            let s_node = self.arena.get_node_mut(s_id);
            s_node.num_visited += 1;

            0.0
        } else if self.arena.get_node(s_id).children.len() == 0 {
            self.expand_state_node(s_id);

            let cost = if self.num_rollouts > 0 {
                let mut runner = self
                    .base_line_policy
                    .into_eval_mut(self.arena.get_node(s_id).assoc, &mut self.mdp)
                    .set_max_horizon(self.lookahead);
                let cost = monte_carlo_evaluation(&mut runner, rng, self.num_rollouts);
                cost
            } else {
                0.0
            };

            let s_node = self.arena.get_node_mut(s_id);
            s_node.num_visited += 1;
            s_node.v = -1.0 * cost;

            -1.0 * cost
        } else {
            let a_id = self
                .arena
                .get_node(s_id)
                .best_intermediate_node_ucb(self.c)
                .expect("no action node selected");
            unsafe {
                let self_p = self as *mut Self;
                let nv = (*self_p).expand_recursive_action(s_id, a_id, rng);
                (*self_p).update_state_node(s_id, a_id, nv);
                nv
            }
        }
    }

    pub(crate) fn expand_recursive_action(
        &mut self,
        s_id: usize,
        a_id: usize,
        rng: &mut ThreadRng,
    ) -> f32 {
        if self.arena.get_node(s_id).children[a_id].children.len() == 0 {
            self.expand_action_node(s_id, a_id);

            let cost = if self.num_rollouts > 0 {
                let s = &self.arena.get_node(s_id).assoc;
                let mut joint_a = self
                    .base_line_policy
                    .get_action_mut(s, &mut self.mdp, rng)
                    .unwrap();
                joint_a.set(self.arena.get_node(s_id).children[a_id].assoc);
                let ss = self.mdp.get_next_state_mut(&s, &joint_a, rng);
                let c = self.mdp.d_cost(&s, &joint_a, &ss);

                let mut runner = self
                    .base_line_policy
                    .into_eval_mut(ss, &mut self.mdp)
                    .set_max_horizon(self.lookahead);
                let cost = monte_carlo_evaluation(&mut runner, rng, self.num_rollouts);
                cost + c
            } else {
                0.0
            };

            let a_node = &mut self.arena.get_node_mut(s_id).children[a_id];
            a_node.num_visited += 1;
            a_node.v = -1.0 * cost;

            -1.0 * cost
        } else {
            let m_id = self.arena.get_node(s_id).children[a_id]
                .best_and_node_ucb(self.c)
                .expect("no action node selected");
            let s = self.arena.get_node(s_id).assoc;
            let message = self.arena.get_node(s_id).children[a_id].children[m_id].a;
            let a = M::Action::from((self.arena.get_node(s_id).children[a_id].assoc, message));
            let ss = self.mdp.get_next_state_mut(&s, &a, rng);

            unsafe {
                let self_p = self as *mut Self;
                if let Some(ss_id) = (*self_p).find_s(s_id, a_id, m_id, ss) {
                    let r = (-1.0) * (*self_p).mdp.cost(&s, &a);
                    let future_r = (*self_p).expand_recursive_state(ss_id, rng);
                    (*self_p).update_action_node(s_id, a_id, m_id, r, future_r);
                    r + future_r
                } else {
                    let ss_id = (*self_p).add_state_node(ss);

                    (*self_p).arena.get_node_mut(s_id).children[a_id].children[m_id]
                        .children
                        .push(ss_id);

                    let r = (-1.0) * (*self_p).mdp.cost(&s, &a);
                    let future_r = (*self_p).expand_recursive_state(ss_id, rng);
                    (*self_p).update_action_node(s_id, a_id, m_id, r, future_r);
                    r + future_r
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use mdp::baker_grid::BakerGridAction::*;
    use mdp::mdp_traits::Build;
    use mdp::policy::random_from_candidates_policy::RandomFromCandidatesPolicy;
    use rand::thread_rng;

    use crate::domains::baker_grid::BakerCOAMDPBuilder;
    use crate::domains::baker_grid::BakerCommunicationAction;
    use crate::domains::baker_grid::BakerJointAction;

    use super::MCTSAM;

    #[test]
    fn test_mcts_am() {
        let builder = BakerCOAMDPBuilder::new(1);
        let oamdp = builder.build();

        let policy = RandomFromCandidatesPolicy::new(
            vec![
                North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest, Stay,
            ]
            .iter()
            .map(|a| BakerJointAction::new(*a, BakerCommunicationAction::None))
            .collect::<Vec<_>>(),
        );

        let mut rng = thread_rng();
        let mut mcts = MCTSAM::new(oamdp, policy);
        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();

        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();

        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();

        mcts.expand_recursive_state(0, &mut rng);
        mcts.dump();
    }
}
