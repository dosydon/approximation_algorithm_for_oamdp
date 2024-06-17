use core::f32::MIN;

use mdp::arena::Arena;
use mdp::episode_runner::monte_carlo_evaluation;
use mdp::mdp_traits::*;
use mdp::policy::policy_traits::GetActionMut;

use crate::decision_node::MCTSDecisionNode;
use crate::{BackupOperator, Budget, MCTSTrait, SetMCTSParams};

use rand::prelude::*;

pub struct MCTS<M: StatesActions, P> {
    pub mdp: M,
    pub(crate) base_line_policy: P,
    pub(crate) arena: Arena<MCTSDecisionNode<M::State, M::Action>>,
    pub(crate) c: f32,
    pub(crate) num_rollouts: usize,
    pub(crate) budget: Budget,
    pub(crate) backup_operator: BackupOperator,
    pub(crate) lookahead: Option<usize>,
}

impl<M, P> MCTSTrait for MCTS<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + GetNextStateMut
        + Cost
        + DCost
        + InitialState
        + ActionEnumerable
        + DisplayState<M::State>,
    P: IntoEvalMut<M> + GetActionMut<M::State, M>,
{
}

impl<M: StatesActions + DisplayState<M::State>, P> DisplayState<M::State> for MCTS<M, P> {
    fn display(&self, s: &M::State) {
        self.mdp.display(s);
    }
}

impl<M: StatesActions, P> SetMCTSParams for MCTS<M, P> {
    fn set_c(&mut self, c: f32) {
        self.c = c;
    }

    fn set_num_rollouts(&mut self, num_rollouts: usize) {
        self.num_rollouts = num_rollouts;
    }

    fn set_budget(&mut self, budget: Budget) {
        self.budget = budget;
    }

    fn set_lookahead(&mut self, horizon: Option<usize>) {
        self.lookahead = horizon;
    }
}

impl<M: StatesActions + InitialState, P> MCTS<M, P> {
    pub fn set_c(mut self, c: f32) -> MCTS<M, P> {
        self.c = c;
        self
    }

    pub fn set_num_rollouts(mut self, num_rollouts: usize) -> MCTS<M, P> {
        self.num_rollouts = num_rollouts;
        self
    }

    pub fn set_budget(mut self, budget: Budget) -> MCTS<M, P> {
        self.budget = budget;
        self
    }
}

impl<M: StatesActions + InitialState, P> MCTS<M, P> {
    pub fn new(mdp: M, base_line_policy: P) -> MCTS<M, P> {
        let mut mcts = MCTS {
            mdp: mdp,
            base_line_policy: base_line_policy,
            arena: Arena::new(),
            c: 0.5,
            num_rollouts: 10,
            budget: Budget::NumIterations(1000),
            backup_operator: BackupOperator::MonteCarlo,
            lookahead: None,
        };

        mcts.add_node(mcts.mdp.initial_state());
        mcts
    }

    pub fn set_backup_operator(mut self, backup_operator: BackupOperator) -> MCTS<M, P> {
        self.backup_operator = backup_operator;
        self
    }

    pub(crate) fn add_node(&mut self, s: M::State) -> usize {
        let next_id = self.arena.next_id();

        self.arena.add_node(MCTSDecisionNode::new(s, next_id));
        next_id
    }

    pub fn dump(&self) {
        for (i, vec) in self.arena.nodes.iter().enumerate() {
            println!("{}: {:?}", i, vec);
        }
    }

    pub fn node_count(&self) -> usize {
        self.arena.next_id()
    }

    pub fn clear(&mut self) {
        self.arena.clear();
        self.add_node(self.mdp.initial_state());
    }
}

