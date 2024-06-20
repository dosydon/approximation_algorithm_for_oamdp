use mdp::{
    heuristic::{HeuristicWithMDP, HeuristicWithMDPMut},
    into_inner::Inner,
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DisplayState, Eval, GetNextStateMut,
        InitialState, IntoIterWith, IsTerminal, PMass, PMassMut, StatesActions,
    },
    policy::policy_traits::GetActionMut,
};
use num_traits::FromPrimitive;
use ordered_float::NotNan;
use rand::rngs::ThreadRng;
use rtdp::rtdp::RTDP;
use std::{collections::HashMap, hash::Hash};
use std::{collections::HashSet, fmt::Debug};

use crate::{
    algorithms::{
        grid_value_function_ssp::GridValueFunctionSSP,
        regular_grid_belief_points::RegularGridBeliefPoints, AssocBeliefPointN,
    },
    oamdp::{oamdp::OAMDP, BeliefState},
    oamdp_d::{VState, OAMDPD},
    traits::{BeliefOverGoal, ProbSassGivenTheta},
};

use super::{
    episode_iterator::EpisodeIterator,
    traits::{RTDPNumStates, RTDPRootValue, RTDPTrait, RTDPTraitAll, RunEpisode},
};

#[allow(non_camel_case_types)]
pub struct RTDP_OAMDP<
    OM,
    M: StatesActions,
    A: PartialEq + Eq + Copy + Clone + Debug + Hash,
    H,
    const N: usize,
