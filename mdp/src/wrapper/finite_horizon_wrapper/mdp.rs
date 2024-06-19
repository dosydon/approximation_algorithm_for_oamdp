use crate::finite_horizon_wrapper::FiniteHorizonWrapperState;
use crate::into_inner::{IntoInner, IntoInnerMost};
use crate::mdp_traits::*;

use core::fmt::Debug;
use core::slice::Iter;
use mdp_derive::ActionEnumerable;

#[derive(PartialEq, Debug, Clone, ActionEnumerable)]
pub struct FiniteHorizonWrapper<M: StatesActions> {
    pub mdp: M,
    max_t: usize,
}

impl<M: StatesActions> FiniteHorizonWrapper<M> {
    pub fn new(mdp: M, max_t: usize) -> FiniteHorizonWrapper<M> {
        FiniteHorizonWrapper {
            mdp: mdp,
            max_t: max_t,
        }
    }
}

impl<'a, M: StatesActions> IntoInner for &'a FiniteHorizonWrapper<M> {
    type IntoInnerResult = &'a M;
    fn into_inner(self) -> &'a M {
        &self.mdp
    }
}

impl<'a, M: StatesActions> IntoInnerMost for &'a FiniteHorizonWrapper<M>
where
    &'a M: IntoInner,
{
    type IntoInnerMostResult = <&'a M as IntoInner>::IntoInnerResult;

    fn into_inner_most(self) -> Self::IntoInnerMostResult {
        self.into_inner().into_inner()
    }
}

impl<M: GetNextState> GetNextState for FiniteHorizonWrapper<M> {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Self::State {
        let inner = self.mdp.get_next_state(&s.s, a, rng);
        FiniteHorizonWrapperState::new(inner, s.t + 1)
    }
}

impl<M: GetNextStateMut> GetNextStateMut for FiniteHorizonWrapper<M> {
    fn get_next_state_mut(
        &mut self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Self::State {
        let inner = self.mdp.get_next_state_mut(&s.s, a, rng);
        FiniteHorizonWrapperState::new(inner, s.t + 1)
    }
}

impl<M: ActionAvailability + StatesActions> ActionAvailability for FiniteHorizonWrapper<M> {
    fn action_available(&self, s: &Self::State, a: &Self::Action) -> bool {
        self.mdp.action_available(&s.s, a)
    }
}

impl<M: StatesActions> StatesActions for FiniteHorizonWrapper<M> {
    type State = FiniteHorizonWrapperState<M::State>;
    type Action = M::Action;
}

impl<M: IsTerminal> IsTerminal for FiniteHorizonWrapper<M> {
    fn is_terminal(&self, s: &Self::State) -> bool {
        if s.t == self.max_t {
            true
        } else {
            self.mdp.is_terminal(&s.s)
        }
    }
}

impl<M: InitialState> InitialState for FiniteHorizonWrapper<M> {
    fn initial_state(&self) -> Self::State {
        FiniteHorizonWrapperState::new(self.mdp.initial_state(), 0)
    }
}

impl<M: PMass<f32>> PMass<f32> for FiniteHorizonWrapper<M> {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        self.mdp
            .p_mass(&s.s, a)
            .into_iter()
            .map(|(underlying_state, p)| {
                (FiniteHorizonWrapperState::new(underlying_state, s.t + 1), p)
            })
            .collect::<Vec<_>>()
    }
}

impl<M: PMassMut<f32>> PMassMut<f32> for FiniteHorizonWrapper<M> {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        self.mdp
            .p_mass_mut(&s.s, a)
            .into_iter()
            .map(|(underlying_state, p)| {
                (FiniteHorizonWrapperState::new(underlying_state, s.t + 1), p)
            })
            .collect::<Vec<_>>()
    }
}

impl<M: PreferredSuccessor> PreferredSuccessor for FiniteHorizonWrapper<M> {
    fn preferred_successor(&self, s: &Self::State, a: &Self::Action) -> Self::State {
        FiniteHorizonWrapperState::new(self.mdp.preferred_successor(&s.s, a), s.t + 1)
    }
}

impl<M: ExplicitTransition> ExplicitTransition for FiniteHorizonWrapper<M> {
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        if st.t + 1 == stt.t {
            self.mdp.p(&st.s, a, &stt.s)
        } else {
            0.0
        }
    }
}

impl<M: Rsa> Rsa for FiniteHorizonWrapper<M> {
    fn rsa(&self, st: &Self::State, at: &Self::Action) -> f32 {
        self.mdp.rsa(&st.s, at)
    }
}

impl<M: DiscountFactor + StatesActions> DiscountFactor for FiniteHorizonWrapper<M> {
    fn get_discount_factor(&self) -> f32 {
        1.0
    }
}

impl<M: Cost> Cost for FiniteHorizonWrapper<M> {
    fn cost(&self, st: &Self::State, at: &Self::Action) -> f32 {
        self.mdp.cost(&st.s, at)
    }
}

impl<M: DCost> DCost for FiniteHorizonWrapper<M> {
    fn d_cost(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        self.mdp.d_cost(&st.s, a, &stt.s)
    }
}

impl<M: StatesActions + SampleInitialState> SampleInitialState for FiniteHorizonWrapper<M> {
    fn sample_initial_state(&self, rng: &mut rand::rngs::ThreadRng) -> Self::State {
        FiniteHorizonWrapperState::new(self.mdp.sample_initial_state(rng), 0)
    }
}

impl<M: Rsas> Rsas for FiniteHorizonWrapper<M> {
    fn rsas(&self, s: &Self::State, a: &Self::Action, ss: &Self::State) -> f32 {
        self.mdp.rsas(&s.s, a, &ss.s)
    }
}

impl<M: StatesActions + DisplayState<M::State>> DisplayState<FiniteHorizonWrapperState<M::State>>
    for FiniteHorizonWrapper<M>
{
    fn display(&self, s: &FiniteHorizonWrapperState<M::State>) {
        self.mdp.display(&s.s);
    }
}

impl<M: RenderTo> RenderTo for FiniteHorizonWrapper<M> {
    fn render_to(&self, s: &Self::State, path: &str) {
        self.mdp.render_to(&s.s, path);
    }
}

// impl<P: SimulatablePolicy<M::State, M>, M: StatesActions>
//     SimulatablePolicy<FiniteHorizonWrapperState<M::State>, FiniteHorizonWrapper<M>> for P
// {
//     fn get_action(
//         &self,
//         s: &FiniteHorizonWrapperState<M::State>,
//         mdp: &FiniteHorizonWrapper<M>,
//         rng: &mut rand::rngs::ThreadRng,
//     ) -> Option<<FiniteHorizonWrapper<M> as StatesActions>::Action> {
//         self.get_action(&s.s, &mdp.mdp, rng)
//     }
// }
