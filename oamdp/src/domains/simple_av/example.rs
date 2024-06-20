use std::marker::PhantomData;

use itertools::iproduct;
use mdp::{
    finite_horizon_wrapper::FiniteHorizonWrapper,
    mdp_traits::{ActionEnumerable, Builder},
    simple_av::{
        SimpleAVParameter, SimpleAVPartialMDP, SimpleAVState, SimpleAVVehicleInFrontMDP,
        SimpleAVVehicleInFrontState,
    },
};
use num_traits::FromPrimitive;
use ordered_float::NotNan;

use crate::{
    belief_cost_function::{BeliefCostFunction, Objective},
    oamdp::{oamdp::OAMDP, BeliefState},
};

use super::{
    communication_action, communication_model::AVCommunicationModel, joint_action::AVJointAction,
};

pub fn example(
    communication_cost: f32,
) -> FiniteHorizonWrapper<
    OAMDP<
        AVCommunicationModel<3>,
        SimpleAVVehicleInFrontMDP,
        BeliefState<SimpleAVVehicleInFrontState, 3>,
        AVJointAction,
        3,
    >,
> {
    let targets = [
        SimpleAVParameter::NonYield(35, 2, 3),
        SimpleAVParameter::Stopping(6, 8),
        SimpleAVParameter::YouHaveLightOff(35, 2, 3),
    ];
    let partial_mdp = SimpleAVPartialMDP::new(0, 40, -2, 5, SimpleAVState::new(0, 2));
    let communication_model =
        AVCommunicationModel::from_targets(&partial_mdp, targets, communication_cost);

    let partial_mdp = SimpleAVPartialMDP::new(0, 40, -2, 5, SimpleAVState::new(0, 2));
    let mdp: SimpleAVVehicleInFrontMDP = partial_mdp.build(targets[1]);

    let physical_actions: Vec<_> = mdp.enumerate_actions().cloned().collect();
    let communication_actions = vec![
        communication_action::AVCommunicationAction::Flash,
        communication_action::AVCommunicationAction::None,
    ];
    let joint_actions = iproduct!(physical_actions.iter(), communication_actions.iter())
        .map(|(a, b)| AVJointAction::new(*a, *b))
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
            objective: Objective::LinearCombination(1.0, 1.0),
            _phantom_s: PhantomData::<BeliefState<SimpleAVVehicleInFrontState, 3>>,
        },
        10,
    );

    oamdp
}
