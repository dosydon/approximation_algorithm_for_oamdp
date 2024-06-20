use crate::belief_cost_function::*;
use crate::oamdp::belief_state::BeliefState;
use core::fmt::Debug;
use core::hash::Hash;
use core::slice::Iter;
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
use mdp::into_inner::Inner;
use mdp::mdp_traits::*;
use std::collections::HashMap;

use ordered_float::*;

use super::belief_tuple::BeliefTuple;
use crate::traits::ProbSassGivenTheta;

pub struct OAMDP<OM, M: StatesActions, A: Eq + Debug + Hash + Copy, const N: usize> {
    pub assumed_model: OM,
    pub mdp: M,
    pub distance_measure: BeliefCostFunction<N>,
    pub initial_belief: [NotNan<f32>; N],
    pub(crate) gamma: f32,
    pub(crate) all_actions: Vec<A>,
    pub objective: Objective,
    pub(crate) cache: HashMap<BeliefTuple<M::State, A, N>, [NotNan<f32>; N]>,
    pub(crate) cache_hit: usize,
    pub(crate) cache_miss: usize,
}

pub type OAMDPFiniteHorizon<OM, M, A, const N: usize> = FiniteHorizonWrapper<OAMDP<OM, M, A, N>>;

impl<
        OM,
        M: StatesActions,
        A: Eq + Debug + Hash + Copy + Inner<Result = M::Action> + Debug,
        const N: usize,
    > OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A> + ActionEnumerable,
    for<'a> &'a OM: ProbSassGivenTheta<M::State, A>,
{
    pub fn get_belief_changes(&self, trace: &Vec<(M::State, Option<A>)>) -> Vec<[NotNan<f32>; N]> {
        let mut belief_changes = vec![];
        let mut belief = self.initial_belief;
        belief_changes.push(belief);

        for ((s, a), (ss, _aa)) in trace.iter().zip(trace.iter().skip(1)) {
            if let Some(a) = a {
                let new_belief = self.get_new_belief(&belief, s, a, ss);
                belief = new_belief;
                belief_changes.push(belief);
            }
        }

        belief_changes
    }
}

impl<
        OM,
        M: StatesActions,
        A: Eq + Debug + Hash + Copy + Inner<Result = M::Action> + Debug,
        const N: usize,
    > OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A> + ActionEnumerable,
    for<'a> &'a mut OM: ProbSassGivenTheta<M::State, A>,
{
    pub fn get_belief_changes_mut(
        &mut self,
        trace: &Vec<(M::State, Option<A>)>,
    ) -> Vec<[NotNan<f32>; N]> {
        let mut belief_changes = vec![];
        let mut belief = self.initial_belief;
        belief_changes.push(belief);

        for ((s, a), (ss, _aa)) in trace.iter().zip(trace.iter().skip(1)) {
            if let Some(a) = a {
                let new_belief = self.get_new_belief_mut(&belief, s, a, ss);
                belief = new_belief;
                belief_changes.push(belief);
            }
        }

        belief_changes
    }
}

impl<OM, M: StatesActions, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    StatesActions for OAMDP<OM, M, A, N>
{
    type State = BeliefState<M::State, N>;
    type Action = A;
}

impl<OM, M: StatesActions + IsTerminal, A: Eq + Debug + Hash + Copy, const N: usize> IsTerminal
    for OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A>,
    Self::State: Inner<Result = M::State>,
{
    fn is_terminal(&self, s: &Self::State) -> bool {
        self.mdp.is_terminal(&s.inner())
    }
}

impl<OM, M: ActionEnumerable, A: Eq + Debug + Hash + Copy, const N: usize> ActionEnumerable
    for OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A>,
{
    fn enumerate_actions(&self) -> Iter<Self::Action> {
        self.all_actions.iter()
    }
    fn num_actions(&self) -> usize {
        self.all_actions.len()
    }
    fn id_to_action(&self, id: usize) -> &Self::Action {
        &self.all_actions[id]
    }
}

impl<OM, M: StatesActions, A: Eq + Debug + Hash + Copy, const N: usize> ActionAvailability
    for OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A>,
{
    fn action_available(&self, _s: &Self::State, _a: &Self::Action) -> bool {
        true
    }
}

impl<OM, M: InitialState, A: Eq + Hash + Debug + Copy, const N: usize> InitialState
    for OAMDP<OM, M, A, N>
where
    Self: StatesActions<State = BeliefState<M::State, N>, Action = A>,
    Self::State: Inner<Result = M::State>,
{
    fn initial_state(&self) -> Self::State {
        BeliefState::new(self.mdp.initial_state(), self.initial_belief)
    }
}

pub type OAMDPFiniteHorizon2<OM, M> = OAMDPFiniteHorizon<OM, M, <M as StatesActions>::Action, 2>;
pub type OAMDP2<OM, M> = OAMDP<OM, M, <M as StatesActions>::Action, 2>;

pub type OAMDPFiniteHorizon3<OM, M> = OAMDPFiniteHorizon<OM, M, <M as StatesActions>::Action, 3>;
pub type OAMDP3<OM, M> = OAMDP<OM, M, <M as StatesActions>::Action, 3>;

pub type OAMDPFiniteHorizon5<OM, M> = OAMDPFiniteHorizon<OM, M, <M as StatesActions>::Action, 5>;
pub type OAMDP5<OM, M> = OAMDP<OM, M, <M as StatesActions>::Action, 5>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::belief_update_type::ObserveabilityAssumption::*;

    use mdp::baker_grid::*;

    use mdp::common::coordinate2::Coordinate2;
    use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;
    use num_traits::FromPrimitive;

    #[test]
    fn test_is_terminal() {
        let width = 9;
        let height = 5;
        let obstacles = vec![];

        let softmax_policy = SoftmaxPolicyBuilder::new(1.0);
        let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles)
            .set_prob_veering(0.1)
            .set_initial_state(BakerGridState::new(2, 0));
        let possible_goals = [
            BakerGridState::new(2, 8),
            BakerGridState::new(0, 8),
            BakerGridState::new(4, 8),
        ];

        let oamdp: OAMDP3<_, _> = OAMDP::new_implicit_model(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            0,
            BeliefCostType::Euclidean,
            Objective::BeliefCostOnly,
            ActionNotObservable,
        );
        let s = BeliefState::<Coordinate2, 3>::new(
            Coordinate2 { i: 2, j: 8 },
            [
                NotNan::<f32>::from_f32(0.84490675).unwrap(),
                NotNan::<f32>::from_f32(0.1536652).unwrap(),
                NotNan::<f32>::from_f32(0.001428055).unwrap(),
            ],
        );
        assert!(oamdp.is_terminal(&s));
    }
}
