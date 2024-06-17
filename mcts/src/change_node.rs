use core::fmt::Debug;
use core::hash::Hash;

use mdp::arena::Arena;

use crate::decision_node::MCTSDecisionNode;

#[derive(PartialEq, Debug, Clone)]
pub struct MCTSChanceNode<A: Eq + PartialEq + Debug + Copy + Clone + Hash> {
    pub a: A,
    pub children: Vec<usize>,
    pub num_visited: usize,
    pub q: f32,
}

impl<A: Eq + PartialEq + Debug + Copy + Clone + Hash> MCTSChanceNode<A> {
    pub fn new(a: A, children: Vec<usize>, cost_estimate: f32) -> MCTSChanceNode<A> {
        MCTSChanceNode::<A> {
            a: a,
            children: children,
            num_visited: 0,
            q: cost_estimate,
        }
    }

    pub fn find_s<S: PartialEq + Eq + Clone + Copy + Hash + Debug>(
        &self,
        s: &S,
        arena: &Arena<MCTSDecisionNode<S, A>>,
    ) -> Option<usize> {
        for child in self.children.iter() {
            let child_node = arena.get_node(*child);
            if child_node.assoc == *s {
                return Some(*child);
            }
        }
        None
    }
}
