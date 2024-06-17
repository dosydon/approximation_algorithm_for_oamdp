use mdp::{
    into_inner::Inner,
    mdp_traits::{
        ExplicitTransition, ExplicitTransitionMut, InitialState, PMass, PMassMut, StatesActions,
    },
};

use std::fmt::Debug;
use std::hash::Hash;

use crate::{
    oamdp::{oamdp::OAMDP, BeliefState},
    traits::BeliefOverGoal,
};

use super::{VState, OAMDPD};

impl<OM, M: InitialState, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    PMass<f32> for OAMDPD<OM, M, A, N>
where
    OAMDP<OM, M, A, N>: PMass<f32, Distribution = Vec<(BeliefState<M::State, N>, f32)>>
        + StatesActions<State = BeliefState<M::State, N>, Action = A>,
{
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Self::Distribution {
        let mut result = vec![];
        if s.is_dummy_initial_state {
            let bs = self.oamdp.initial_state();
            for (v, coeff) in self
                .translator
                .get_corner_and_lambdas(&bs.get_belief_over_goal())
                .iter()
            {
                if *coeff <= 0.0 {
                    continue;
                }
                let vs = VState::new(bs.inner(), *v);
                result.push((vs, *coeff));
            }
        } else {
            let bs = self.to_belief_state(s);
            for (bss, p) in self.oamdp.p_mass(&bs, a) {
                for (v, coeff) in self
                    .translator
                    .get_corner_and_lambdas(&bss.get_belief_over_goal())
                    .iter()
                {
                    if *coeff <= 0.0 {
                        continue;
                    }
                    let vs = VState::new(bss.inner(), *v);
                    result.push((vs, p * coeff));
                }
            }
        }

        result
    }
}

impl<OM, M: InitialState, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    PMassMut<f32> for OAMDPD<OM, M, A, N>
where
    OAMDP<OM, M, A, N>: PMassMut<f32, Distribution = Vec<(BeliefState<M::State, N>, f32)>>
        + StatesActions<State = BeliefState<M::State, N>, Action = A>,
{
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Self::Distribution {
        let mut result = vec![];
        if s.is_dummy_initial_state {
            let bs = self.oamdp.initial_state();
            for (v, coeff) in self
                .translator
                .get_corner_and_lambdas(&bs.get_belief_over_goal())
                .iter()
            {
                if *coeff <= 0.0 {
                    continue;
                }
                let vs = VState::new(bs.inner(), *v);
                result.push((vs, *coeff));
            }
        } else {
            let bs = self.to_belief_state(s);
            for (bss, p) in self.oamdp.p_mass_mut(&bs, a) {
                for (v, coeff) in self
                    .translator
                    .get_corner_and_lambdas(&bss.get_belief_over_goal())
                    .iter()
                {
                    if *coeff <= 0.0 {
                        continue;
                    }
                    let vs = VState::new(bss.inner(), *v);
                    result.push((vs, p * coeff));
                }
            }
        }

        result
    }
}

impl<
        OM,
        M: InitialState,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy + Inner<Result = M::Action>,
        const N: usize,
    > ExplicitTransition for OAMDPD<OM, M, A, N>
where
    OAMDPD<OM, M, A, N>: PMass<f32>,
{
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        self.p_mass(st, a)
            .into_iter()
            .find(|(s, _)| s == stt)
            .map(|(_, p)| p)
            .unwrap_or(0.0)
    }
}

impl<
        OM,
        M: InitialState,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy + Inner<Result = M::Action>,
        const N: usize,
    > ExplicitTransitionMut for OAMDPD<OM, M, A, N>
where
    OAMDPD<OM, M, A, N>: PMassMut<f32> + PMass<f32>,
{
    fn p_mut(&mut self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        self.p_mass_mut(st, a)
            .into_iter()
            .find(|(s, _)| s == stt)
            .map(|(_, p)| p)
            .unwrap_or(0.0)
    }
}
