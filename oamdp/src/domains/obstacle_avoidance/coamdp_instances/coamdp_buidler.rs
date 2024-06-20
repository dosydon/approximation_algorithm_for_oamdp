use itertools::iproduct;
use mdp::{
    finite_horizon_wrapper::FiniteHorizonWrapper,
    mdp_traits::{ActionEnumerable, BuildFrom},
    simple_av::VehicleConfiguration,
    simple_av_obstacle_avoidance::{
        Lane, ObstacleAvoidanceBuilder, ObstacleAvoidanceMDP, ObstacleAvoidanceParameter,
        ObstacleAvoidanceState, VehicleConfigurationLane,
    },
};
use num_traits::FromPrimitive;
use ordered_float::NotNan;

use crate::{
    belief_cost_function::{BeliefCostFunction, Objective},
    domains::obstacle_avoidance::{
        communication_action::ObstacleAvoidanceCommunicationAction,
        communication_model::ObstacleAvoidanceCommunicationModel,
        joint_action::ObstacleAvoidanceJointAction,
    },
    oamdp::{oamdp::OAMDP, OAMDPFiniteHorizon},
};

pub struct ObstacleAvoidanceCOAMDPBuilder<const N: usize> {
    partial_mdp: ObstacleAvoidanceBuilder,
    possible_goals: [ObstacleAvoidanceParameter; N],
    communication_actions: Vec<ObstacleAvoidanceCommunicationAction>,
    max_t: usize,
    mdp: ObstacleAvoidanceMDP,
    belief_cost_function: BeliefCostFunction<N>,
    cost_type: Objective,
    communication_cost: f32,
}

fn get_partial_mdp(_id: usize) -> ObstacleAvoidanceBuilder {
    let start = ObstacleAvoidanceState::new(
        VehicleConfigurationLane::new(0, 3, Lane::Center),
        VehicleConfiguration::new(0, 2),
    );
    let builder = ObstacleAvoidanceBuilder::new(30, 4)
        .set_collision_zone(12, 18)
        .set_start_state(start);
    builder
}

fn get_possible_goals(_id: usize) -> [ObstacleAvoidanceParameter; 3] {
    let targets = [
        ObstacleAvoidanceParameter::AwareNotYielding,
        ObstacleAvoidanceParameter::AwareYielding,
        ObstacleAvoidanceParameter::NotAwareNotYielding,
    ];
    targets
}

fn get_true_goal(id: usize) -> usize {
    match id {
        1 => 0,
        2 => 1,
        3 => 1,
        _ => panic!("not matching id"),
    }
}

fn get_belief_cost_function(id: usize) -> BeliefCostFunction<3> {
    match id {
        1 => BeliefCostFunction::get_legible_cost_function(get_true_goal(id)),
        2 => BeliefCostFunction::get_legible_cost_function(get_true_goal(id)),
        3 => BeliefCostFunction::get_legible_cost_function(get_true_goal(id)),
        _ => panic!("not matching id"),
    }
}

fn get_cost_type(id: usize) -> Objective {
    match id {
        1 => Objective::LinearCombination(1.0, 1.0),
        2 => Objective::LinearCombination(1.0, 1.0),
        3 => Objective::LinearCombination(1.0, 1.0),
        _ => panic!("not matching id"),
    }
}

fn get_communication_cost(id: usize) -> f32 {
    match id {
        3 => 1.0,
        _ => 0.25,
    }
}

impl ObstacleAvoidanceCOAMDPBuilder<3> {
    pub fn new(instance_id: usize) -> Self {
        let true_goal = get_true_goal(instance_id);
        let partial_mdp = get_partial_mdp(instance_id);
        let possible_goals = get_possible_goals(instance_id);
        let mdp = partial_mdp.build_from(&possible_goals[true_goal]);

        ObstacleAvoidanceCOAMDPBuilder {
            partial_mdp: partial_mdp,
            possible_goals: possible_goals,
            communication_actions: vec![
                ObstacleAvoidanceCommunicationAction::Acknowledge,
                ObstacleAvoidanceCommunicationAction::None,
            ],
            max_t: 20,
            mdp: mdp,
            belief_cost_function: get_belief_cost_function(instance_id),
            cost_type: get_cost_type(instance_id),
            communication_cost: get_communication_cost(instance_id),
        }
    }
}

impl ObstacleAvoidanceCOAMDPBuilder<3> {
    pub fn build(
        self,
    ) -> OAMDPFiniteHorizon<
        ObstacleAvoidanceCommunicationModel<3>,
        ObstacleAvoidanceMDP,
        ObstacleAvoidanceJointAction,
        3,
    > {
        let physical_actions: Vec<_> = self.mdp.enumerate_actions().cloned().collect();

        let joint_actions = iproduct!(physical_actions.iter(), self.communication_actions.iter())
            .map(|(a, b)| ObstacleAvoidanceJointAction::new(*a, *b))
            .collect::<Vec<_>>();

        let communication_model = ObstacleAvoidanceCommunicationModel::from_targets(
            &self.partial_mdp,
            self.possible_goals,
            self.communication_cost,
        );

        let oamdp = FiniteHorizonWrapper::new(
            OAMDP::new(
                communication_model,
                self.mdp,
                self.belief_cost_function,
                [NotNan::from_f32(1.0 / 3.0).unwrap(); 3],
                0.9,
                joint_actions,
                self.cost_type,
            ),
            self.max_t,
        );

        oamdp
    }
}
