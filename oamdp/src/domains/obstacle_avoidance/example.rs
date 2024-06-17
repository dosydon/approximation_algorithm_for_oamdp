use std::marker::PhantomData;

use itertools::iproduct;
use mdp::{
    finite_horizon_wrapper::FiniteHorizonWrapper,
    mdp_traits::{ActionEnumerable, Builder},
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
    oamdp::{oamdp::OAMDP, BeliefState},
};

use super::{
    communication_action, communication_model::ObstacleAvoidanceCommunicationModel,
    joint_action::ObstacleAvoidanceJointAction,
};

pub fn example(
    communication_cost: f32,
) -> FiniteHorizonWrapper<
    OAMDP<
        ObstacleAvoidanceCommunicationModel<3>,
        ObstacleAvoidanceMDP,
        BeliefState<ObstacleAvoidanceState, 3>,
        ObstacleAvoidanceJointAction,
        3,
    >,
> {
    let targets = [
        ObstacleAvoidanceParameter::AwareNotYielding,
        ObstacleAvoidanceParameter::AwareYielding,
        ObstacleAvoidanceParameter::NotAwareNotYielding,
    ];

    let start = ObstacleAvoidanceState::new(
        VehicleConfigurationLane::new(0, 3, Lane::Center),
        VehicleConfiguration::new(0, 2),
    );
    let builder = ObstacleAvoidanceBuilder::new(30, 4)
        .set_collision_zone(12, 18)
        .set_start_state(start);

    let communication_model =
        ObstacleAvoidanceCommunicationModel::from_targets(&builder, targets, communication_cost);
    let mdp = builder.build(targets[1]);

    let physical_actions: Vec<_> = mdp.enumerate_actions().cloned().collect();
    let communication_actions = vec![
        communication_action::ObstacleAvoidanceCommunicationAction::Acknowledge,
        communication_action::ObstacleAvoidanceCommunicationAction::None,
    ];
    let joint_actions = iproduct!(physical_actions.iter(), communication_actions.iter())
        .map(|(a, b)| ObstacleAvoidanceJointAction::new(*a, *b))
        .collect::<Vec<_>>();

    let mut target_belief = [NotNan::from_f32(0.0).unwrap(); 3];
    target_belief[1] = NotNan::from_f32(1.0).unwrap();

    let oamdp = FiniteHorizonWrapper::new(
        OAMDP {
            assumed_model: communication_model,
            mdp: mdp,
            distance_measure: BeliefCostFunction::Euclidean(target_belief),
            initial_belief: [NotNan::from_f32(1.0 / 3.0).unwrap(); 3],
            gamma: 0.9,
            all_actions: joint_actions,
            objective: Objective::LinearCombination(0.0, 1.0),
            _phantom_s: PhantomData::<BeliefState<ObstacleAvoidanceState, 3>>,
        },
        40,
    );

    oamdp
}
