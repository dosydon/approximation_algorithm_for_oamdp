use mdp::{
    mdp_traits::{ActionEnumerable, StatesActions},
    spelling::{SpellingAction, SpellingMDPE},
};

use crate::{
    oamdp::oamdp::OAMDP,
    traits::{DomainAction, EnumerateDomainAction, EnumerateMessage, Message},
};

use super::{
    communication_action::SpellingCommunicationAction,
    communication_model::SpellingCommunicationModel, joint_action::SpellingJointAction,
};

impl<const N: usize, const NL: usize> DomainAction
    for OAMDP<SpellingCommunicationModel<NL, N>, SpellingMDPE<NL>, SpellingJointAction, N>
where
    Self: StatesActions<Action = SpellingJointAction>,
{
    type DomainAction = SpellingAction;
}

impl<const N: usize, const NL: usize> EnumerateDomainAction
    for OAMDP<SpellingCommunicationModel<NL, N>, SpellingMDPE<NL>, SpellingJointAction, N>
where
    Self: StatesActions<Action = SpellingJointAction>,
{
    fn enumerate_domain_actions(&self) -> std::slice::Iter<Self::DomainAction> {
        self.mdp.enumerate_actions()
    }
}

impl<const N: usize, const NL: usize> Message
    for OAMDP<SpellingCommunicationModel<NL, N>, SpellingMDPE<NL>, SpellingJointAction, N>
where
    Self: StatesActions<Action = SpellingJointAction>,
{
    type Message = SpellingCommunicationAction;
}

impl<const N: usize, const NL: usize> EnumerateMessage
    for OAMDP<SpellingCommunicationModel<NL, N>, SpellingMDPE<NL>, SpellingJointAction, N>
where
    Self: StatesActions<Action = SpellingJointAction>,
{
    fn enumerate_message(&self) -> std::slice::Iter<Self::Message> {
        self.assumed_model.messages.iter()
    }
}
