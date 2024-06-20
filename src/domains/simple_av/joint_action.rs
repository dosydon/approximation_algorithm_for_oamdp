use mdp::{into_inner::Inner, simple_av::SimpleAVAction};

use crate::traits::Set;

use super::communication_action::AVCommunicationAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AVJointAction {
    pub(crate) av_action: SimpleAVAction,
    pub(crate) communication_action: AVCommunicationAction,
}

impl AVJointAction {
    pub(crate) fn new(
        av_action: SimpleAVAction,
        communication_action: AVCommunicationAction,
    ) -> Self {
        AVJointAction {
            av_action,
            communication_action,
        }
    }
}

impl Inner for AVJointAction {
    type Result = SimpleAVAction;
    fn inner(&self) -> SimpleAVAction {
        self.av_action
    }
}

impl Set<SimpleAVAction> for AVJointAction {
    fn set(&mut self, a: SimpleAVAction) {
        self.av_action = a;
    }
}

impl Set<AVCommunicationAction> for AVJointAction {
    fn set(&mut self, a: AVCommunicationAction) {
        self.communication_action = a;
    }
}

impl From<(SimpleAVAction, AVCommunicationAction)> for AVJointAction {
    fn from(value: (SimpleAVAction, AVCommunicationAction)) -> Self {
        AVJointAction {
            av_action: value.0,
            communication_action: value.1,
        }
    }
}

impl From<SimpleAVAction> for AVJointAction {
    fn from(value: SimpleAVAction) -> Self {
        AVJointAction {
            av_action: value,
            communication_action: AVCommunicationAction::None,
        }
    }
}
