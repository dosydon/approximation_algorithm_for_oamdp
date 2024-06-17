// use super::BakerCommunicationAction;
use core::fmt::Debug;
use mdp::{baker_grid::BakerGridAction, into_inner::Inner};
use std::hash::Hash;

use crate::traits::{Message, Set};

use super::BakerCommunicationAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BakerJointAction {
    pub grid_action: BakerGridAction,
    pub(crate) communication_action: BakerCommunicationAction,
}

impl BakerJointAction {
    pub fn new(
        grid_action: BakerGridAction,
        communication_action: BakerCommunicationAction,
    ) -> Self {
        BakerJointAction {
            grid_action,
            communication_action,
        }
    }
}

impl Message for BakerJointAction {
    type Message = BakerCommunicationAction;
}

impl Inner for BakerJointAction {
    type Result = BakerGridAction;
    fn inner(&self) -> Self::Result {
        self.grid_action
    }
}

impl From<BakerJointAction> for BakerCommunicationAction {
    fn from(a: BakerJointAction) -> Self {
        a.communication_action
    }
}

impl From<(BakerGridAction, BakerCommunicationAction)> for BakerJointAction {
    fn from(pair: (BakerGridAction, BakerCommunicationAction)) -> Self {
        BakerJointAction::new(pair.0, pair.1)
    }
}

impl From<BakerGridAction> for BakerJointAction {
    fn from(a: BakerGridAction) -> Self {
        BakerJointAction::new(a, BakerCommunicationAction::None)
    }
}

impl Set<BakerCommunicationAction> for BakerJointAction {
    fn set(&mut self, m: BakerCommunicationAction) {
        self.communication_action = m;
    }
}

impl Set<BakerGridAction> for BakerJointAction {
    fn set(&mut self, a: BakerGridAction) {
        self.grid_action = a;
    }
}
