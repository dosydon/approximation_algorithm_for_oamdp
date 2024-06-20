use mcts::{BackupOperator, Budget, MCTSTrait, RunEpisode, SetMCTSParams};
use mdp::episode_runner::monte_carlo_evaluation;
use mdp::mdp_traits::*;
use mdp::{arena::Arena, policy::policy_traits::GetActionMut};

use crate::traits::{DomainAction, EnumerateDomainAction, EnumerateMessage, Message, Set};
use rand::prelude::*;

use super::{intermediate_node::IntermediateNode, state_node::StateNode};

pub struct MCTSMA<M: StatesActions + DomainAction + Message, P> {
    pub mdp: M,
    pub(crate) base_line_policy: P,
    pub(crate) arena: Arena<StateNode<M::State, M::DomainAction, M::Message>>,
    pub(crate) c: f32,
    pub(crate) num_rollouts: usize,
    pub(crate) budget: Budget,
    pub(crate) backup_operator: BackupOperator,
    pub(crate) lookahead: Option<usize>,
}

impl<M: StatesActions + DomainAction + Message, P> SetMCTSParams for MCTSMA<M, P> {
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

impl<M: StatesActions + DomainAction + Message, P> MCTSTrait for MCTSMA<M, P> where
    Self: Eval + RunEpisode
{
}

impl<M: DisplayState<M::State> + StatesActions + DomainAction + Message, P> DisplayState<M::State>
    for MCTSMA<M, P>
{
    fn display(&self, s: &M::State) {
        self.mdp.display(s);
    }
}

impl<M: StatesActions + InitialState + DomainAction + Message, P> MCTSMA<M, P> {
    pub fn new(mdp: M, base_line_policy: P) -> MCTSMA<M, P> {
        let mut mcts = MCTSMA {
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

    pub fn set_c(mut self, c: f32) -> MCTSMA<M, P> {
        self.c = c;
        self
    }

    pub fn set_backup_operator(mut self, b: BackupOperator) -> MCTSMA<M, P> {
        self.backup_operator = b;
        self
    }

    pub fn set_budget(mut self, budget: Budget) -> MCTSMA<M, P> {
        self.budget = budget;
        self
    }

    pub fn set_num_rollouts(mut self, num_rollouts: usize) -> MCTSMA<M, P> {
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
    > MCTSMA<M, P>
where
    M::Action: From<(M::DomainAction, M::Message)> + Set<M::Message>,
{
    fn expand_state_node(&mut self, id: usize) {
        for (m_id, m) in self.mdp.enumerate_message().enumerate() {
            let child = IntermediateNode::new(*m, m_id, id);
            self.arena.get_node_mut(id).add_child(child);
        }
    }

    fn expand_message_node(&mut self, s_id: usize, m_id: usize) {
        for a in self.mdp.enumerate_domain_actions() {
            self.arena.get_node_mut(s_id).children[m_id].add_child(*a, 0.0);
        }
    }

    pub(crate) fn expand_recursive_state(&mut self, s_id: usize, rng: &mut ThreadRng) -> f32 {
        if self.mdp.is_terminal(&self.arena.get_node(s_id).assoc) {
            let s_node = self.arena.get_node_mut(s_id);
            s_node.num_visited += 1;

            0.0
        } else if self.arena.get_node(s_id).children.len() == 0 {
            assert!(self.arena.get_node(s_id).num_visited == 0);
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
            let m_id = self
                .arena
                .get_node(s_id)
                .best_intermediate_node_ucb(self.c)
                .expect("no message node selected");
            unsafe {
                let self_p = self as *mut Self;
                let nv = (*self_p).expand_recursive_message(s_id, m_id, rng);
                (*self_p).update_state_node(s_id, m_id, nv);
                nv
            }
        }
    }

    pub(crate) fn expand_recursive_message(
        &mut self,
        s_id: usize,
        m_id: usize,
        rng: &mut ThreadRng,
    ) -> f32 {
        if self.arena.get_node(s_id).children[m_id].children.len() == 0 {
            self.expand_message_node(s_id, m_id);

            let cost = if self.num_rollouts > 0 {
                let s = &self.arena.get_node(s_id).assoc;
                let mut joint_a = self
                    .base_line_policy
                    .get_action_mut(s, &mut self.mdp, rng)
                    .unwrap();
                joint_a.set(self.arena.get_node(s_id).children[m_id].assoc);
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

            let m_node = &mut self.arena.get_node_mut(s_id).children[m_id];
            m_node.num_visited += 1;
            m_node.v = -1.0 * cost;

            -1.0 * cost
        } else {
            let a_id = self.arena.get_node(s_id).children[m_id]
                .best_and_node_ucb(self.c)
                .expect("no action node selected");
            let s = self.arena.get_node(s_id).assoc;
            let domain_a = self.arena.get_node(s_id).children[m_id].children[a_id].a;
            let a = M::Action::from((domain_a, self.arena.get_node(s_id).children[m_id].assoc));
            let ss = self.mdp.get_next_state_mut(&s, &a, rng);

            unsafe {
                let self_p = self as *mut Self;
                if let Some(ss_id) = (*self_p).find_s(s_id, m_id, a_id, ss) {
                    let r = (-1.0) * (*self_p).mdp.cost(&s, &a);
                    let future_r = (*self_p).expand_recursive_state(ss_id, rng);
                    assert!(r <= 0.0);
                    assert!(future_r <= 0.0);

                    (*self_p).update_message_node(s_id, m_id, a_id, r, future_r);
                    r + future_r
                } else {
                    let ss_id = (*self_p).add_state_node(ss);
                    //                     trace!("{:?}", ss);

                    (*self_p).arena.get_node_mut(s_id).children[m_id].children[a_id]
                        .children
                        .push(ss_id);

                    let r = (-1.0) * (*self_p).mdp.cost(&s, &a);
                    let future_r = (*self_p).expand_recursive_state(ss_id, rng);
                    assert!(r <= 0.0);
                    assert!(future_r <= 0.0);

                    (*self_p).update_message_node(s_id, m_id, a_id, r, future_r);
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
    use mdp::mdp_traits::StatesActions;
    use mdp::policy::random_from_candidates_policy::RandomFromCandidatesPolicy;
    use rand::thread_rng;

    use crate::domains::baker_grid::BakerCOAMDPBuilder;
    use crate::domains::baker_grid::BakerCommunicationAction;
    use crate::domains::baker_grid::BakerJointAction;
    use crate::traits::DomainAction;
    use crate::traits::Message;

    use super::MCTSMA;

    impl<M: StatesActions + DomainAction + Message, P> MCTSMA<M, P> {
        pub(crate) fn is_visit_count_consistent_state_node(&self, s_id: usize) -> bool {
            let s_node = self.arena.get_node(s_id);
            if s_node.num_visited <= 0 {
                true
            } else {
                let mut sum = 0;
                for (m_id, m_node) in s_node.children.iter().enumerate() {
                    if !self.is_visit_count_consistent_message_node(s_id, m_id) {
                        return false;
                    }
                    sum += m_node.num_visited;
                }

                s_node.num_visited == (sum + 1)
            }
        }

        pub(crate) fn is_visit_count_consistent_message_node(
            &self,
            s_id: usize,
            m_id: usize,
        ) -> bool {
            let s_node = self.arena.get_node(s_id);
            let m_node = &s_node.children[m_id];
            if m_node.num_visited <= 0 {
                true
            } else {
                let mut sum = 0;
                for (_, a_node) in m_node.children.iter().enumerate() {
                    sum += a_node.num_visited;
                    for ss_id in a_node.children.iter() {
                        if !self.is_visit_count_consistent_state_node(*ss_id) {
                            return false;
                        }
                    }
                }
                m_node.num_visited == (sum + 1)
            }
        }
    }

    #[test]
    fn test_mcts_split() {
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
        let mut mcts = MCTSMA::new(oamdp, policy);
        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();
        assert!(mcts.is_visit_count_consistent_state_node(0));

        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();
        assert!(mcts.is_visit_count_consistent_state_node(0));

        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();
        assert!(mcts.is_visit_count_consistent_state_node(0));

        mcts.expand_recursive_state(0, &mut rng);
        mcts.dump();
        assert!(mcts.is_visit_count_consistent_state_node(0));
    }

    #[test]
    fn test_mcts_split_max() {
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
        let mut mcts = MCTSMA::new(oamdp, policy).set_backup_operator(mcts::BackupOperator::Max);
        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();
        assert!(mcts.is_visit_count_consistent_state_node(0));

        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();
        assert!(mcts.is_visit_count_consistent_state_node(0));

        mcts.expand_recursive_state(0, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        mcts.dump();
        assert!(mcts.is_visit_count_consistent_state_node(0));

        mcts.expand_recursive_state(0, &mut rng);
        mcts.dump();
        assert!(mcts.is_visit_count_consistent_state_node(0));
    }
}
