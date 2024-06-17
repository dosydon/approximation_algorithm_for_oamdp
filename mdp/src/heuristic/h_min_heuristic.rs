use core::hash::Hash;
use std::fmt::Debug;

use crate::heuristic::HeuristicWithMDPMut;
use crate::mdp_traits::*;
use crate::state_queue::StateQueue;
use crate::value_iteration::ValueTable;
use std::collections::HashSet;

use super::HeuristicWithMDP;

pub struct HminHeuristic<S: PartialEq + Eq + Clone + Copy + Hash + Debug> {
    vt: ValueTable<S>,
    is_solved: HashSet<S>,
    is_disabled: bool,
}

impl<S: PartialEq + Eq + Clone + Hash + Copy + Debug> HminHeuristic<S> {
    pub fn new() -> HminHeuristic<S> {
        HminHeuristic {
            vt: ValueTable::new(0.0),
            is_solved: HashSet::new(),
            is_disabled: false,
        }
    }
    pub fn disable(mut self) -> HminHeuristic<S> {
        self.is_disabled = true;
        self
    }
}
impl<
        M: ActionAvailability + ActionEnumerable + Cost + PMassMut<f32> + StatesActions + IsTerminal,
    > HeuristicWithMDP<M> for HminHeuristic<M::State>
{
    fn h_with(&self, s: &M::State, _mdp: &M) -> f32 {
        let _err = 1e-3;
        if self.is_disabled {
            0.0
        } else {
            self.vt.get_value(s)
        }
    }
}

impl<
        M: ActionAvailability + ActionEnumerable + Cost + PMassMut<f32> + StatesActions + IsTerminal,
    > HeuristicWithMDPMut<M> for HminHeuristic<M::State>
{
    fn h_with_mut(&mut self, s: &M::State, mdp: &mut M) -> f32 {
        let err = 1e-3;
        if self.is_disabled {
            0.0
        } else {
            while !self.is_solved.contains(s) {
                let visited = self.trial(s, mdp);
                //                 println!("{}", visited.len());
                for i in 0..visited.len() {
                    self.check_solved(&visited[visited.len() - i - 1], mdp, err);
                }
            }
            self.vt.get_value(s)
        }
    }
}

impl<S: PartialEq + Eq + Clone + Hash + Copy + Debug> HminHeuristic<S> {
    fn successor<
        M: ActionEnumerable
            + ActionAvailability
            + Cost
            + PMassMut<f32>
            + IsTerminal
            + StatesActions<State = S>,
    >(
        &mut self,
        s: &M::State,
        mdp: &mut M,
    ) -> (M::State, f32) {
        let mut best_s = None;
        let mut best_v = 1e+6;
        for a_id in 0..mdp.num_actions() {
            let a = *mdp.id_to_action(a_id);
            if mdp.action_available(&s, &a) {
                for (ss, _p) in mdp.p_mass_mut(&s, &a) {
                    let v = mdp.cost(&s, &a) + self.vt.get_value(&ss);
                    if v < best_v {
                        best_v = v;
                        best_s = Some(ss);
                    }
                }
            }
        }
        (best_s.unwrap(), best_v)
    }

    fn trial<
        M: ActionEnumerable
            + ActionAvailability
            + Cost
            + PMassMut<f32>
            + StatesActions<State = S>
            + IsTerminal,
    >(
        &mut self,
        s: &M::State,
        mdp: &mut M,
    ) -> Vec<M::State> {
        let mut cs = *s;
        let mut max_residual = 0.0;
        let mut visited = vec![cs];

        while !mdp.is_terminal(&cs) {
            let (best_s, best_v) = self.successor(&cs, mdp);

            let residual = (self.vt.get_value(&cs) - best_v).abs();
            if residual > max_residual {
                max_residual = residual;
            }
            //             println!("{:?} {}", cs, best_v);
            self.vt.set_value(&cs, best_v);
            cs = best_s;
            visited.push(cs);
        }
        visited
    }

    fn check_solved<
        M: ActionEnumerable
            + ActionAvailability
            + Cost
            + PMassMut<f32>
            + StatesActions<State = S>
            + IsTerminal,
    >(
        &mut self,
        s: &M::State,
        mdp: &mut M,
        epsilon: f32,
    ) -> bool {
        let mut open = StateQueue::new();
        open.push(*s);
        let mut closed = StateQueue::new();
        let mut rv = true;

        while open.len() > 0 {
            if let Some(cs) = open.pop() {
                closed.push(cs);

                let (best_s, best_v) = self.successor(&cs, mdp);
                let residual = (self.vt.get_value(&cs) - best_v).abs();

                if residual > epsilon {
                    rv = false;
                    continue;
                }

                if !self.is_solved.contains(&best_s) && !closed.contains(&best_s) {
                    open.push(best_s);
                }
            }
        }

        if rv {
            for ss in closed {
                self.is_solved.insert(ss);
            }
        } else {
            for ss in closed {
                let (_best_s, best_v) = self.successor(&ss, mdp);
                self.vt.set_value(&ss, best_v);
            }
        }

        rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid_world::GridWorldMDP;
    use crate::value_iteration::value_iteration_ssp;

    #[test]
    fn test_hmin_heuristic() {
        let mut mdp = GridWorldMDP::default();
        let mut heuristic = HminHeuristic::new();
        let vt = value_iteration_ssp(&mdp);
        for s_id in 0..mdp.num_states() {
            let s = *mdp.id_to_state(s_id);
            let h = heuristic.h_with_mut(&s, &mut mdp);
            let v = vt.get_value(&s);
            assert!(h <= v);
            assert!(heuristic.check_solved(&s, &mut mdp, 1e-3));
        }
    }
}
