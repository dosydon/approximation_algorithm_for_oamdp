use mdp::mdp_traits::StatesActions;
use mdp::spelling::{SpellingMDPE, SpellingState};

use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;
use crate::traits::CommunicationCost;

use super::communication_action::SpellingCommunicationAction;
use super::communication_model::SpellingCommunicationModel;
use super::joint_action::SpellingJointAction;

impl<const NL: usize, const N: usize> CommunicationCost
    for OAMDP<SpellingCommunicationModel<NL, N>, SpellingMDPE<NL>, SpellingJointAction, N>
where
    Self: StatesActions<State = BeliefState<SpellingState<NL>, N>, Action = SpellingJointAction>,
{
    fn communication_cost(&self, _s: &Self::State, a: &Self::Action) -> f32 {
        match &a.communication_action {
            SpellingCommunicationAction::None => 0.0,
            _ => self.assumed_model.communication_cost,
        }
    }
}
