use mdp::mdp_traits::StatesActions;

use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;
use crate::traits::CommunicationCost;

use super::RecycleState;
use super::{
    action::{RecycleCommunicationAction, RecycleJointAction},
    communication_model::RecycleCommunicationModel,
    mdp::RecycleMDP,
};

impl<const K: usize, const N: usize> CommunicationCost
    for OAMDP<RecycleCommunicationModel<K>, RecycleMDP<K>, RecycleJointAction, N>
where
    Self: StatesActions<State = BeliefState<RecycleState<K>, N>, Action = RecycleJointAction>,
{
    fn communication_cost(&self, _s: &Self::State, a: &Self::Action) -> f32 {
        match &a.communication_action {
            RecycleCommunicationAction::None => 0.0,
            _ => self.assumed_model.communication_cost,
        }
    }
}
