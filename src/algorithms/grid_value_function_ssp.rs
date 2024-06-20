use crate::algorithms::belief_point::BeliefPoint;
use crate::algorithms::AssocBeliefPointN;
use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;
use crate::traits::BeliefOverGoal;
use mdp::into_inner::Inner;
use mdp::mdp_traits::*;
use mdp::policy::policy_traits::GetAction;
use ordered_float::NotNan;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::f32::MAX;
use std::fmt::Debug;
use std::hash::Hash;

use crate::algorithms::regular_grid_belief_points::RegularGridBeliefPoints;

pub enum PolicyType {
    OneStepLookAhead,
    SnatchGridPoint,
}

pub struct GridValueFunctionSSP<
    S: Copy + Hash + Eq,
    B: BeliefPoint<N> + Clone + Copy + Debug,
    const N: usize,
> {
    pub table: HashMap<S, RegularGridBeliefPoints<B, N>>,
    policy_type: PolicyType,
}

impl<S: Copy + Hash + Eq, B: BeliefPoint<N> + Clone + Copy + Debug, const N: usize>
    GridValueFunctionSSP<S, B, N>
{
    pub fn new(table: HashMap<S, RegularGridBeliefPoints<B, N>>) -> Self {
        Self {
            table,
            policy_type: PolicyType::OneStepLookAhead,
        }
    }

    pub fn num_states(&self) -> usize {
        let mut sum = 0;
        for grid in self.table.values() {
            sum += grid.grid.len();
        }
        sum
    }

    pub fn num_domain_states(&self) -> usize {
        self.table.len()
    }

    pub fn set_policy_type(mut self, policy_type: PolicyType) -> Self {
        self.policy_type = policy_type;
        self
    }
}

impl<S: Copy + Hash + Eq, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    GridValueFunctionSSP<S, AssocBeliefPointN<A, N>, N>
{
    pub fn get_value(&self, bs: &BeliefState<S, N>) -> f32
    where
        S: Debug,
    {
        self.table
            .get(&bs.inner())
            .unwrap()
            .get_value_convex_interpolation(&bs.get_belief_over_goal())
    }

    pub fn qsa_ssp<OM, M>(
        &self,
        s: &S,
        b: &[NotNan<f32>; N],
        a: &A,
        oamdp: &OAMDP<OM, M, A, N>,
    ) -> f32
    where
        S: Debug,
        M: StatesActions<State = S>,
        OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<S, N>, Action = A>
            + PMass<f32>
            + Cost
            + ActionEnumerable,
    {
        let belief_s = BeliefState::new(*s, *b);
        let cost = oamdp.cost(&belief_s, a);
        let mut future_term = 0.0;
        for (ss, p) in oamdp.p_mass(&belief_s, a) {
            future_term += p * self
                .table
                .get(&ss.inner())
                .unwrap()
                .get_value_convex_interpolation(&ss.get_belief_over_goal());
        }
        let qsa = cost + future_term;
        qsa
    }

    pub fn qsa_ssp_mut<OM, M>(
        &self,
        s: &S,
        b: &[NotNan<f32>; N],
        a: &A,
        oamdp: &mut OAMDP<OM, M, A, N>,
    ) -> f32
    where
        S: Debug,
        M: StatesActions<State = S>,
        OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<S, N>, Action = A>
            + PMassMut<f32>
            + Cost
            + ActionEnumerable,
    {
        let belief_s = BeliefState::new(*s, *b);
        let cost = oamdp.cost(&belief_s, a);
        let mut future_term = 0.0;
        for (ss, p) in oamdp.p_mass_mut(&belief_s, a) {
            future_term += p * self
                .table
                .get(&ss.inner())
                .unwrap()
                .get_value_convex_interpolation(&ss.get_belief_over_goal());
        }
        let qsa = cost + future_term;
        qsa
    }
}

impl<S: Copy + Hash + Eq, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    GridValueFunctionSSP<S, AssocBeliefPointN<A, N>, N>
{
    fn one_step_lookahead<OM, M>(
        &self,
        s: &BeliefState<M::State, N>,
        mdp: &OAMDP<OM, M, A, N>,
        _rng: &mut rand::rngs::ThreadRng,
    ) -> Option<A>
    where
        S: Debug,
        M: StatesActions<State = S>,
        OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<S, N>, Action = A>
            + PMass<f32>
            + Cost
            + ActionEnumerable,
    {
        let mut result = None;
        let mut best_qsa = MAX;
        for a in mdp.enumerate_actions() {
            let qsa = self.qsa_ssp(&s.inner(), &s.get_belief_over_goal(), a, mdp);

            if qsa < best_qsa {
                best_qsa = qsa;
                result = Some(*a);
            }
        }
        result
    }
}

impl<S: Copy + Hash + Eq, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    GridValueFunctionSSP<S, AssocBeliefPointN<A, N>, N>
{
    fn snatch_grid_point<OM, M>(
        &self,
        s: &BeliefState<M::State, N>,
        mdp: &OAMDP<OM, M, A, N>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<A>
    where
        S: Debug,
        M: StatesActions<State = S>,
        OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<S, N>, Action = A>
            + PMass<f32>
            + Cost
            + ActionEnumerable,
    {
        if let Some(gp) = self.table.get(&s.inner()) {
            let pairs = gp
                .translator
                .get_corner_and_lambdas(&s.get_belief_over_goal());
            if let Ok(pair) = pairs.choose_weighted(rng, |(_v, w)| *w) {
                let bs = BeliefState::new(s.inner(), gp.translator.v_to_b(&pair.0));
                self.one_step_lookahead(&bs, mdp, rng)
            } else {
                panic!("{:?}", pairs);
            }
        } else {
            None
        }
    }
}

impl<
        OM,
        S: Copy + Hash + Eq,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy,
        M: StatesActions,
        const N: usize,
    > GetAction<BeliefState<M::State, N>, OAMDP<OM, M, A, N>>
    for GridValueFunctionSSP<S, AssocBeliefPointN<A, N>, N>
where
    S: Debug,
    M: StatesActions<State = S>,
    OAMDP<OM, M, A, N>:
        StatesActions<State = BeliefState<S, N>, Action = A> + PMass<f32> + Cost + ActionEnumerable,
{
    fn get_action(
        &self,
        s: &BeliefState<M::State, N>,
        mdp: &OAMDP<OM, M, A, N>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<A> {
        match self.policy_type {
            PolicyType::OneStepLookAhead => self.one_step_lookahead(s, mdp, rng),
            PolicyType::SnatchGridPoint => self.snatch_grid_point(s, mdp, rng),
        }
    }
}
