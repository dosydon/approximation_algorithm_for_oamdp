use mdp::mdp_traits::{ActionEnumerable, StatesActions};



use crate::{
    oamdp::oamdp::OAMDP,
    traits::{DomainAction, EnumerateDomainAction, EnumerateMessage, Message},
};

use super::{
    RecycleAction, RecycleCommunicationAction, RecycleCommunicationModel, RecycleJointAction,
    RecycleMDP,
};

impl<const K: usize, const N: usize> DomainAction
    for OAMDP<RecycleCommunicationModel<K>, RecycleMDP<K>, RecycleJointAction, N>
where
    Self: StatesActions<Action = RecycleJointAction>,
{
    type DomainAction = RecycleAction;
}

impl<const K: usize, const N: usize> EnumerateDomainAction
    for OAMDP<RecycleCommunicationModel<K>, RecycleMDP<K>, RecycleJointAction, N>
where
    Self: StatesActions<Action = RecycleJointAction>,
{
    fn enumerate_domain_actions(&self) -> std::slice::Iter<Self::DomainAction> {
        self.mdp.enumerate_actions()
    }
}

impl<const K: usize, const N: usize> Message
    for OAMDP<RecycleCommunicationModel<K>, RecycleMDP<K>, RecycleJointAction, N>
where
    Self: StatesActions<Action = RecycleJointAction>,
{
    type Message = RecycleCommunicationAction;
}

impl<const K: usize, const N: usize> EnumerateMessage
    for OAMDP<RecycleCommunicationModel<K>, RecycleMDP<K>, RecycleJointAction, N>
where
    Self: StatesActions<Action = RecycleJointAction>,
{
    fn enumerate_message(&self) -> std::slice::Iter<Self::Message> {
        self.assumed_model.messages.iter()
    }
}
