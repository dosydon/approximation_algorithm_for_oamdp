use mdp::{
    baker_grid::{BakerGridMDP, BakerGridState},
    mdp_traits::StatesActions,
};

use crate::oamdp::BeliefState;
use crate::{oamdp::oamdp::OAMDP, traits::CommunicationCost};

use super::{
    communication_model::BakerCommunicationModel, BakerCommunicationAction, BakerJointAction,
};

impl<const N: usize> CommunicationCost
    for OAMDP<BakerCommunicationModel<N>, BakerGridMDP, BakerJointAction, N>
where
    Self: StatesActions<State = BeliefState<BakerGridState, N>, Action = BakerJointAction>,
{
    fn communication_cost(&self, _s: &Self::State, a: &Self::Action) -> f32 {
        let communication_cost = match &a.communication_action {
            BakerCommunicationAction::None => 0.0,
            __ => self.assumed_model.communication_model.communication_cost,
        };
        communication_cost
    }
}
