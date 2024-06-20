use mcts::BackupOperator;
use mdp::mdp_traits::{IsTerminal, StatesActions};

use crate::traits::{DomainAction, Message};

use super::MCTSAM;
impl<M: DomainAction + Message + IsTerminal + StatesActions, P> MCTSAM<M, P> {
    pub(crate) fn update_state_node(&mut self, s_id: usize, a_id: usize, nv: f32) {
        match self.backup_operator {
            BackupOperator::MonteCarlo => self.update_state_node_monte_carlo(s_id, a_id, nv),
            BackupOperator::Max => self.update_state_node_max(s_id),
        }
    }

    pub(crate) fn update_state_node_monte_carlo(&mut self, s_id: usize, _a_id: usize, nv: f32) {
        let s_node = self.arena.get_node_mut(s_id);
        s_node.num_visited += 1;

        s_node.v = s_node.v + (nv - s_node.v) / s_node.num_visited as f32;
    }

    pub(crate) fn update_state_node_max(&mut self, s_id: usize) {
        let s_node = self.arena.get_node_mut(s_id);
        s_node.num_visited += 1;

        s_node.v = s_node.max_child();
    }

    pub(crate) fn update_action_node(
        &mut self,
        s_id: usize,
        a_id: usize,
        m_id: usize,
        r: f32,
        future_r: f32,
    ) {
        match self.backup_operator {
            BackupOperator::MonteCarlo => {
                self.update_action_node_monte_carlo(s_id, a_id, m_id, r, future_r)
            }
            BackupOperator::Max => self.update_action_node_max(s_id, a_id, m_id, r, future_r),
        }
    }

    fn update_action_node_monte_carlo(
        &mut self,
        s_id: usize,
        a_id: usize,
        m_id: usize,
        r: f32,
        future_r: f32,
    ) {
        let nv = r + future_r;
        let s_node = self.arena.get_node_mut(s_id);
        let a_node = &mut s_node.children[a_id];
        let m_node = &mut a_node.children[m_id];

        m_node.num_visited += 1;
        a_node.num_visited += 1;
        m_node.q = m_node.q + (nv - m_node.q) / m_node.num_visited as f32;
        a_node.v = a_node.v + (nv - a_node.v) / a_node.num_visited as f32;
    }

    fn update_action_node_max(
        &mut self,
        s_id: usize,
        a_id: usize,
        m_id: usize,
        r: f32,
        _future_r: f32,
    ) {
        let s_node = self.arena.get_node(s_id);
        let a_node = &s_node.children[a_id];
        let m_node = &a_node.children[m_id];

        let mut future = 0.0;
        let mut total_visited = 0;
        assert!(m_node.children.len() > 0);
        for ss_id in m_node.children.iter() {
            let v = self.arena.get_node(*ss_id).v;
            let num_visited = self.arena.get_node(*ss_id).num_visited;
            //             println!("{} {}", v, num_visited);
            if num_visited <= 0 {
                continue;
            }

            future += v * num_visited as f32;
            total_visited += num_visited;
        }
        assert!(total_visited > 0, "{:?}", m_node);

        let s_node = self.arena.get_node_mut(s_id);
        let a_node = &mut s_node.children[a_id];
        let m_node = &mut a_node.children[m_id];

        m_node.num_visited += 1;
        a_node.num_visited += 1;
        //         assert_eq!(total_visited, a_node.num_visited);

        future /= total_visited as f32;
        //         println!("{} {}", r, future);

        m_node.q = r + future;
        a_node.v = a_node.max_child();
    }
}
