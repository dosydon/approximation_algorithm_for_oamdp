use super::change_node::MCTSChanceNode;
use core::fmt::Debug;
use core::hash::Hash;
use log::debug;
use std::f32::{MAX, MIN};

#[derive(PartialEq, Debug, Clone)]
pub struct MCTSDecisionNode<
    S: Eq + PartialEq + Debug + Copy + Clone + Hash,
    A: Eq + PartialEq + Debug + Copy + Clone + Hash,
> {
    pub assoc: S,
    pub id: usize,
    pub children: Vec<MCTSChanceNode<A>>,
    pub num_visited: usize,
    pub v: f32,
}

impl<
        S: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > MCTSDecisionNode<S, A>
{
    pub fn new(s: S, id: usize) -> MCTSDecisionNode<S, A> {
        MCTSDecisionNode {
            assoc: s,
            id: id,
            children: vec![],
            num_visited: 0,
            v: 0.0,
        }
    }

    pub fn add_child(&mut self, a: A, initial_estimate: f32) {
        let and_node = MCTSChanceNode::new(a, vec![], initial_estimate);
        self.children.push(and_node);
    }

    pub(crate) fn max_child(&self) -> f32 {
        let mut max = MIN;
        for and_node in &self.children {
            if and_node.num_visited == 0 {
                continue;
            }
            if and_node.q > max {
                max = and_node.q;
            }
        }
        max
    }

    pub fn best_and_node_greedy(&self) -> Option<&MCTSChanceNode<A>> {
        let mut cur_max = MIN;
        let mut result = None;
        if self.children.len() == 0 {
            return None;
        }
        debug!("{} {}", self.num_visited, self.v);

        for and_node in self.children.iter() {
            let v = and_node.q;
            if and_node.num_visited == 0 {
                continue;
            }
            debug!("a: {:?}", and_node.a);
            debug!("q: {:?}", v);
            debug!("n: {:?}", and_node.num_visited);
            //
            //             for c in and_node.children.iter() {
            //                 debug!("c: {:?}", c);
            //             }

            if v > cur_max {
                cur_max = v;
                result = Some(and_node);
            }
        }

        result
    }

    pub fn best_and_node_greedy_id(&self) -> Option<usize> {
        let mut cur_max = MIN;
        let mut result = None;
        if self.children.len() == 0 {
            return None;
        }
        debug!("{} {}", self.num_visited, self.v);

        for (a_id, and_node) in self.children.iter().enumerate() {
            let v = and_node.q;
            if and_node.num_visited == 0 {
                continue;
            }
            debug!(
                "{:?} {:?} {:?} {:?}",
                and_node.a,
                v,
                and_node.num_visited,
                self.ucb(and_node, 10.0)
            );
            if v > cur_max {
                cur_max = v;
                result = Some(a_id);
            }
        }

        result
    }

    fn bonus_bar_constant(&self, and_node: &MCTSChanceNode<A>) -> f32 {
        if and_node.num_visited > 0 {
            ((2.0) * (self.num_visited as f32).ln() / (and_node.num_visited as f32)).sqrt()
        } else {
            MAX / 2.0
        }
    }

    fn ucb(&self, and_node: &MCTSChanceNode<A>, c: f32) -> f32 {
        and_node.q + c * self.bonus_bar_constant(and_node)
    }

    pub fn best_and_node_ucb(&self, c: f32) -> Option<usize> {
        let mut cur_max = MIN;
        let mut cur_best = None;

        for (a_id, and_node) in self.children.iter().enumerate() {
            let v = self.ucb(and_node, c);
            if v > cur_max {
                cur_max = v;
                cur_best = Some(a_id);
            }
        }

        cur_best
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use mdp::grid_world::GridWorldAction::{AttemptDown, AttemptLeft, AttemptRight, AttemptUp};
//     use mdp::grid_world::GridWorldState;
//
//     use assert_approx_eq::assert_approx_eq;
//
//     #[test]
//     fn test_mcts_or_node() {
//         let mut or_node = MCTSDecisionNode::new(GridWorldState::new(0, 0), 0);
//         for a in [AttemptUp, AttemptDown, AttemptLeft, AttemptRight].iter() {
//             or_node.add_child(*a, 0.0);
//         }
//         or_node.num_visited = 10;
//         or_node.children[0].num_visited = 1;
//         or_node.children[1].num_visited = 2;
//         or_node.children[2].num_visited = 3;
//         or_node.children[3].num_visited = 4;
//
//         or_node.children[0].q = -2.5;
//         or_node.children[1].q = -2.0;
//         or_node.children[2].q = -3.0;
//         or_node.children[3].q = -4.0;
//
//         assert_approx_eq!(
//             or_node.bonus_bar_constant(&or_node.children[0]),
//             2.1459,
//             1e-4
//         );
//         assert_approx_eq!(
//             or_node.bonus_bar_constant(&or_node.children[1]),
//             1.5174271,
//             1e-4
//         );
//         assert_approx_eq!(
//             or_node.bonus_bar_constant(&or_node.children[2]),
//             1.2389,
//             1e-4
//         );
//         assert_approx_eq!(
//             or_node.bonus_bar_constant(&or_node.children[3]),
//             1.072983,
//             1e-4
//         );
//
//         assert_eq!(or_node.best_and_node_greedy(), Some(&or_node.children[1]));
//         assert_eq!(or_node.best_and_node_ucb(0.0), Some(1));
//         assert_eq!(or_node.best_and_node_ucb(1.0), Some(0));
//         assert_eq!(or_node.best_and_node_ucb(-1.0), Some(0));
//     }
// }
//
