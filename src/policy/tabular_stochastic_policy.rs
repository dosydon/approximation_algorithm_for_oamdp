use crate::mdp_traits::*;
use crate::policy::policy_traits::*;
use core::fmt::Debug;
use rand::prelude::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::tabular_policy::TabularPolicy;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TabularStochasticPolicy<M: StatesActions> {
    #[serde(bound(
        serialize = "HashMap<(M::State, M::Action), f32>: Serialize",
        deserialize = "HashMap<(M::State, M::Action), f32>: Deserialize<'de>"
    ))]
    pub table: HashMap<(M::State, M::Action), f32>,
}

impl<M: StatesActions> TabularStochasticPolicy<M> {
    pub fn new(table: HashMap<(M::State, M::Action), f32>) -> TabularStochasticPolicy<M> {
        TabularStochasticPolicy { table: table }
    }
}

impl<M: ActionEnumerable + StatesActions> GetAction<M::State, M> for TabularStochasticPolicy<M> {
    fn get_action(&self, s: &M::State, mdp: &M, rng: &mut ThreadRng) -> Option<M::Action> {
        let candidates = mdp
            .enumerate_actions()
            .map(|a| (*a, self.table.get(&(*s, *a)).unwrap()))
            .collect::<Vec<_>>();

        Some(candidates.choose_weighted(rng, |item| item.1).unwrap().0)
    }
}

impl<M: ActionEnumerable + StateEnumerable + StatesActions> TabularStochasticPolicy<M> {
    pub fn determinize(&self, mdp: &M) -> TabularPolicy<M::State, M::Action> {
        let mut table = HashMap::new();
        for s in mdp.enumerate_states() {
            let mut action = None;
            let mut current_best = 0.0;
            for a in mdp.enumerate_actions() {
                if let Some(p) = self.table.get(&(*s, *a)) {
                    if p >= &current_best {
                        current_best = *self.table.get(&(*s, *a)).unwrap();
                        action = Some(*a)
                    }
                } else {
                    println!("{:?} {:?}", s, a);
                }
            }

            table.insert(*s, action.unwrap());
        }

        TabularPolicy::new(table)
    }
}
