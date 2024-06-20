use mdp::heuristic::HeuristicWithMDPMut;
use mdp::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, GetNextStateMut, InitialState, IsTerminal, PMass,
    PMassMut, State, StatesActions,
};
use mdp::policy::policy_traits::{GetAction, GetActionMut};

use crate::BRTDP;

impl<S: State, H, U> BRTDP<S, H, U> {
    pub(crate) fn get_lb_mut<M>(&mut self, s: &M::State, mdp: &mut M) -> f32
    where
        M: StatesActions<State = S> + PMassMut<f32> + Cost,
        H: HeuristicWithMDPMut<M>,
    {
        self.lb.get_value(s).max(self.h.h_with_mut(s, mdp))
    }

    pub(crate) fn get_lb_qsa_mut<M>(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32
    where
        M: StatesActions<State = S> + PMassMut<f32> + Cost,
        H: HeuristicWithMDPMut<M>,
    {
        mdp.p_mass_mut(s, a)
            .into_iter()
            .map(|(ss, p)| self.get_lb_mut(&ss, mdp) * p)
            .sum::<f32>()
            + mdp.cost(s, a)
    }

    pub(crate) fn update_lb<M>(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32
    where
        M: StatesActions<State = S> + PMassMut<f32> + Cost,
        H: HeuristicWithMDPMut<M>,
    {
        let qsa = self.get_lb_qsa_mut(&s, &a, mdp);
        let residual = (self.get_lb_mut(s, mdp) - qsa).abs();
        self.lb.set_value(s, qsa);
        //         println!("{:?}: {:?}", s, qsa);

        residual
    }
}
