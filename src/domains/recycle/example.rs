use crate::{
    belief_cost_function::{BeliefCostFunction, Objective},
    domains::recycle::{
        Location, RecycleCommunicationAction, RecycleCommunicationModel, RecycleJointAction,
        RecycleMDP,
    },
    oamdp::oamdp::OAMDP,
};
use itertools::iproduct;
use mdp::{
    finite_horizon_wrapper::FiniteHorizonWrapper, mdp_traits::ActionEnumerable,
    policy::softmax_policy::SoftmaxPolicy, value_iteration::value_iteration_ssp,
};
use num_traits::FromPrimitive;
use ordered_float::NotNan;

// fn enumerate_targets() -> Vec<[Location; 3]> {
//     let targets = vec![
//         [Location::Compost, Location::Compost, Location::Compost],
//         [Location::Compost, Location::Compost, Location::Recycle],
//         [Location::Compost, Location::Compost, Location::Trash],
//         [Location::Compost, Location::Recycle, Location::Compost],
//         [Location::Compost, Location::Recycle, Location::Recycle],
//         [Location::Compost, Location::Recycle, Location::Trash],
//         [Location::Compost, Location::Trash, Location::Compost],
//         [Location::Compost, Location::Trash, Location::Recycle],
//         [Location::Compost, Location::Trash, Location::Trash],
//         [Location::Recycle, Location::Compost, Location::Compost],
//         [Location::Recycle, Location::Compost, Location::Recycle],
//         [Location::Recycle, Location::Compost, Location::Trash],
//         [Location::Recycle, Location::Recycle, Location::Compost],
//         [Location::Recycle, Location::Recycle, Location::Recycle],
//         [Location::Recycle, Location::Recycle, Location::Trash],
//         [Location::Recycle, Location::Trash, Location::Compost],
//         [Location::Recycle, Location::Trash, Location::Recycle],
//         [Location::Recycle, Location::Trash, Location::Trash],
//         [Location::Trash, Location::Compost, Location::Compost],
//         [Location::Trash, Location::Compost, Location::Recycle],
//         [Location::Trash, Location::Compost, Location::Trash],
//         [Location::Trash, Location::Recycle, Location::Compost],
//         [Location::Trash, Location::Recycle, Location::Recycle],
//         [Location::Trash, Location::Recycle, Location::Trash],
//         [Location::Trash, Location::Trash, Location::Compost],
//         [Location::Trash, Location::Trash, Location::Recycle],
//         [Location::Trash, Location::Trash, Location::Trash],
//     ];
//     targets
// }
//
pub(crate) fn enumerate_communication_actions() -> Vec<RecycleCommunicationAction> {
    vec![
        RecycleCommunicationAction::None,
        RecycleCommunicationAction::Announce(Location::Recycle),
        RecycleCommunicationAction::Announce(Location::Compost),
        RecycleCommunicationAction::Announce(Location::Trash),
    ]
}

pub fn example_explain_failure(
) -> FiniteHorizonWrapper<OAMDP<RecycleCommunicationModel<5>, RecycleMDP<5>, RecycleJointAction, 2>>
{
    let mdp = RecycleMDP::new(
        [Location::Compost, Location::Recycle, Location::Trash],
        [Location::Trash; 5],
        [0, 1, 2, 0, 1],
        0.5,
    );

    let mdp0 = RecycleMDP::new(
        [Location::Compost, Location::Recycle, Location::Trash],
        [Location::Trash; 5],
        [0, 1, 2, 0, 1],
        0.5,
    );
    let vt0 = value_iteration_ssp(&mdp0);
    let policy0 = SoftmaxPolicy::new(0.3, vt0);

    let mdp1 = RecycleMDP::new(
        [Location::Compost, Location::Compost, Location::Trash],
        [Location::Trash; 5],
        [0, 1, 2, 0, 1],
        0.9,
    );
    let vt1 = value_iteration_ssp(&mdp1);
    let policy1 = SoftmaxPolicy::new(0.3, vt1);

    let cm = RecycleCommunicationModel::new(
        vec![mdp0, mdp1],
        vec![policy0, policy1],
        0.25,
        vec![
            [Location::Compost, Location::Recycle, Location::Trash],
            [Location::Compost, Location::Compost, Location::Trash],
        ],
        [0, 1, 2, 0, 1],
    );

    let communication_actions = enumerate_communication_actions();

    let physical_actions: Vec<_> = mdp.enumerate_actions().cloned().collect();
    let joint_actions = iproduct!(physical_actions.iter(), communication_actions.iter())
        .map(|(a, b)| RecycleJointAction::new(*a, *b))
        .collect::<Vec<_>>();

    let mut target_belief = [NotNan::from_f32(0.0).unwrap(); 2];
    target_belief[0] = NotNan::from_f32(1.0).unwrap();

    let oamdp = FiniteHorizonWrapper::new(
        OAMDP::new(
            cm,
            mdp,
            BeliefCostFunction::Euclidean(target_belief),
            [NotNan::from_f32(1.0 / 2.0).unwrap(); 2],
            0.9,
            joint_actions,
            Objective::LinearCombination(1.0, 1.0),
        ),
        15,
    );

    oamdp
}

