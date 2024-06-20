use mdp::common::coordinate2::Coordinate2;
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
use mdp::heuristic::ZeroHeuristic;
use mdp::mdp_traits::Build;
use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;
use mdp::spelling::Letter;
use mdp::spelling::Letter::*;
use mdp::spelling::SpellingAction;
use mdp::spelling::SpellingMDP;
use mdp::spelling::SpellingMDPBuilder;
use mdp::spelling::SpellingState;
use mdp::state_enumerable_wrapper::StateEnumerableWrapper;
use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicy;
use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicyBuilder;

use crate::belief_cost_function::BeliefCostType;
use crate::belief_cost_function::Objective;
use crate::belief_update_type::ObserveabilityAssumption;
use crate::oamdp::oamdp::OAMDP3;

use crate::oamdp::OAMDPFiniteHorizon;
use crate::observer_model::ImplicitCommunicationModel;
use crate::observer_model::SoftmaxModel;

type SpellingMDPE = StateEnumerableWrapper<SpellingMDP<4>>;
// type SpellingMDPE = SpellingMDP<4>;

pub struct SpellingOAMDPBuilder<PB, const N: usize> {
    possible_goals: [[Letter; 4]; N],
    policy_builder: PB,
    max_t: usize,
    builder: SpellingMDPBuilder<4>,
    belief_cost_type: BeliefCostType,
    true_goal: usize,
    objective: Objective,
}

impl<PB, const N: usize> SpellingOAMDPBuilder<PB, N> {
    pub fn set_horizon(mut self, max_t: usize) -> Self {
        self.max_t = max_t;
        self
    }
}

fn get_possible_goals(_id: usize) -> [[Letter; 4]; 3] {
    [[A, R, M, S], [R, A, M, S], [M, A, R, S]]
}

fn get_true_goal(id: usize) -> usize {
    match id {
        1 => 0,
        2 => 0,
        3 => 2,
        4 => 0,
        5 => 0,
        6 => 1,
        7 => 2,
        8 => 0,
        9 => 1,
        10 => 2,
        11 => 2,
        12 => 0,
        _ => panic!("not matching id"),
    }
}

fn get_builder(id: usize) -> SpellingMDPBuilder<4> {
    match id {
        1 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        2 => SpellingMDPBuilder::new(
            5,
            5,
            vec![
                Coordinate2::new(1, 1),
                Coordinate2::new(2, 1),
                Coordinate2::new(3, 1),
                Coordinate2::new(1, 3),
                Coordinate2::new(2, 3),
                Coordinate2::new(3, 3),
            ],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        3 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        12 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(3, 3), (3, 4), (4, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]),
        ),
        11 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(3, 3), (3, 4), (4, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]),
        ),
        4 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]),
        ),
        5 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        8 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        6 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        9 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        7 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        10 => SpellingMDPBuilder::new(
            5,
            5,
            vec![],
            [(0, 0), (1, 1), (3, 3), (4, 4)],
            SpellingState::new(Coordinate2::new(2, 2), [A, A, A, A]),
        ),
        _ => panic!("not matching id"),
    }
}

fn get_belief_cost_type(id: usize) -> BeliefCostType {
    match id {
        3 => BeliefCostType::Disimulation,
        8 => BeliefCostType::Disimulation,
        9 => BeliefCostType::Disimulation,
        10 => BeliefCostType::Disimulation,
        11 => BeliefCostType::Disimulation,
        12 => BeliefCostType::Disimulation,
        _ => BeliefCostType::TVDistance,
    }
}

fn get_objective(_id: usize) -> Objective {
    Objective::LinearCombination(1.0, 0.5)
}

impl SpellingOAMDPBuilder<SoftmaxPolicyBuilder, 3> {
    pub fn new(instance_id: usize) -> Self {
        let true_goal = get_true_goal(instance_id);
        let builder = get_builder(instance_id);
        let possible_goals = get_possible_goals(instance_id);

        SpellingOAMDPBuilder {
            possible_goals: possible_goals,
            policy_builder: SoftmaxPolicyBuilder::new(0.3),
            builder: builder,
            max_t: 20,
            belief_cost_type: get_belief_cost_type(instance_id),
            true_goal: true_goal,
            objective: get_objective(instance_id),
        }
    }
}

impl SpellingOAMDPBuilder<RTDPSoftmaxPolicyBuilder, 3> {
    pub fn new_rtdp(instance_id: usize) -> Self {
        let true_goal = get_true_goal(instance_id);
        let builder = get_builder(instance_id);
        let possible_goals = get_possible_goals(instance_id);

        SpellingOAMDPBuilder {
            possible_goals: possible_goals,
            policy_builder: RTDPSoftmaxPolicyBuilder::new(0.3),
            builder: builder,
            max_t: 20,
            belief_cost_type: get_belief_cost_type(instance_id),
            true_goal: true_goal,
            objective: get_objective(instance_id),
        }
    }
}

impl Build<OAMDPFiniteHorizon<SoftmaxModel<SpellingMDPE, 3>, SpellingMDPE, SpellingAction, 3>>
    for SpellingOAMDPBuilder<SoftmaxPolicyBuilder, 3>
{
    fn build(
        self,
    ) -> OAMDPFiniteHorizon<SoftmaxModel<SpellingMDPE, 3>, SpellingMDPE, SpellingAction, 3> {
        let oamdp = FiniteHorizonWrapper::new(
            OAMDP3::new_implicit_model(
                &self.builder,
                &self.policy_builder,
                self.possible_goals,
                self.true_goal,
                self.belief_cost_type,
                self.objective,
                ObserveabilityAssumption::OnlyActionsAreConsidered,
            ),
            self.max_t,
        );

        oamdp
    }
}

impl
    Build<
        OAMDPFiniteHorizon<
            ImplicitCommunicationModel<
                RTDPSoftmaxPolicy<SpellingState<4>, ZeroHeuristic>,
                SpellingMDP<4>,
                3,
            >,
            SpellingMDP<4>,
            SpellingAction,
            3,
        >,
    > for SpellingOAMDPBuilder<RTDPSoftmaxPolicyBuilder, 3>
{
    fn build(
        self,
    ) -> OAMDPFiniteHorizon<
        ImplicitCommunicationModel<
            RTDPSoftmaxPolicy<SpellingState<4>, ZeroHeuristic>,
            SpellingMDP<4>,
            3,
        >,
        SpellingMDP<4>,
        SpellingAction,
        3,
    > {
        let oamdp = FiniteHorizonWrapper::new(
            OAMDP3::new_implicit_model(
                &self.builder,
                &self.policy_builder,
                self.possible_goals,
                self.true_goal,
                self.belief_cost_type,
                self.objective,
                ObserveabilityAssumption::OnlyActionsAreConsidered,
            ),
            self.max_t,
        );

        oamdp
    }
}
