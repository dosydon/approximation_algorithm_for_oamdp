use core::f32::MAX;
use core::fmt::Debug;
use core::hash::Hash;
use log::info;
use mdp::arena::Arena;
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, InitialState, IsTerminal, PMassMut, StatesActions,
};
use mdp::policy::tabular_policy::TabularPolicy;
use multi_objective_mdp::cmdp_traits::{MultiCost, MultiDCost};
use multi_objective_mdp::MultiHeuristicWithMDPMut;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
struct AOStarOrNode<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> {
    pub(in crate::aostar) s: S,
    pub(in crate::aostar) id: usize,
    pub(in crate::aostar) children: Vec<Vec<(usize, f32)>>,
    pub(in crate::aostar) f: [f32; N],
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize> AOStarOrNode<S, N> {
    pub fn new(s: S, id: usize, f: [f32; N]) -> AOStarOrNode<S, N> {
        AOStarOrNode::<S, N> {
            s: s,
            id: id,
            children: vec![],
            f: f,
        }
    }
}

pub struct AOStarN<M: StatesActions, H, const N: usize> {
    pub mdp: M,
    pub table: HashMap<M::State, usize>,
    arena: Arena<AOStarOrNode<M::State, N>>,
    pub(in crate::aostar) heuristic: H,
    pub(in crate::aostar) visited: HashSet<usize>,
}

impl<
        M: InitialState + StatesActions + MultiDCost<N> + IsTerminal,
        H: MultiHeuristicWithMDPMut<M, N>,
        const N: usize,
    > AOStarN<M, H, N>
{
    pub fn new(mdp: M, heuristic: H) -> AOStarN<M, H, N> {
        let mut aostar = AOStarN {
            mdp: mdp,
            table: HashMap::new(),
            arena: Arena::new(),
            heuristic: heuristic,
            visited: HashSet::new(),
        };

        aostar.add_node(aostar.mdp.initial_state());
        aostar
    }
    pub(in crate::aostar) fn add_node(&mut self, s: M::State) -> usize {
        if self.table.contains_key(&s) {
            *self.table.get(&s).unwrap()
        } else {
            let next_id = self.arena.next_id();
            let mut f = [0.0; N];
            for i in 0..N {
                f[i] = self.heuristic.multi_h_with_mut(&s, &mut self.mdp, i);
            }

            self.arena.add_node(AOStarOrNode::new(s, next_id, f));
            //             println!("adding {:?} {:?}", s, self.heuristic.h(&s));
            self.table.insert(s, next_id);

            next_id
        }
    }

    pub(in crate::aostar) fn is_terminal(&self, s: &M::State) -> bool {
        self.mdp.is_terminal(s)
    }
    pub fn num_generated(&self) -> usize {
        self.arena.next_id()
    }
    pub fn root_f(&self) -> [f32; N] {
        self.arena.get_node(0).f
    }
}

impl<
        M: ActionAvailability
            + ActionEnumerable
            + InitialState
            + PMassMut<f32>
            + StatesActions
            + IsTerminal
            + MultiCost<N>
            + MultiDCost<N>,
        H: MultiHeuristicWithMDPMut<M, N>,
        const N: usize,
    > AOStarN<M, H, N>
{
    pub fn ilaostar(&mut self, epsilon: f32) {
        'outer: loop {
            loop {
                info!("{}", self.num_generated());
                //                 self.dump();
                self.visited.clear();
                if !self.expand_recursive(0) {
                    break;
                }
            }

            'inner: loop {
                self.visited.clear();
                let error = self.residual_error(0);
                if error < epsilon {
                    break 'outer;
                }
                if error == MAX {
                    break 'inner;
                }
            }
        }
    }
    pub(in crate::aostar) fn qsa(
        &self,
        id: usize,
        possible_outcomes: &Vec<(usize, f32)>,
        a: &M::Action,
        i: usize,
    ) -> f32 {
        let qsa = possible_outcomes
            .iter()
            .map(|(next_id, p)| self.arena.get_node(*next_id).f[i] * *p)
            .sum::<f32>()
            + self.mdp.multi_cost(&self.arena.get_node(id).s, a)[i];
        qsa
    }
    pub fn nodes_on_solution(&self) -> HashSet<usize> {
        let mut visited = HashSet::new();
        self.nodes_on_solution_inner(0, &mut visited);

        visited
    }
    fn nodes_on_solution_inner(&self, id: usize, visited: &mut HashSet<usize>) {
        if !visited.contains(&id) {
            visited.insert(id);
            if self.arena.get_node(id).children.len() > 0 {
                let and_node_id = self.most_promising_and_node(id).unwrap();
                for (c, _p) in self.arena.get_node(id).children[and_node_id].iter() {
                    self.nodes_on_solution_inner(*c, visited);
                }
            }
        }
    }
    pub fn count_on_solution(&self) -> usize {
        self.nodes_on_solution().len()
    }
    pub fn residual_error(&mut self, id: usize) -> f32 {
        if self.is_terminal(&self.arena.get_node(id).s) {
            0.0
        } else if self.visited.contains(&id) {
            0.0
        } else {
            self.visited.insert(id);
            let mut error = 0.0;

            if self.arena.get_node(id).children.len() == 0 {
                error = MAX;
            } else {
                if let Some(and_node_id) = self.most_promising_and_node(id) {
                    let cloned = self.arena.get_node(id).children[and_node_id].clone();
                    for (c, _p) in cloned.iter() {
                        error = error.max(self.residual_error(*c));
                    }

                    if !and_node_id == self.most_promising_and_node(id).unwrap() {
                        error = error.max(MAX);
                    }
                }
            }

            error = error.max(self.update(id));
            error
        }
    }
    pub fn aostar(&mut self) {
        loop {
            self.visited.clear();
            if !self.expand_recursive(0) {
                break;
            }
            //             println!("{:?}", self.arena.next_id());
        }
    }
    pub fn expand_node(&mut self, id: usize) -> bool {
        unsafe {
            let self_p = self as *mut Self;
            for a in self.mdp.enumerate_actions() {
                if self.mdp.action_available(&self.arena.get_node(id).s, a) {
                    let mut for_a = vec![];
                    for (s, p) in (*self_p).mdp.p_mass_mut(&self.arena.get_node(id).s, a) {
                        let new_id = (*self_p).add_node(s);
                        for_a.push((new_id, p));
                    }
                    (*self_p).arena.get_node_mut(id).children.push(for_a);
                }
            }
        }
        self.mdp
            .enumerate_actions()
            .any(|a| self.mdp.action_available(&self.arena.get_node(id).s, a))
    }
    fn expand_recursive(&mut self, id: usize) -> bool {
        //         println!("{:?}", id);
        if self.is_terminal(&self.arena.get_node(id).s) {
            false
        } else if self.visited.contains(&id) {
            false
        } else {
            self.visited.insert(id);
            let mut flag = false;

            if self.arena.get_node(id).children.len() == 0 {
                self.expand_node(id);
                flag = true;
                self.update(id);
            } else {
                if let Some(and_node_id) = self.most_promising_and_node(id) {
                    unsafe {
                        let self_p = self as *mut Self;
                        for (c, _p) in (*self_p).arena.get_node(id).children[and_node_id].iter() {
                            flag |= self.expand_recursive(self.arena.get_node(*c).id);
                        }
                    }
                    self.update(id);
                    if Some(and_node_id) != self.most_promising_and_node(id) {
                        flag |= true;
                    }
                } else {
                    self.update(id);
                    //                     println!("something weird");
                }
            }

            flag
        }
    }
    fn update(&mut self, id: usize) -> f32 {
        if self
            .mdp
            .enumerate_actions()
            .all(|a| !self.mdp.action_available(&self.arena.get_node(id).s, a))
        {
            self.arena.get_node_mut(id).f = [MAX; N];
            MAX
        } else if self.arena.get_node(id).children.len() > 0 {
            let prev = self.arena.get_node_mut(id).f;
            for i in 0..N {
                self.arena.get_node_mut(id).f[i] = self
                    .mdp
                    .enumerate_actions()
                    .filter(|a| self.mdp.action_available(&self.arena.get_node(id).s, a))
                    .zip(self.arena.get_node(id).children.iter())
                    .map(|(a, possible_outcomes)| self.qsa(id, possible_outcomes, a, i))
                    .fold(1. / 0., f32::min);
            }
            //             println!("updating {:?} -> {:?}", self.arena.get_node(id), prev);
            (self.arena.get_node_mut(id).f[0] - prev[0]).abs()
        } else {
            0.0
        }
    }
    fn most_promising_and_node(&self, id: usize) -> Option<usize> {
        let mut cur_min = MAX;
        let mut cur_min_id = None;
        for (aid, a) in self
            .mdp
            .enumerate_actions()
            .filter(|a| self.mdp.action_available(&self.arena.get_node(id).s, a))
            .enumerate()
        {
            if self.qsa(id, &self.arena.get_node(id).children[aid], a, 0) < cur_min {
                cur_min = self.qsa(id, &self.arena.get_node(id).children[aid], a, 0);
                cur_min_id = Some(aid);
            }
        }

        cur_min_id
    }
    pub fn to_policy(&self) -> TabularPolicy<M::State, M::Action> {
        let mut table = HashMap::new();
        for id in 0..self.arena.next_id() {
            if self.arena.get_node(id).children.len() > 0 {
                let and_node_id = self.most_promising_and_node(id).unwrap();
                table.insert(
                    self.arena.get_node(id).s,
                    *self
                        .mdp
                        .enumerate_actions()
                        .filter(|a| self.mdp.action_available(&self.arena.get_node(id).s, a))
                        .nth(and_node_id)
                        .unwrap(),
                );
            }
        }

        TabularPolicy::new(table)
    }
}

impl<
        M: ActionAvailability
            + ActionEnumerable
            + InitialState
            + PMassMut<f32>
            + StatesActions
            + MultiCost<N>
            + MultiDCost<N>,
        H: MultiHeuristicWithMDPMut<M, N>,
        const N: usize,
    > MultiHeuristicWithMDPMut<M, N> for AOStarN<M, H, N>
{
    fn multi_h_with_mut(&mut self, s: &M::State, mdp: &mut M, i: usize) -> f32 {
        if let Some(id) = self.table.get(s) {
            self.arena.get_node(*id).f[i]
        } else {
            self.heuristic.multi_h_with_mut(s, mdp, i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /* use assert_approx_eq::assert_approx_eq; */
    use mdp::grid_world::{GridWorldMDP, GridWorldState};
    use mdp::heuristic::ZeroHeuristic;

    #[test]
    fn test_ilaostar_multi_objective() {
        let mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let zero_heuristic = ZeroHeuristic {};
        let mut aostar = AOStarN::new(mdp, zero_heuristic);
        let err = 1e-3;
        aostar.ilaostar(err);
        //         assert_approx_eq!(aostar.root_f()[0], 7.558234211, err);
        println!("{:?}", aostar.root_f());
    }
}
