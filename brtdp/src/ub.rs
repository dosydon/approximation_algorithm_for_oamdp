use mdp::{
    heuristic::HeuristicWithMDPMut,
    mdp_traits::{ActionAvailability, ActionEnumerable, Cost, PMassMut, State, StatesActions},
};

use crate::{action_selection::ActionSelection, traits::UpperBoundWithMDPMut, BRTDP};

impl<S: State, H, U> BRTDP<S, H, U> {
    pub(crate) fn get_ub_mut<M>(&mut self, s: &M::State, mdp: &mut M) -> f32
    where
        M: StatesActions<State = S> + PMassMut<f32> + Cost,
        U: UpperBoundWithMDPMut<M>,
    {
        self.ub.get_value(s).min(self.u.u_with_mut(s, mdp))
    }

    pub(crate) fn get_ub_qsa_mut<M>(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32
    where
        M: StatesActions<State = S> + PMassMut<f32> + Cost,
        U: UpperBoundWithMDPMut<M>,
    {
        mdp.p_mass_mut(s, a)
            .into_iter()
            .map(|(ss, p)| self.get_ub_mut(&ss, mdp) * p)
            .sum::<f32>()
            + mdp.cost(s, a)
    }

    pub(crate) fn update_ub<M>(&mut self, s: &M::State, mdp: &mut M) -> f32
    where
        M: PMassMut<f32> + Cost + ActionEnumerable + ActionAvailability + StatesActions<State = S>,
        U: UpperBoundWithMDPMut<M>,
        H: HeuristicWithMDPMut<M>,
    {
        if let Some(a) = self.best_action_mut(s, mdp, ActionSelection::UB) {
            let qsa = self.get_ub_qsa_mut(&s, &a, mdp);
            let residual = (self.get_ub_mut(s, mdp) - qsa).abs();
            self.ub.set_value(s, qsa);
            residual
        } else {
            0.0
        }
    }
}
