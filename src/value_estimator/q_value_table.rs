use crate::mdp_traits::*;
use crate::value_estimator::{CostEstimator, ValueEstimator};
use crate::value_iteration::ValueTable;
use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashMap;

pub struct QValueTable<
    S: Eq + PartialEq + Debug + Clone + Hash,
    A: Eq + PartialEq + Debug + Clone + Hash,
> {
    pub q_value_table: HashMap<(S, A), f32>,
    initial_value: f32,
}

impl<S: Eq + PartialEq + Debug + Clone + Hash, A: Eq + PartialEq + Debug + Clone + Hash>
    QValueTable<S, A>
{
    pub fn new(initial_value: f32) -> QValueTable<S, A> {
        QValueTable {
            q_value_table: HashMap::new(),
            initial_value: initial_value,
        }
    }

    pub fn from_value_table_ssp<M>(
        mdp: &M,
        vt: &ValueTable<M::State>,
    ) -> QValueTable<M::State, M::Action>
    where
        M: StatesActions<State = S, Action = A>
            + ActionAvailability
            + ActionEnumerable
            + PMass<f32>
            + Cost
            + StateEnumerable,
    {
        let mut table = HashMap::new();
        for s in mdp.enumerate_states() {
            for a in mdp.enumerate_actions() {
                if mdp.action_available(s, a) {
                    table.insert((s.clone(), a.clone()), vt.get_qsa_ssp(s, a, mdp));
                }
            }
        }
        QValueTable {
            q_value_table: table,
            initial_value: 0.0,
        }
    }

    pub fn from_value_table<M>(
        mdp: &M,
        vt: &ValueTable<M::State>,
    ) -> QValueTable<M::State, M::Action>
    where
        M: StatesActions<State = S, Action = A>
            + ActionAvailability
            + ActionEnumerable
            + PMass<f32>
            + Rsa
            + DiscountFactor
            + StateEnumerable,
    {
        let mut table = HashMap::new();
        for s in mdp.enumerate_states() {
            for a in mdp.enumerate_actions() {
                if mdp.action_available(s, a) {
                    table.insert((s.clone(), a.clone()), vt.get_qsa(s, a, mdp));
                }
            }
        }
        QValueTable {
            q_value_table: table,
            initial_value: 0.0,
        }
    }

    fn get_value(&self, s: &S, a: &A) -> f32 {
        if let Some(v) = self.q_value_table.get(&(s.clone(), a.clone())) {
            *v
        } else {
            self.initial_value
        }
    }

    pub fn td_error(&self, st: &S, at: &A, stt: &S, att: &A, r: f32, discount_factor: f32) -> f32 {
        let qvt = self.get_value(st, at);
        let qvtt = self.get_value(stt, att);
        r + discount_factor * qvtt - qvt
    }
}

impl<M: ActionAvailability + ActionEnumerable + PMass<f32> + Cost> CostEstimator<M>
    for QValueTable<M::State, M::Action>
{
    fn get_value_ssp(&self, s: &M::State, mdp: &M) -> f32 {
        mdp.enumerate_actions()
            .filter(|a| mdp.action_available(s, a))
            .map(|a| self.get_qsa_ssp(s, a, mdp))
            .fold(1. / 0., f32::min)
    }
    fn get_qsa_ssp(&self, s: &M::State, a: &M::Action, _mdp: &M) -> f32 {
        if let Some(v) = self.q_value_table.get(&(*s, *a)) {
            *v
        } else {
            self.initial_value
        }
    }
}

impl<
        S: Eq + PartialEq + Debug + Clone + Hash,
        A: Eq + PartialEq + Debug + Clone + Hash,
        M: ActionAvailability + ActionEnumerable + PMass<f32> + Rsa,
    > ValueEstimator<S, A, M> for QValueTable<S, A>
where
    M::Action: AsRef<A>,
{
    fn get_qsa(&self, s: &S, a: &A, _mdp: &M) -> f32 {
        self.get_value(s, a)
    }
    fn get_max_qsa(&self, s: &S, mdp: &M) -> f32 {
        mdp.enumerate_actions()
            .map(|a| self.get_qsa(s, a.as_ref(), mdp))
            .fold(-1. / 0., f32::max)
    }
}
