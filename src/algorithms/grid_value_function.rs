use crate::algorithms::belief_point::BeliefPoint;
use crate::algorithms::AssocBeliefPointN;
use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;
use crate::traits::BeliefOverGoal;
use mdp::into_inner::Inner;
use mdp::mdp_traits::*;
use mdp::policy::policy_traits::GetAction;
use ordered_float::NotNan;
use std::collections::HashMap;
use std::f32::MIN;
use std::fmt::Debug;
use std::hash::Hash;

use crate::algorithms::regular_grid_belief_points::RegularGridBeliefPoints;

pub struct GridValueFunction<
    S: Copy + Hash + Eq,
    B: BeliefPoint<N> + Clone + Copy + Debug,
    const N: usize,
> {
    pub table: HashMap<S, RegularGridBeliefPoints<B, N>>,
}

impl<S: Copy + Hash + Eq, B: BeliefPoint<N> + Clone + Copy + Debug, const N: usize>
    GridValueFunction<S, B, N>
{
    #[allow(dead_code)]
    pub fn new(table: HashMap<S, RegularGridBeliefPoints<B, N>>) -> Self {
        Self { table }
    }
}

impl<S: Copy + Hash + Eq, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    GridValueFunction<S, AssocBeliefPointN<A, N>, N>
{
    pub(crate) fn qsa<OM, M>(
        &self,
        s: &S,
        b: &[NotNan<f32>; N],
        a: &A,
        oamdp: &OAMDP<OM, M, A, N>,
        gamma: f32,
    ) -> f32
    where
        S: Debug,
        M: StatesActions<State = S>,
        OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>, Action = A>
            + PMass<f32>
            + Rsa
            + ActionEnumerable,
    {
        let belief_s = BeliefState::new(*s, *b);
        let rsa = oamdp.rsa(&belief_s, a);
        let mut future_term = 0.0;
        for (ss, p) in oamdp.p_mass(&belief_s, a) {
            future_term += p * self
                .table
                .get(&ss.inner())
                .unwrap()
                .get_value_convex_interpolation(&ss.get_belief_over_goal());
        }
        let qsa = rsa + gamma * future_term;
        qsa
    }
}

impl<
        OM,
        S: Copy + Hash + Eq,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy,
        M: StatesActions,
        const N: usize,
    > GetAction<BeliefState<M::State, N>, OAMDP<OM, M, A, N>>
    for GridValueFunction<S, AssocBeliefPointN<A, N>, N>
where
    S: Debug,
    M: StatesActions<State = S>,
    OAMDP<OM, M, A, N>:
        StatesActions<State = BeliefState<S, N>, Action = A> + PMass<f32> + Rsa + ActionEnumerable,
{
    fn get_action(
        &self,
        s: &BeliefState<M::State, N>,
        mdp: &OAMDP<OM, M, A, N>,
        _rng: &mut rand::rngs::ThreadRng,
    ) -> Option<A> {
        let mut result = None;
        let mut best_qsa = MIN;
        for a in mdp.enumerate_actions() {
            let qsa = self.qsa(
                &s.inner(),
                &s.get_belief_over_goal(),
                a,
                mdp,
                mdp.get_discount_factor(),
            );

            if qsa > best_qsa {
                best_qsa = qsa;
                result = Some(*a);
            }
        }
        result
    }
}
