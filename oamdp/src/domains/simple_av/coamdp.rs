use mdp::{
    mdp_traits::{ActionEnumerable, StatesActions},
    simple_av::{SimpleAVAction, SimpleAVVehicleInFrontMDP},
};

use std::slice::Iter;

use crate::traits::DomainAction;
use crate::{
    oamdp::oamdp::OAMDP,
    traits::{EnumerateDomainAction, EnumerateMessage, Message},
};

use super::{
    communication_model::AVCommunicationModel, joint_action::AVJointAction, AVCommunicationAction,
};

impl<const N: usize> DomainAction
    for OAMDP<AVCommunicationModel<N>, SimpleAVVehicleInFrontMDP, AVJointAction, N>
where
    Self: StatesActions<Action = AVJointAction>,
{
    type DomainAction = SimpleAVAction;
}

impl<const N: usize> EnumerateDomainAction
    for OAMDP<AVCommunicationModel<N>, SimpleAVVehicleInFrontMDP, AVJointAction, N>
where
    Self: StatesActions<Action = AVJointAction>,
{
    fn enumerate_domain_actions(&self) -> Iter<Self::DomainAction> {
        self.mdp.enumerate_actions()
    }
}

impl<const N: usize> Message
    for OAMDP<AVCommunicationModel<N>, SimpleAVVehicleInFrontMDP, AVJointAction, N>
where
    Self: StatesActions<Action = AVJointAction>,
{
    type Message = AVCommunicationAction;
}

impl<const N: usize> EnumerateMessage
    for OAMDP<AVCommunicationModel<N>, SimpleAVVehicleInFrontMDP, AVJointAction, N>
where
    Self: StatesActions<Action = AVJointAction>,
{
    fn enumerate_message(&self) -> std::slice::Iter<Self::Message> {
        self.assumed_model.messages.iter()
    }
}
