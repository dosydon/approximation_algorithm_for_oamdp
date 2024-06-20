use mdp::mdp_traits::{ActionEnumerable, StatesActions};
use mdp::simple_av_obstacle_avoidance::{ObstacleAvoidanceAction, ObstacleAvoidanceMDP};


use std::{slice::Iter};

use crate::traits::DomainAction;
use crate::{
    oamdp::oamdp::OAMDP,
    traits::{EnumerateDomainAction, EnumerateMessage, Message},
};

use super::communication_action::ObstacleAvoidanceCommunicationAction;
use super::communication_model::ObstacleAvoidanceCommunicationModel;
use super::joint_action::ObstacleAvoidanceJointAction;

impl<const N: usize> DomainAction
    for OAMDP<
        ObstacleAvoidanceCommunicationModel<N>,
        ObstacleAvoidanceMDP,
        ObstacleAvoidanceJointAction,
        N,
    >
where
    Self: StatesActions<Action = ObstacleAvoidanceJointAction>,
{
    type DomainAction = ObstacleAvoidanceAction;
}

impl<const N: usize> EnumerateDomainAction
    for OAMDP<
        ObstacleAvoidanceCommunicationModel<N>,
        ObstacleAvoidanceMDP,
        ObstacleAvoidanceJointAction,
        N,
    >
where
    Self: StatesActions<Action = ObstacleAvoidanceJointAction>,
{
    fn enumerate_domain_actions(&self) -> Iter<Self::DomainAction> {
        self.mdp.enumerate_actions()
    }
}

impl<const N: usize> Message
    for OAMDP<
        ObstacleAvoidanceCommunicationModel<N>,
        ObstacleAvoidanceMDP,
        ObstacleAvoidanceJointAction,
        N,
    >
where
    Self: StatesActions<Action = ObstacleAvoidanceJointAction>,
{
    type Message = ObstacleAvoidanceCommunicationAction;
}

impl<const N: usize> EnumerateMessage
    for OAMDP<
        ObstacleAvoidanceCommunicationModel<N>,
        ObstacleAvoidanceMDP,
        ObstacleAvoidanceJointAction,
        N,
    >
where
    Self: StatesActions<Action = ObstacleAvoidanceJointAction>,
{
    fn enumerate_message(&self) -> std::slice::Iter<Self::Message> {
        self.assumed_model.messages.iter()
    }
}