// pub fn example(
//     communication_cost: f32,
//     assumed_success_prob: f32,
//     actual_success_prob: f32,
// ) -> FiniteHorizonWrapper<
//     OAMDP<
//         RecycleCommunicationModel<3>,
//         RecycleMDP<3>,
//         BeliefState<RecycleState<3>, 27>,
//         RecycleJointAction,
//         27,
//     >,
// > {
//     let true_target = 5;
//     let targets = enumerate_targets();
//     println!("targets: {:?}", targets[true_target]);
//
//     let mdp = RecycleMDP::new(
//         targets[true_target].clone(),
//         [Location::Trash, Location::Trash, Location::Trash],
//         [0, 1, 2],
//         actual_success_prob,
//     );
//
//     let cm = RecycleCommunicationModel::from_targets(
//         targets,
//         communication_cost,
//         [Location::Trash, Location::Trash, Location::Trash],
//         [0, 1, 2],
//         assumed_success_prob,
//     );
//
//     let communication_actions = enumerate_communication_actions();
//
//     let physical_actions: Vec<_> = mdp.enumerate_actions().cloned().collect();
//     let joint_actions = iproduct!(physical_actions.iter(), communication_actions.iter())
//         .map(|(a, b)| RecycleJointAction::new(*a, *b))
//         .collect::<Vec<_>>();
//
//     let mut target_belief = [NotNan::from_f32(0.0).unwrap(); 27];
//     target_belief[true_target] = NotNan::from_f32(1.0).unwrap();
//
//     let oamdp = FiniteHorizonWrapper::new(
//         OAMDP {
//             assumed_model: cm,
//             mdp: mdp,
//             distance_measure: BeliefCostFunction::Euclidean(target_belief),
//             initial_belief: [NotNan::from_f32(1.0 / 27.0).unwrap(); 27],
//             gamma: 0.9,
//             all_actions: joint_actions,
//             objective: Objective::LinearCombination(1.0, 1.0),
//             _phantom_s: PhantomData::<BeliefState<RecycleState<3>, 27>>,
//         },
//         10,
//     );
//
//     oamdp
// }
//
// pub fn example5(
//     communication_cost: f32,
//     assumed_success_prob: f32,
//     actual_success_prob: f32,
// ) -> FiniteHorizonWrapper<
//     OAMDP<
//         RecycleCommunicationModel<5>,
//         RecycleMDP<5>,
//         BeliefState<RecycleState<5>, 27>,
//         RecycleJointAction,
//         27,
//     >,
// > {
//     let targets = enumerate_targets();
//
//     let cm = RecycleCommunicationModel::from_targets(
//         targets,
//         communication_cost,
//         [
//             Location::Trash,
//             Location::Trash,
//             Location::Trash,
//             Location::Trash,
//             Location::Trash,
//         ],
//         [0, 1, 2, 0, 0],
//         assumed_success_prob,
//     );
//
//     let communication_actions = enumerate_communication_actions();
//
//     let mdp = RecycleMDP::new(
//         [Location::Recycle, Location::Trash, Location::Compost],
//         [
//             Location::Trash,
//             Location::Trash,
//             Location::Trash,
//             Location::Trash,
//             Location::Trash,
//         ],
//         [0, 1, 2, 0, 0],
//         actual_success_prob,
//     );
//
//     let physical_actions: Vec<_> = mdp.enumerate_actions().cloned().collect();
//     let joint_actions = iproduct!(physical_actions.iter(), communication_actions.iter())
//         .map(|(a, b)| RecycleJointAction::new(*a, *b))
//         .collect::<Vec<_>>();
//
//     let mut target_belief = [NotNan::from_f32(0.0).unwrap(); 27];
//     target_belief[16] = NotNan::from_f32(1.0).unwrap();
//
//     let oamdp = FiniteHorizonWrapper::new(
//         OAMDP {
//             assumed_model: cm,
//             mdp: mdp,
//             distance_measure: BeliefCostFunction::Euclidean(target_belief),
//             initial_belief: [NotNan::from_f32(1.0 / 27.0).unwrap(); 27],
//             gamma: 0.9,
//             all_actions: joint_actions,
//             objective: Objective::LinearCombination(1.0, 1.0),
//             _phantom_s: PhantomData::<BeliefState<RecycleState<5>, 27>>,
//         },
//         20,
//     );
//
//     oamdp
// }
//
