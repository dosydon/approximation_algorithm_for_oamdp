use log::debug;
use mcts::MCTSChanceNode;
use std::f32::{MAX, MIN};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct IntermediateNode<
    M: Eq + PartialEq + Debug + Copy + Clone + Hash,
    A: Eq + PartialEq + Debug + Copy + Clone + Hash,
> {
    pub(crate) assoc: M,
    pub(crate) m_id: usize,
    pub(crate) parent_s_id: usize,
    pub(crate) children: Vec<MCTSChanceNode<A>>,
    pub(crate) num_visited: usize,
    pub(crate) v: f32,
}

impl<
        M: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > IntermediateNode<M, A>
{
    pub(crate) fn new(assoc: M, m_id: usize, parent_s_id: usize) -> Self {
        IntermediateNode {
            assoc,
            m_id,
            parent_s_id,
            children: Vec::new(),
            num_visited: 0,
            v: 0.0,
        }
    }

    pub(crate) fn add_child(&mut self, a: A, initial_estimate: f32) {
        self.children
            .push(MCTSChanceNode::new(a, vec![], initial_estimate));
    }

    pub(crate) fn max_child(&self) -> f32 {
        let mut max = MIN;
        for child in &self.children {
            if child.num_visited == 0 {
                continue;
            }
            if child.q > max {
                max = child.q;
            }
        }
        assert!(max > MIN, "{:?}", self);
        max
    }

    fn bonus_bar_constant(&self, a_id: usize) -> f32 {
        if self.children[a_id].num_visited > 0 {
            ((2.0) * (self.num_visited as f32).ln() / (self.children[a_id].num_visited as f32))
                .sqrt()
        } else {
            MAX / 2.0
        }
    }

    fn ucb(&self, a_id: usize, c: f32) -> f32 {
        self.children[a_id].q + c * self.bonus_bar_constant(a_id)
    }

    pub(crate) fn best_and_node_ucb(&self, c: f32) -> Option<usize> {
        let mut cur_max = MIN;
        let mut cur_best = None;

        for a_id in 0..self.children.len() {
            let v = self.ucb(a_id, c);
            if v > cur_max {
                cur_max = v;
                cur_best = Some(a_id);
            }
        }

        cur_best
    }

    pub(crate) fn best_and_node_greedy_id(&self) -> Option<usize> {
        let mut cur_max = MIN;
        let mut result = None;
        if self.children.len() == 0 {
            return None;
        }

        for (a_id, and_node) in self.children.iter().enumerate() {
            let v = and_node.q;
            if and_node.num_visited == 0 {
                continue;
            }
            debug!(
                "{:?} {:?} {:?} {:?} {}",
                and_node.a,
                v,
                and_node.num_visited,
                and_node.children.len(),
                self.ucb(a_id, 5.0)
            );
            if v > cur_max {
                cur_max = v;
                result = Some(a_id);
            }
        }

        result
    }
}
