use mdp::mdp_traits::*;

use crate::traits::{DomainAction, Message};

use super::{mcts_am::MCTSAM, MCTSMA};

impl<M: StatesActions + InitialState + DomainAction + Message, P> MCTSMA<M, P> {
    pub(crate) fn find_s(&self, id: usize, m_id: usize, a_id: usize, s: M::State) -> Option<usize> {
        let s_node = self.arena.get_node(id);
        let m_node = &s_node.children[m_id];
        let a_node = &m_node.children[a_id];
        for s_id in a_node.children.iter() {
            if self.arena.get_node(*s_id).assoc == s {
                return Some(*s_id);
            }
        }
        None
    }
}

impl<M: StatesActions + InitialState + DomainAction + Message, P> MCTSAM<M, P> {
    pub(crate) fn find_s(&self, id: usize, a_id: usize, m_id: usize, s: M::State) -> Option<usize> {
        let s_node = self.arena.get_node(id);
        let a_node = &s_node.children[a_id];
        let m_node = &a_node.children[m_id];

        for s_id in m_node.children.iter() {
            if self.arena.get_node(*s_id).assoc == s {
                return Some(*s_id);
            }
        }
        None
    }
}
