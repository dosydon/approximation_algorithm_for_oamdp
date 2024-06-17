use mcts::BackupOperator;
use mdp::mdp_traits::*;

use crate::traits::{DomainAction, Message};

use super::MCTSMA;

impl<M: DomainAction + Message + IsTerminal + StatesActions, P> MCTSMA<M, P> {
    pub(crate) fn update_state_node(&mut self, s_id: usize, m_id: usize, nv: f32) {
        match self.backup_operator {
            BackupOperator::MonteCarlo => self.update_state_node_monte_carlo(s_id, m_id, nv),
            BackupOperator::Max => self.update_state_node_max(s_id),
        }
    }

    pub(crate) fn update_state_node_monte_carlo(&mut self, s_id: usize, _m_id: usize, nv: f32) {
        let s_node = self.arena.get_node_mut(s_id);
        s_node.num_visited += 1;

        s_node.v = s_node.v + (nv - s_node.v) / s_node.num_visited as f32;
    }

    pub(crate) fn update_state_node_max(&mut self, s_id: usize) {
        let s_node = self.arena.get_node_mut(s_id);
        s_node.num_visited += 1;

        s_node.v = s_node.max_child();
    }

    pub(crate) fn update_message_node(
        &mut self,
        s_id: usize,
        m_id: usize,
        a_id: usize,
        r: f32,
        future_r: f32,
    ) {
        match self.backup_operator {
            BackupOperator::MonteCarlo => {
                self.update_message_node_monte_carlo(s_id, m_id, a_id, r, future_r)
            }
            BackupOperator::Max => self.update_message_node_max(s_id, m_id, a_id, r, future_r),
        }
    }

    fn update_message_node_monte_carlo(
        &mut self,
        s_id: usize,
        m_id: usize,
        a_id: usize,
        r: f32,
        future_r: f32,
    ) {
        let nv = r + future_r;
        let s_node = self.arena.get_node_mut(s_id);
        let m_node = &mut s_node.children[m_id];
        let a_node = &mut m_node.children[a_id];

        a_node.num_visited += 1;
        m_node.num_visited += 1;
        a_node.q = a_node.q + (nv - a_node.q) / a_node.num_visited as f32;
        m_node.v = m_node.v + (nv - m_node.v) / m_node.num_visited as f32;
    }

    fn update_message_node_max(
        &mut self,
        s_id: usize,
        m_id: usize,
        a_id: usize,
        r: f32,
        _future_r: f32,
    ) {
        let s_node = self.arena.get_node(s_id);
        let m_node = &s_node.children[m_id];
        let a_node = &m_node.children[a_id];

        let mut future = 0.0;
        let mut total_visited = 0;
        assert!(a_node.children.len() > 0);

        for ss_id in a_node.children.iter() {
            let v = self.arena.get_node(*ss_id).v;
            let num_visited = self.arena.get_node(*ss_id).num_visited;
            if num_visited <= 0 {
                continue;
            }

            future += v * num_visited as f32;
            total_visited += num_visited;
        }

        let s_node = self.arena.get_node_mut(s_id);
        let m_node = &mut s_node.children[m_id];
        let a_node = &mut m_node.children[a_id];

        m_node.num_visited += 1;
        a_node.num_visited += 1;
        //         assert_eq!(total_visited, a_node.num_visited, "{:?}", a_node);

        future /= total_visited as f32;

        a_node.q = r + future;
        m_node.v = m_node.max_child();
    }
}
