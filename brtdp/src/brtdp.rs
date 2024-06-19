use core::fmt::Debug;
use core::hash::Hash;
use mdp::heuristic::{HeuristicWithMDP, HeuristicWithMDPMut};
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, GetNextStateMut, InitialState, IsTerminal, PMass,
    PMassMut, State, StatesActions,
};
use mdp::policy::policy_traits::{GetAction, GetActionMut};

use mdp::value_estimator::CostEstimator;
use mdp::value_iteration::ValueTable;
use rand::prelude::*;
use std::collections::HashSet;

pub struct BRTDP<S: State, H> {
    pub lb: ValueTable<S>,
    pub lb: ValueTable<S>,
    pub h: H,
    pub is_solved: HashSet<S>,
    pub max_t: usize,
}

// impl<M: StatesActions + PMass<f32> + Cost, H: HeuristicWithMDP<M>> CostEstimator<M>
//     for RTDP<M::State, H>
// {
//     fn get_value_ssp(&self, s: &M::State, mdp: &M) -> f32 {
//         self.vt.get_value(s).max(self.h.h_with(s, mdp))
//     }
//
//     fn get_qsa_ssp(&self, s: &M::State, a: &M::Action, mdp: &M) -> f32 {
//         mdp.p_mass(s, a)
//             .into_iter()
//             .map(|(ss, p)| self.get_value_ssp(&ss, mdp) * p)
//             .sum::<f32>()
//             + mdp.cost(s, a)
//     }
// }
//
// impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDP<S, H> {
//     pub fn get_value_ssp_mut<M>(&mut self, s: &M::State, mdp: &mut M) -> f32
//     where
//         M: StatesActions<State = S> + PMassMut<f32> + Cost,
//         H: HeuristicWithMDPMut<M>,
//     {
//         self.vt.get_value(s).max(self.h.h_with_mut(s, mdp))
//     }
//     pub fn get_qsa_ssp_mut<M>(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32
//     where
//         M: StatesActions<State = S> + PMassMut<f32> + Cost,
//         H: HeuristicWithMDPMut<M>,
//     {
//         mdp.p_mass_mut(s, a)
//             .into_iter()
//             .map(|(ss, p)| self.get_value_ssp_mut(&ss, mdp) * p)
//             .sum::<f32>()
//             + mdp.cost(s, a)
//     }
//
//     pub fn update<M>(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32
//     where
//         M: StatesActions<State = S> + PMassMut<f32> + Cost,
//         H: HeuristicWithMDPMut<M>,
//     {
//         let qsa = self.get_qsa_ssp_mut(&s, &a, mdp);
//         let residual = (self.get_value_ssp_mut(s, mdp) - qsa).abs();
//         self.vt.set_value(s, qsa);
//
//         residual
//     }
// }
//
