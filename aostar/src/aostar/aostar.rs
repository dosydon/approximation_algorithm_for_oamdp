use core::f32::MAX;
use core::fmt::Debug;
use core::hash::Hash;
use itertools::Itertools;
use log::info;
use mdp::arena::Arena;
use mdp::heuristic::HeuristicWithMDPMut;
use mdp::into_inner::Inner;
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, InitialState, IsTerminal, PMassMut, StatesActions,
};
// use mdp::policy::closest_point_policy::ClosestPointPolicy;
use mdp::policy::tabular_policy::TabularPolicy;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
pub struct AOStarOrNode<S: Eq + PartialEq + Debug + Copy + Clone + Hash> {
    pub(in crate::aostar) s: S,
    pub(in crate::aostar) id: usize,
    pub(in crate::aostar) children: Vec<Vec<(usize, f32)>>,
    pub(in crate::aostar) f: f32,
}

impl<S: Eq + PartialEq + Debug + Copy + Clone + Hash> AOStarOrNode<S> {
    pub fn new(s: S, id: usize, f: f32) -> AOStarOrNode<S> {
        AOStarOrNode {
            s: s,
            id: id,
            children: vec![],
            f: f,
        }
    }
}

pub struct AOStar<M: StatesActions, H: HeuristicWithMDPMut<M>> {
    pub mdp: M,
    pub table: HashMap<M::State, usize>,
    pub(in crate::aostar) arena: Arena<AOStarOrNode<M::State>>,
    pub(in crate::aostar) heuristic: H,
    pub(in crate::aostar) visited: HashSet<usize>,
}

