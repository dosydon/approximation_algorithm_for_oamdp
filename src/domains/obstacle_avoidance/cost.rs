
use mdp::mdp_traits::{StatesActions};
use mdp::simple_av_obstacle_avoidance::{ObstacleAvoidanceMDP, ObstacleAvoidanceState};


use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;
use crate::traits::{CommunicationCost};

use super::communication_action::ObstacleAvoidanceCommunicationAction;
use super::communication_model::ObstacleAvoidanceCommunicationModel;
use super::joint_action::ObstacleAvoidanceJointAction;

impl<const N: usize> CommunicationCost
    for OAMDP<
        ObstacleAvoidanceCommunicationModel<N>,
        ObstacleAvoidanceMDP,
        ObstacleAvoidanceJointAction,
        N,
    >
where
    Self: StatesActions<
        State = BeliefState<ObstacleAvoidanceState, N>,
        Action = ObstacleAvoidanceJointAction,
    >,
{
    fn communication_cost(&self, _s: &Self::State, a: &Self::Action) -> f32 {
        let communication_cost = match &a.communication_action {
            ObstacleAvoidanceCommunicationAction::None => 0.0,
            __ => self.assumed_model.communication_cost,
        };
        communication_cost
    }
}

// impl<const N: usize> DCost
//     for OAMDP<
//         ObstacleAvoidanceCommunicationModel<N>,
//         ObstacleAvoidanceMDP,
//         ObstacleAvoidanceJointAction,
//         N,
//     >
// where
//     Self: StatesActions<
//         State = BeliefState<ObstacleAvoidanceState, N>,
//         Action = ObstacleAvoidanceJointAction,
//     >,
// {
//     fn d_cost(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
//         let b = st.get_belief_over_goal();
//         let information_cost = self.distance_measure.b_cost(&b);
//         let communication_cost = match &a.communication_action {
//             ObstacleAvoidanceCommunicationAction::None => 0.0,
//             _ => self.assumed_model.communication_cost,
//         };
//
//         match self.objective {
//             Objective::BeliefCostOnly => information_cost + communication_cost,
//             Objective::LinearCombination(c, d) => {
//                 let s_cost = self
//                     .mdp
//                     .d_cost(&st.into_inner(), &a.into_inner(), &stt.into_inner())
//                     + communication_cost;
//                 //                 debug!("s_cost: {}", d * s_cost);
//                 //                 debug!("information_cost: {}", c * information_cost);
//                 c * information_cost + d * s_cost
//             }
//         }
//     }
// }
//
// impl<const N: usize> Cost
//     for OAMDP<
//         ObstacleAvoidanceCommunicationModel<N>,
//         ObstacleAvoidanceMDP,
//         ObstacleAvoidanceJointAction,
//         N,
//     >
// where
//     Self: StatesActions<
//         State = BeliefState<ObstacleAvoidanceState, N>,
//         Action = ObstacleAvoidanceJointAction,
//     >,
// {
//     fn cost(&self, st: &Self::State, a: &Self::Action) -> f32 {
//         let b = st.get_belief_over_goal();
//         let information_cost = self.distance_measure.b_cost(&b);
//         let communication_cost = match &a.communication_action {
//             ObstacleAvoidanceCommunicationAction::None => 0.0,
//             _ => self.assumed_model.communication_cost,
//         };
//
//         match self.objective {
//             Objective::BeliefCostOnly => information_cost + communication_cost,
//             Objective::LinearCombination(c, d) => {
//                 let s_cost = self.mdp.cost(&st.into_inner(), &a.into_inner()) + communication_cost;
//                 //                 debug!("s_cost: {}", d * s_cost);
//                 //                 debug!("information_cost: {}", c * information_cost);
//                 c * information_cost + d * s_cost
//             }
//         }
//     }
// }
//