> {
    pub rtdp: RTDP<VState<M::State, N>, H>,
    pub oamdp_d: OAMDPD<OM, M, A, N>,
    pub(crate) max_t: Option<usize>,
}

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    RTDP_OAMDP<OM, M, A, H, N>
{
    pub fn new(oamdp: OAMDP<OM, M, A, N>, h: H, k: usize) -> RTDP_OAMDP<OM, M, A, H, N> {
        RTDP_OAMDP {
            rtdp: RTDP::new(h),
            oamdp_d: OAMDPD::new(oamdp, k),
            max_t: None,
        }
    }

    pub fn set_max_horizon(mut self, max_t: usize) -> Self {
        self.max_t = Some(max_t);
        self
    }
}

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    RTDP_OAMDP<OM, M, A, H, N>
where
    OAMDPD<OM, M, A, N>: StatesActions<State = VState<M::State, N>, Action = A>
        + PMass<f32>
        + Cost
        + ActionEnumerable,
    H: HeuristicWithMDP<OAMDPD<OM, M, A, N>>,
{
    pub fn to_grid_vf(&self) -> GridValueFunctionSSP<M::State, AssocBeliefPointN<A, N>, N> {
        let mut table: HashMap<M::State, RegularGridBeliefPoints<AssocBeliefPointN<A, N>, N>> =
            HashMap::new();
        for (vs, v) in self.rtdp.vt.value_table.iter() {
            let a = self.rtdp.best_action(vs, &self.oamdp_d);
            let bp = AssocBeliefPointN::new_not_nan(
                a,
                NotNan::from_f32(*v).unwrap(),
                self.oamdp_d.translator.v_to_b(&vs.v),
            );
            if let Some(grid) = table.get_mut(&vs.s) {
                grid.push(bp);
            } else {
                let mut grid =
                    RegularGridBeliefPoints::new(self.oamdp_d.translator.num_bin_per_dim);
                grid.push(bp);
                table.insert(vs.s, grid);
            }
            println!("{:?} {:?}", vs, v);
        }
        GridValueFunctionSSP::new(table)
    }
}

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    RTDPNumStates for RTDP_OAMDP<OM, M, A, H, N>
{
    fn num_states(&self) -> usize {
        self.rtdp.num_states()
    }

    fn num_domain_states(&self) -> usize {
        let mut hash_set = HashSet::new();
        for vs in self.rtdp.vt.value_table.keys() {
            hash_set.insert(vs.inner());
        }
        hash_set.len()
    }
}

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    RTDPTrait for RTDP_OAMDP<OM, M, A, H, N>
where
    OAMDPD<OM, M, A, N>: InitialState
        + StatesActions<State = VState<M::State, N>>
        + PMassMut<f32>
        + Cost
        + IsTerminal
        + GetNextStateMut
        + ActionEnumerable
        + ActionAvailability,
    H: HeuristicWithMDPMut<OAMDPD<OM, M, A, N>>,
{
    fn rtdp(&mut self, num_trials: usize, rng: &mut ThreadRng) {
        self.rtdp.solve(&mut self.oamdp_d, rng, num_trials)
    }

    fn lrtdp(&mut self, num_trials: usize, rng: &mut ThreadRng) {
        self.rtdp.lrtdp(&mut self.oamdp_d, num_trials, rng, 1e-3)
    }
}

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    RTDPRootValue for RTDP_OAMDP<OM, M, A, H, N>
where
    OAMDPD<OM, M, A, N>: InitialState
        + StatesActions<State = VState<M::State, N>>
        + PMassMut<f32>
        + Cost
        + IsTerminal,
    H: HeuristicWithMDPMut<OAMDPD<OM, M, A, N>>,
{
    fn root_value(&mut self) -> f32 {
        self.rtdp
            .get_value_ssp_mut(&self.oamdp_d.initial_state(), &mut self.oamdp_d)
    }
}

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    RTDP_OAMDP<OM, M, A, H, N>
{
    pub fn root_value(&mut self) -> f32
    where
        OAMDPD<OM, M, A, N>: InitialState
            + StatesActions<State = VState<M::State, N>>
            + PMassMut<f32>
            + Cost
            + IsTerminal,
        H: HeuristicWithMDPMut<OAMDPD<OM, M, A, N>>,
    {
        self.rtdp
            .get_value_ssp_mut(&self.oamdp_d.initial_state(), &mut self.oamdp_d)
    }

    fn get_value_mut(&mut self, bs: &BeliefState<M::State, N>) -> f32
    where
        OAMDPD<OM, M, A, N>: StatesActions<State = VState<M::State, N>> + PMassMut<f32> + Cost,
        H: HeuristicWithMDPMut<OAMDPD<OM, M, A, N>>,
    {
        let b = bs.get_belief_over_goal();
        let mut value = 0.0;
        for (v, w) in self.oamdp_d.translator.get_corner_and_lambdas(&b) {
            let vs = VState::new(bs.inner(), v);

            if !self.rtdp.is_solved.contains(&vs) {
                println!("{:?} is not labeld", vs);
                println!("{:?}", self.oamdp_d.to_belief_state(&vs));
            }

            value += w * self.rtdp.get_value_ssp_mut(&vs, &mut self.oamdp_d);
        }
        value
    }

    pub fn get_qsa_ssp_mut(&mut self, bs: &BeliefState<M::State, N>, a: &A) -> f32
    where
        OAMDPD<OM, M, A, N>: StatesActions<State = VState<M::State, N>> + PMassMut<f32> + Cost,
        OAMDP<OM, M, A, N>:
            StatesActions<State = BeliefState<M::State, N>, Action = A> + PMassMut<f32> + Cost,
        H: HeuristicWithMDPMut<OAMDPD<OM, M, A, N>>,
    {
        let mut future_term = 0.0;
        let cost = self.oamdp_d.oamdp.cost(bs, a);
        for (bss, p) in self.oamdp_d.oamdp.p_mass_mut(bs, a) {
            future_term += p * self.get_value_mut(&bss)
        }
        cost + future_term
    }
}

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    RunEpisode for RTDP_OAMDP<OM, M, A, H, N>
where
    Self: GetActionMut<BeliefState<M::State, N>, OAMDP<OM, M, A, N>>,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>, Action = A>
        + DisplayState<BeliefState<M::State, N>>,
    for<'b> &'b mut OM: ProbSassGivenTheta<M::State, A>,
    for<'a> &'a mut Self: IntoIterWith<
        'a,
        Item = (BeliefState<M::State, N>, A, BeliefState<M::State, N>, f32),
        I = EpisodeIterator<'a, OM, M, A, H, N>,
    >,
    for<'a> EpisodeIterator<'a, OM, M, A, H, N>:
        Iterator<Item = (BeliefState<M::State, N>, A, BeliefState<M::State, N>, f32)>,
{
    fn run_episode(&mut self, rng: &mut ThreadRng) {
        let max_t = self.max_t;
        unsafe {
            let self_p = self as *mut Self;
            for (s, a, _ss, _c) in (*self_p).into_iter_with(rng).set_max_t(max_t) {
                println!("{:?}", a);
                (*self_p).oamdp_d.oamdp.display(&s);
            }
        }

        self.oamdp_d.oamdp.print_cache_stats();
    }
}

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    RTDPTraitAll for RTDP_OAMDP<OM, M, A, H, N>
where
    Self: RTDPTrait + RTDPNumStates + RTDPRootValue + RunEpisode + Eval,
{
}