impl<
        M: ActionEnumerable + InitialState + PMassMut<f32> + StatesActions + IsTerminal,
        H: HeuristicWithMDPMut<M>,
    > AOStar<M, H>
{
    pub fn new(mdp: M, heuristic: H) -> AOStar<M, H> {
        let mut aostar = AOStar {
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

            self.arena.add_node(AOStarOrNode::new(
                s,
                next_id,
                self.heuristic.h_with_mut(&s, &mut self.mdp),
            ));
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
    pub fn root_f(&self) -> f32 {
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
            + Cost,
        H: HeuristicWithMDPMut<M>,
    > AOStar<M, H>
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
    ) -> f32 {
        let qsa = possible_outcomes
            .iter()
            .map(|(next_id, p)| self.arena.get_node(*next_id).f * *p)
            .sum::<f32>()
            + self.mdp.cost(&self.arena.get_node(id).s, a);
        //         println!("{:?} {:?}", possible_outcomes, qsa);
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
            self.arena.get_node_mut(id).f = MAX;
            MAX
        } else if self.arena.get_node(id).children.len() > 0 {
            let prev = self.arena.get_node_mut(id).f;
            self.arena.get_node_mut(id).f = self
                .mdp
                .enumerate_actions()
                .filter(|a| self.mdp.action_available(&self.arena.get_node(id).s, a))
                .zip(self.arena.get_node(id).children.iter())
                .map(|(a, possible_outcomes)| self.qsa(id, possible_outcomes, a))
                .fold(1. / 0., f32::min);
            //             println!("updating {:?} -> {:?}", self.arena.get_node(id), prev);
            (self.arena.get_node_mut(id).f - prev).abs()
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
            if self.qsa(id, &self.arena.get_node(id).children[aid], a) < cur_min {
                cur_min = self.qsa(id, &self.arena.get_node(id).children[aid], a);
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

    //     pub fn to_closest_point_policy(&self) -> ClosestPointPolicy<M> {
    //         let mut table = HashMap::new();
    //         for id in 0..self.arena.next_id() {
    //             if self.arena.get_node(id).children.len() > 0 {
    //                 let and_node_id = self.most_promising_and_node(id).unwrap();
    //                 table.insert(
    //                     self.arena.get_node(id).s,
    //                     *self
    //                         .mdp
    //                         .enumerate_actions()
    //                         .filter(|a| self.mdp.action_available(&self.arena.get_node(id).s, a))
    //                         .nth(and_node_id)
    //                         .unwrap(),
    //                 );
    //             }
    //         }
    //
    //         ClosestPointPolicy::new(table)
    //     }
    pub fn dump(&self) {
        for (i, vec) in self.arena.nodes.iter().enumerate() {
            println!("{:?}", vec);
            if vec.children.len() > 0 {
                for (a, possible_outcomes) in self
                    .mdp
                    .enumerate_actions()
                    .filter(|a| self.mdp.action_available(&self.arena.get_node(i).s, a))
                    .zip(self.arena.get_node(i).children.iter())
                {
                    println!("{:?} {:?}", a, self.qsa(i, possible_outcomes, a));
                }
            }
        }
    }
}

impl<
        M: ActionAvailability
            + ActionEnumerable
            + InitialState
            + PMassMut<f32>
            + StatesActions
            + IsTerminal
            + Cost,
        H: HeuristicWithMDPMut<M>,
    > AOStar<M, H>
where
    M::State: Inner,
    <M::State as Inner>::Result: Hash + Eq + Debug,
{
    pub fn dump_on_solution(&self) {
        let nodes_on_solution = self.nodes_on_solution();
        let hashmap = nodes_on_solution
            .into_iter()
            .map(|id| ((self.arena.get_node(id).s.inner()), id))
            .into_group_map();
        for key in hashmap.keys() {
            println!("{:?}", key);
            for id in hashmap.get(key).unwrap().iter() {
                println!("{:?}", self.arena.get_node(*id));
                //
                if self.arena.get_node(*id).children.len() > 0 {
                    for (a, possible_outcomes) in self
                        .mdp
                        .enumerate_actions()
                        .zip(self.arena.get_node(*id).children.iter())
                    {
                        println!("{:?} {:?}", a, self.qsa(*id, possible_outcomes, a));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
    use mdp::grid_world::{GridWorldMDP, GridWorldState};
    use mdp::heuristic::ZeroHeuristic;

    #[test]
    fn test_arena_new() {
        let mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let finite_horizon_mdp = FiniteHorizonWrapper::new(mdp, 9);
        let zero_heuristic = ZeroHeuristic {};
        let mut aostar = AOStar::new(finite_horizon_mdp, zero_heuristic);
        let err = 1e-3;
        aostar.aostar();
        assert_approx_eq!(aostar.root_f(), 7.31875, err);
        assert_approx_eq!(aostar.residual_error(0), 0.0, err);
        assert_eq!(112, aostar.count_on_solution());
        assert_eq!(112, aostar.num_generated());
    }

    #[test]
    fn test_aostar_short_horizon() {
        let mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let finite_horizon_mdp = FiniteHorizonWrapper::new(mdp, 4);
        let zero_heuristic = ZeroHeuristic {};
        let mut aostar = AOStar::new(finite_horizon_mdp, zero_heuristic);
        let err = 1e-3;
        aostar.aostar();
        assert_approx_eq!(aostar.root_f(), 4.0, err);
        assert_approx_eq!(aostar.residual_error(0), 0.0, err);
    }

    #[test]
    fn test_ilaostar_finite_horizon() {
        let mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let finite_horizon_mdp = FiniteHorizonWrapper::new(mdp, 9);
        let zero_heuristic = ZeroHeuristic {};
        let mut aostar = AOStar::new(finite_horizon_mdp, zero_heuristic);
        let err = 1e-3;
        aostar.ilaostar(err);
        assert_approx_eq!(aostar.root_f(), 7.31875, err);
        assert_approx_eq!(aostar.residual_error(0), 0.0, err);
    }

    #[test]
    fn test_ilaostar_indefinite_horizon() {
        let mdp = GridWorldMDP::new(
            4,
            4,
            GridWorldState::new(0, 0),
            GridWorldState::new(3, 3),
            vec![GridWorldState::new(2, 3)],
            vec![],
        );
        let zero_heuristic = ZeroHeuristic {};
        let mut aostar = AOStar::new(mdp, zero_heuristic);
        let err = 1e-3;
        aostar.ilaostar(err);
        assert_approx_eq!(aostar.root_f(), 7.558234211, err);
        assert_approx_eq!(aostar.residual_error(0), 0.0, err);
    }
}
