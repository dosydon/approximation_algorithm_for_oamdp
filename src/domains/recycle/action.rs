use mdp::into_inner::Inner;
use serde::{Deserialize, Serialize};

use crate::traits::Set;

use super::location::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecycleAction {
    Moveto(usize, Location),
    PickUp(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecycleCommunicationAction {
    Announce(Location),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecycleJointAction {
    pub(crate) domain_action: RecycleAction,
    pub(crate) communication_action: RecycleCommunicationAction,
}

impl RecycleJointAction {
    pub fn new(
        domain_action: RecycleAction,
        communication_action: RecycleCommunicationAction,
    ) -> Self {
        Self {
            domain_action: domain_action,
            communication_action: communication_action,
        }
    }
}

impl Inner for RecycleJointAction {
    type Result = RecycleAction;
    fn inner(&self) -> RecycleAction {
        self.domain_action
    }
}

impl From<(RecycleAction, RecycleCommunicationAction)> for RecycleJointAction {
    fn from(pair: (RecycleAction, RecycleCommunicationAction)) -> Self {
        Self::new(pair.0, pair.1)
    }
}

impl From<RecycleAction> for RecycleJointAction {
    fn from(a: RecycleAction) -> Self {
        Self::new(a, RecycleCommunicationAction::None)
    }
}

impl Set<RecycleCommunicationAction> for RecycleJointAction {
    fn set(&mut self, m: RecycleCommunicationAction) {
        self.communication_action = m;
    }
}

impl Set<RecycleAction> for RecycleJointAction {
    fn set(&mut self, m: RecycleAction) {
        self.domain_action = m;
    }
}
