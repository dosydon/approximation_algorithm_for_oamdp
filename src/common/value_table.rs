use crate::heuristic::HeuristicWithMDP;
use crate::mdp_traits::*;
use crate::value_estimator::{CostEstimator, ValueEstimator};
use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ValueTable<S: Eq + PartialEq + Debug + Clone + Hash> {
    pub value_table: HashMap<S, f32>,
    initial_value: f32,
}

impl<S: Eq + PartialEq + Debug + Clone + Hash + Copy> ValueTable<S> {
    pub fn new(initial_value: f32) -> ValueTable<S> {
        let table = ValueTable {
            value_table: HashMap::new(),
            initial_value: initial_value,
        };

        table
    }
    pub fn get_value(&self, s: &S) -> f32 {
        if let Some(v) = self.value_table.get(s) {
            return *v;
        } else {
            return self.initial_value;
        }
    }
    pub fn set_value(&mut self, s: &S, value: f32) {
        self.value_table.insert(*s, value);
    }
}

impl<M: StateEnumerable + ActionEnumerable + PMass<f32> + Rsa + DiscountFactor>
    ValueEstimator<M::State, M::Action, M> for ValueTable<M::State>
{
    fn get_qsa(&self, s: &M::State, a: &M::Action, mdp: &M) -> f32 {
        mdp.p_mass(s, a)
            .into_iter()
            .map(|(ss, p)| self.get_value(&ss) * p * mdp.get_discount_factor())
            .sum::<f32>()
            + mdp.rsa(s, a)
    }
    fn get_max_qsa(&self, s: &M::State, mdp: &M) -> f32 {
        mdp.enumerate_actions()
            .map(|a| self.get_qsa(s, a, mdp))
            .fold(0. / 0., f32::max)
    }
}

impl<M: ActionAvailability + ActionEnumerable + PMass<f32> + Cost> CostEstimator<M>
    for ValueTable<M::State>
{
    fn get_value_ssp(&self, s: &M::State, mdp: &M) -> f32 {
        mdp.enumerate_actions()
            .filter(|a| mdp.action_available(s, a))
            .map(|a| self.get_qsa_ssp(s, a, mdp))
            .fold(1. / 0., f32::min)
    }
    fn get_qsa_ssp(&self, s: &M::State, a: &M::Action, mdp: &M) -> f32 {
        mdp.p_mass(s, a)
            .into_iter()
            .map(|(ss, p)| self.get_value(&ss) * p)
            .sum::<f32>()
            + mdp.cost(s, a)
    }
}

impl<S: Eq + PartialEq + Debug + Clone + Hash> ValueTable<S> {
    pub fn get_greedy_action_ssp<M: ActionAvailability + ActionEnumerable + PMass<f32> + Cost>(
        &self,
        s: &M::State,
        mdp: &M,
    ) -> M::Action
    where
        M: StatesActions<State = S>,
    {
        let mut action = *(mdp.enumerate_actions().nth(0).unwrap());
        let mut current_best = 1e+12;
        for a in mdp.enumerate_actions() {
            if current_best >= self.get_qsa_ssp(s, a, mdp) {
                action = *a;
                current_best = self.get_qsa_ssp(s, a, mdp);
            }
        }

        return action;
    }
}

impl<M: ActionAvailability + ActionEnumerable + PMass<f32> + Cost> HeuristicWithMDP<M>
    for ValueTable<M::State>
{
    fn h_with(&self, s: &<M as StatesActions>::State, _mdp: &M) -> f32 {
        self.get_value(s)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::tiger::*;
//
//     #[test]
//     fn test_value_table() {
//         let mdp = TigerProblem {};
//         let mut value_table = ValueTable::new(&mdp);
//         value_table.set_value(TigerState::TigerInLeft, 1.0, &mdp);
//         assert_eq!(1.0, value_table.get_value(TigerState::TigerInLeft, &mdp));
//     }
// }
