use log::debug;
use std::f32::{MAX, MIN};
use std::fmt::Debug;
use std::hash::Hash;

use super::intermediate_node::IntermediateNode;

#[derive(PartialEq, Debug, Clone)]
pub struct StateNode<
    S: Eq + PartialEq + Debug + Copy + Clone + Hash,
    A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    M: Eq + PartialEq + Debug + Copy + Clone + Hash,
> {
    pub(crate) assoc: S,
    pub(crate) id: usize,
    pub(crate) children: Vec<IntermediateNode<M, A>>,
    pub(crate) num_visited: usize,
    pub(crate) v: f32,
}

impl<
        S: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
        M: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > StateNode<S, A, M>
{
    pub(crate) fn new(s: S, id: usize) -> StateNode<S, A, M> {
        StateNode {
            assoc: s,
            id: id,
            children: vec![],
            num_visited: 0,
            v: 0.0,
        }
    }

    pub(crate) fn add_child(&mut self, child: IntermediateNode<M, A>) {
        self.children.push(child);
    }

    pub(crate) fn max_child(&self) -> f32 {
        let mut max = MIN;
        for child in &self.children {
            if child.num_visited == 0 {
                continue;
            }

            if child.v > max {
                max = child.v;
            }
        }
        assert!(max > MIN);
        max
    }

    fn bonus_bar_constant(&self, m_id: usize) -> f32 {
        if self.children[m_id].num_visited > 0 {
            ((2.0) * (self.num_visited as f32).ln() / (self.children[m_id].num_visited as f32))
                .sqrt()
        } else {
            MAX / 2.0
        }
    }

    fn ucb(&self, m_id: usize, c: f32) -> f32 {
        self.children[m_id].v + c * self.bonus_bar_constant(m_id)
    }

    pub(crate) fn best_intermediate_node_ucb(&self, c: f32) -> Option<usize> {
        let mut cur_max = MIN;
        let mut cur_best = None;

        assert!(self.children.len() > 0);
        for m_id in 0..self.children.len() {
            let v = self.ucb(m_id, c);
            if v > cur_max {
                cur_max = v;
                cur_best = Some(m_id);
            }
        }

        if cur_best == None {
            println!("{} {:?}", self.num_visited, self.children);
        }

        cur_best
    }

    pub(crate) fn best_intermediate_node_greedy_id(&self, c: f32) -> Option<(usize, usize)> {
        let mut cur_max = MIN;
        let mut result = None;
        if self.children.len() == 0 {
            return None;
        }
        debug!("{} {}", self.num_visited, self.v);

        for (m_id, m_node) in self.children.iter().enumerate() {
            let v = m_node.v;
            if m_node.num_visited == 0 {
                continue;
            }

            debug!(
                "{:?} {:?} {:?} {:?}",
                m_node.assoc,
                v,
                m_node.num_visited,
                self.ucb(m_id, c)
            );
            if let Some(a_id) = m_node.best_and_node_greedy_id() {
                if v > cur_max {
                    cur_max = v;
                    result = Some((m_id, a_id));
                }
            }
        }

        result
    }
}
