use mdp::into_inner::Inner;
use mdp::mdp_traits::StatesActions;
use mdp::simple_av::{SimpleAVVehicleInFrontMDP, SimpleAVVehicleInFrontState};

use crate::oamdp::oamdp::OAMDP;
use crate::traits::{BeliefOverGoal, CommunicationCost};

use super::communication_model::AVCommunicationModel;
use super::joint_action::AVJointAction;
use super::AVCommunicationAction;

impl<const N: usize> CommunicationCost
    for OAMDP<AVCommunicationModel<N>, SimpleAVVehicleInFrontMDP, AVJointAction, N>
where
    Self: StatesActions<Action = AVJointAction>,
    Self::State: Inner<Result = SimpleAVVehicleInFrontState> + BeliefOverGoal<N>,
{
    fn communication_cost(&self, _s: &Self::State, a: &Self::Action) -> f32 {
        let communication_cost = match &a.communication_action {
            AVCommunicationAction::None => 0.0,
            _ => self.assumed_model.communication_cost,
        };
        communication_cost
    }
}