impl<
        M: ActionAvailability
            + ActionEnumerable
            + InitialState
            + IsTerminal
            + StatesActions
            + GetNextStateMut
            + InitialState
            + Cost,
        P: IntoEvalMut<M>,
    > MCTS<M, P>
{
    pub fn solve(&mut self, n: usize, rng: &mut ThreadRng) {
        for _i in 0..n {
            self.expand_recursive(0, rng);
        }
    }

    fn expand_node(&mut self, id: usize) {
        for a in self.mdp.enumerate_actions().cloned() {
            self.arena.get_node_mut(id).add_child(a, 0.0);
        }
    }

    fn update_monte_carlo(&mut self, s_id: usize, a_id: usize, r: f32, future_r: f32) {
        let nv = r + future_r;
        let s_node = self.arena.get_node_mut(s_id);
        s_node.num_visited += 1;
        s_node.children[a_id].num_visited += 1;
        s_node.v = s_node.v + (nv - s_node.v) / s_node.num_visited as f32;
        s_node.children[a_id].q = s_node.children[a_id].q
            + (nv - s_node.children[a_id].q) / s_node.children[a_id].num_visited as f32;
    }

    fn update(&mut self, s_id: usize, a_id: usize, r: f32, future_r: f32) {
        match self.backup_operator {
            BackupOperator::MonteCarlo => self.update_monte_carlo(s_id, a_id, r, future_r),
            BackupOperator::Max => self.update_max(s_id, a_id, r, future_r),
        }
    }

    fn update_max(&mut self, s_id: usize, a_id: usize, r: f32, _future_r: f32) {
        let s_node = self.arena.get_node(s_id);

        let mut future = 0.0;
        let mut total_visited = 0;
        assert!(s_node.children[a_id].children.len() > 0);
        for ss_id in s_node.children[a_id].children.iter() {
            let ss_node = self.arena.get_node(*ss_id);
            assert!(ss_node.num_visited > 0);
            future += ss_node.v * ss_node.num_visited as f32;
            total_visited += ss_node.num_visited;
        }
        future /= (s_node.children[a_id].num_visited + 1) as f32;

        let s_node = self.arena.get_node_mut(s_id);
        s_node.num_visited += 1;
        s_node.children[a_id].num_visited += 1;
        assert_eq!(total_visited, s_node.children[a_id].num_visited);

        s_node.children[a_id].q = r + future;
        s_node.v = s_node.max_child();
    }

    pub(crate) fn expand_recursive(&mut self, s_id: usize, rng: &mut ThreadRng) -> f32 {
        if self.mdp.is_terminal(&self.arena.get_node(s_id).assoc) {
            let s_node = self.arena.get_node_mut(s_id);
            s_node.num_visited += 1;
            s_node.v = 0.0;

            0.0
        } else if self.arena.get_node(s_id).children.len() == 0 {
            self.expand_node(s_id);
            let cost = if self.num_rollouts <= 0 {
                0.0
            } else {
                let mut runner = self
                    .base_line_policy
                    .into_eval_mut(self.arena.get_node(s_id).assoc, &mut self.mdp)
                    .set_max_horizon(self.lookahead);
                let cost = monte_carlo_evaluation(&mut runner, rng, self.num_rollouts);
                //                 assert!(
                //                     cost > 0.0,
                //                     "cost: {:?} {:?}",
                //                     cost,
                //                     self.arena.get_node(s_id)
                //                 );
                cost
            };

            let s_node = self.arena.get_node_mut(s_id);
            s_node.num_visited += 1;
            s_node.v = -1.0 * cost;

            -1.0 * cost
        } else {
            if let Some(a_id) = self.arena.get_node(s_id).best_and_node_ucb(self.c) {
                let s = self.arena.get_node(s_id).assoc;
                let a = self.arena.get_node(s_id).children[a_id].a;
                let ss = self.mdp.get_next_state_mut(&s, &a, rng);

                unsafe {
                    let self_p = self as *mut Self;
                    if let Some(ss_id) =
                        (*self_p).arena.get_node(s_id).children[a_id].find_s(&ss, &(*self_p).arena)
                    {
                        let r = (-1.0) * (*self_p).mdp.cost(&s, &a);
                        let future_r = (*self_p).expand_recursive(ss_id, rng);
                        let nv = r + future_r;
                        (*self_p).update(s_id, a_id, r, future_r);
                        nv
                    } else {
                        let ss_id = (*self_p).add_node(ss);

                        (*self_p).arena.get_node_mut(s_id).children[a_id]
                            .children
                            .push(ss_id);

                        let r = (-1.0) * (*self_p).mdp.cost(&s, &a);
                        let future_r = (*self_p).expand_recursive(ss_id, rng);
                        let nv = r + future_r;
                        (*self_p).update(s_id, a_id, r, future_r);
                        nv
                    }
                }
            } else {
                panic!("no and node selected");
            }
        }
    }

    pub fn root_value(&self) -> f32 {
        let mut cur_max = MIN;
        let s_node = self.arena.get_node(0);

        for (a_id, _a) in self.mdp.enumerate_actions().enumerate() {
            let v = s_node.children[a_id].q;
            if v > cur_max {
                cur_max = v;
            }
        }
        cur_max
    }

    pub fn dump_tree(&self, max_depth: usize) {
        self.dump_tree_recursive(0, 0, max_depth);
    }

    fn dump_tree_recursive(&self, node_id: usize, cur_depth: usize, max_depth: usize) {
        if cur_depth > max_depth {
            return;
        }

        let s_node = self.arena.get_node(node_id);
        println!(
            "{} {}: num_visited: {} v: {}",
            "   ".repeat(2 * cur_depth),
            node_id,
            s_node.num_visited,
            s_node.v
        );

        for (a_id, _a) in self.mdp.enumerate_actions().enumerate() {
            let a_node = &s_node.children[a_id];
            println!(
                "{} {:?} num_visited: {} v: {}",
                "   ".repeat(2 * cur_depth + 1),
                a_node.a,
                a_node.num_visited,
                a_node.q
            );
            for ss_id in a_node.children.iter() {
                self.dump_tree_recursive(*ss_id, cur_depth + 1, max_depth)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
    use mdp::grid_world::GridWorldAction::AttemptUp;
    use mdp::grid_world::{GridWorldMDP, GridWorldState};
    use mdp::policy::random_from_candidates_policy::RandomFromCandidatesPolicy;
    use mdp::policy::random_policy::RandomPolicy;

    impl<
            M: ActionAvailability
                + ActionEnumerable
                + InitialState
                + IsTerminal
                + StatesActions
                + GetNextState
                + InitialState
                + DCost
                + Cost,
            P: IntoEvalMut<M>,
        > MCTS<M, P>
    {
        fn is_visit_count_consistent(&self, s_id: usize) -> bool {
            let s_node = self.arena.get_node(s_id);
            let mut sum = 0;
            for a_node in s_node.children.iter() {
                sum += a_node.num_visited;
                for ss_id in a_node.children.iter() {
                    if !self.is_visit_count_consistent(*ss_id) {
                        return false;
                    }
                }
            }
            //         println!("{} {}", s_node.num_visited, sum);
            s_node.num_visited == (sum + 1)
        }
    }

    #[test]
    fn test_mcts_expand_recursive() {
        let mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let finite_horizon_mdp = FiniteHorizonWrapper::new(mdp, 4);

        let random_policy = RandomPolicy {};
        let mut mcts = MCTS::new(finite_horizon_mdp, random_policy);
        let mut rng = thread_rng();
        mcts.solve(1, &mut rng);
        assert_eq!(mcts.node_count(), 1);
        assert_eq!(mcts.arena.get_node(0).num_visited, 1);
        assert_eq!(mcts.arena.get_node(0).children.len(), 4);

        mcts.dump();
        assert!(mcts.is_visit_count_consistent(0));

        mcts.solve(4, &mut rng);
        assert!(mcts.is_visit_count_consistent(0));
        mcts.dump();

        mcts.solve(10, &mut rng);
        assert!(mcts.is_visit_count_consistent(0));
        mcts.dump();
    }

    #[test]
    fn test_mcts() {
        let mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let finite_horizon_mdp = FiniteHorizonWrapper::new(mdp, 4);

        let random_policy = RandomFromCandidatesPolicy::new(vec![AttemptUp]);
        let mut mcts = MCTS::new(finite_horizon_mdp, random_policy);
        let mut rng = thread_rng();
        mcts.solve(1, &mut rng);
        mcts.dump();
        mcts.solve(1, &mut rng);
        mcts.dump();
        mcts.solve(1, &mut rng);
        mcts.dump();
    }
}
