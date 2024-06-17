use crate::mdp_traits::*;
use crate::policy::policy_traits::{PartialPolicy, Policy};
use core::fmt::Debug;
use rand::prelude::*;
use std::collections::HashMap;

use super::policy_traits::GetAction;
use core::hash::Hash;

#[derive(Debug)]
pub struct StochasticTabularPolicy<
    S: Eq + PartialEq + Debug + Copy + Clone + Hash,
    A: Eq + PartialEq + Debug + Copy + Clone + Hash,
> {
    pub table: HashMap<(S, A), f32>,
}

impl<
        S: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > StochasticTabularPolicy<S, A>
{
    pub fn new(table: HashMap<(S, A), f32>) -> StochasticTabularPolicy<S, A> {
        StochasticTabularPolicy { table }
    }
}

impl<M: StatesActions> Policy<M::Action, M> for StochasticTabularPolicy<M::State, M::Action> {
    fn get_probability(&self, s: &M::State, a: &M::Action, _mdp: &M) -> f32 {
        *self.table.get(&(*s, *a)).unwrap()
    }
}

impl<M: StatesActions> PartialPolicy<M> for StochasticTabularPolicy<M::State, M::Action> {
    fn get_probability_maybe(&self, s: &M::State, a: &M::Action, _mdp: &M) -> Option<f32> {
        match self.table.get(&(*s, *a)) {
            Some(p) => Some(*p),
            None => None,
        }
    }
}

impl<M: StatesActions + ActionEnumerable> GetAction<M::State, M>
    for StochasticTabularPolicy<M::State, M::Action>
{
    fn get_action(&self, s: &M::State, mdp: &M, rng: &mut ThreadRng) -> Option<M::Action> {
        let possibilities = mdp
            .enumerate_actions()
            .map(|a| (*a, *self.table.get(&(*s, *a)).unwrap()))
            .collect::<Vec<_>>();
        match possibilities.choose_weighted(rng, |item| item.1) {
            Ok(action) => Some(action.0),
            Err(err) => {
                panic!("{:?} {:?}", err, possibilities);
            }
        }
    }
}
