use mdp::mdp_traits::StatesActions;

use crate::traits::{DomainAction, Message};

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

    pub(crate) fn is_visit_count_consistent_message_node(&self, s_id: usize, m_id: usize) -> bool {
        let s_node = self.arena.get_node(s_id);
        let m_node = &s_node.children[m_id];
        if m_node.num_visited <= 0 {
            true
        } else {
            let mut sum = 0;
            for (a_id, a_node) in m_node.children.iter().enumerate() {
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
