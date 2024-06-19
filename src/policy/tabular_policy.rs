use crate::common::value_table::ValueTable;
use crate::mdp_traits::*;
use crate::policy::policy_traits::*;
use crate::value_estimator::CostEstimator;
use crate::value_estimator::QValueTable;
use crate::value_estimator::ValueEstimator;
use core::fmt::Debug;
use core::hash::Hash;
use num_traits::cast::FromPrimitive;
use ordered_float::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::private_traits::GetActionMutFrom;
use super::private_traits::Sealed;

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq)]
pub struct TabularPolicy<
    S: Eq + PartialEq + Debug + Copy + Clone + Hash,
    A: Eq + PartialEq + Debug + Copy + Clone + Hash,
> {
    #[serde(bound(
        serialize = "HashMap<S, A>: Serialize",
        deserialize = "HashMap<S, A>: Deserialize<'de>"
    ))]
    pub table: HashMap<S, A>,
}

impl<
        S: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > TabularPolicy<S, A>
{
    pub fn get_action_(&self, s: &S) -> Option<A> {
        self.table.get(s).map(|a| *a)
    }
}

impl<
        S: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > TabularPolicy<S, A>
{
    pub fn new(table: HashMap<S, A>) -> TabularPolicy<S, A> {
        TabularPolicy { table: table }
    }
}

impl<
        M,
        S: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > GetActionProbabilityMaybe<M> for TabularPolicy<S, A>
where
    M: StatesActions<State = S, Action = A>,
{
    fn get_action_probability_maybe(&self, s: &M::State, a: &M::Action, _mdp: &M) -> Option<f32> {
        if let Some(best) = self.table.get(s) {
            if best == a {
                Some(1.0)
            } else {
                Some(0.0)
            }
        } else {
            None
        }
    }
}

impl<M: StatesActions> GetAction<M::State, M> for TabularPolicy<M::State, M::Action> {
    fn get_action(&self, s: &M::State, _mdp: &M, _rng: &mut ThreadRng) -> Option<M::Action> {
        self.table.get(s).map(|a| *a)
    }
}

impl<M: StatesActions> GetActionProbabilityMut<M::Action, M>
    for TabularPolicy<M::State, M::Action>
{
    fn get_action_probability_mut(
        &mut self,
        s: &<M as StatesActions>::State,
        a: &M::Action,
        _mdp: &mut M,
    ) -> f32 {
        if let Some(best) = self.table.get(s) {
            if best == a {
                1.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

impl<
        S: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > Sealed for TabularPolicy<S, A>
{
}
impl<M: StatesActions> GetActionMutFrom<M::State, M> for TabularPolicy<M::State, M::Action> {}

impl<
        S: Eq + PartialEq + Debug + Copy + Clone + Hash,
        A: Eq + PartialEq + Debug + Copy + Clone + Hash,
    > TabularPolicy<S, A>
{
    pub fn from_value_table<M>(mdp: &M, vt: &ValueTable<M::State>) -> TabularPolicy<S, A>
    where
        M: StatesActions<State = S, Action = A>
            + ActionAvailability
            + StateEnumerable
            + ActionEnumerable
            + PMass<f32>
            + DiscountFactor
            + Rsa,
    {
        let mut table = HashMap::new();
        for s in mdp.enumerate_states() {
            let maxa = *mdp
                .enumerate_actions()
                .filter(|a| mdp.action_available(s, a))
                .max_by_key(|a| NotNan::<f32>::from_f32(vt.get_qsa(s, *a, mdp)))
                .unwrap();
            assert!(mdp.action_available(s, &maxa));
            table.insert(*s, maxa);
        }
        TabularPolicy { table: table }
    }

    pub fn from_value_table_ssp<M>(
        mdp: &M,
        vt: &ValueTable<M::State>,
    ) -> TabularPolicy<M::State, M::Action>
    where
        M: StatesActions<State = S, Action = A>
            + ActionAvailability
            + ActionEnumerable
            + StateEnumerable
            + PMass<f32>
            + Cost,
    {
        let mut table = HashMap::new();
        for s in mdp.enumerate_states() {
            let mina = *mdp
                .enumerate_actions()
                .filter(|a| mdp.action_available(s, a))
                .min_by_key(|a| NotNan::<f32>::from_f32(vt.get_qsa_ssp(s, *a, mdp)))
                .unwrap();
            table.insert(*s, mina);
        }
        TabularPolicy { table: table }
    }

    pub fn from_q_value_table_ssp<M>(
        mdp: &M,
        qvt: &QValueTable<M::State, M::Action>,
    ) -> TabularPolicy<M::State, M::Action>
    where
        M: StatesActions<State = S, Action = A>
            + ActionAvailability
            + StateEnumerable
            + ActionEnumerable
            + PMass<f32>
            + Cost,
    {
        let mut table = HashMap::new();
        for s in mdp.enumerate_states() {
            if let Some(mina) = mdp
                .enumerate_actions()
                .filter(|a| mdp.action_available(s, a))
                .min_by_key(|a| NotNan::<f32>::from_f32(qvt.get_qsa_ssp(s, *a, mdp)))
            {
                table.insert(*s, *mina);
            } else {
                panic!("{:?}", s);
            }
        }
        TabularPolicy { table: table }
    }

    pub fn from_q_value_table<M>(mdp: &M, qvt: &QValueTable<S, A>) -> TabularPolicy<S, A>
    where
        M: StatesActions + ActionAvailability + ActionEnumerable + PMass<f32> + Rsa,
        S: AsRef<S>,
        A: AsRef<A>,
        M::State: AsRef<S>,
        M::Action: AsRef<A>,
    {
        let mut table = HashMap::new();
        for (s, _aa) in qvt.q_value_table.keys() {
            if !table.contains_key(s) {
                if let Some(mina) = mdp.enumerate_actions().max_by_key(|a| {
                    NotNan::<f32>::from_f32(qvt.get_qsa(s.as_ref(), a.as_ref(), mdp))
                }) {
                    table.insert(*s, *mina.as_ref());
                } else {
                    panic!("{:?}", s);
                }
            }
        }
        TabularPolicy { table: table }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baker_grid::*;
    use crate::value_iteration::value_iteration_ssp;

    #[test]
    fn test_serde_tabular_policy() {
        let mdp = BakerGridMDP::new(3, 3, vec![], BakerGridState::new(0, 2));
        let value_table = value_iteration_ssp(&mdp);
        let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
        let serialized = serde_yaml::to_string(&tabular_policy).unwrap();
        //         println!("{:?}", serialized);
        let deserialized: TabularPolicy<BakerGridState, BakerGridAction> =
            serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(tabular_policy, deserialized);
    }
}
