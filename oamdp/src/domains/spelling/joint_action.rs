use mdp::{into_inner::Inner, spelling::SpellingAction};

use crate::traits::Set;

use super::communication_action::SpellingCommunicationAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpellingJointAction {
    pub(crate) domain_action: SpellingAction,
    pub(crate) communication_action: SpellingCommunicationAction,
}

impl SpellingJointAction {
    pub fn new(
        domain_action: SpellingAction,
        communication_action: SpellingCommunicationAction,
    ) -> Self {
        Self {
            domain_action: domain_action,
            communication_action: communication_action,
        }
    }
}

impl Inner for SpellingJointAction {
    type Result = SpellingAction;
    fn inner(&self) -> SpellingAction {
        self.domain_action
    }
}

impl From<(SpellingAction, SpellingCommunicationAction)> for SpellingJointAction {
    fn from(value: (SpellingAction, SpellingCommunicationAction)) -> Self {
        SpellingJointAction {
            domain_action: value.0,
            communication_action: value.1,
        }
    }
}

impl From<SpellingAction> for SpellingJointAction {
    fn from(a: SpellingAction) -> Self {
        SpellingJointAction {
            domain_action: a,
            communication_action: SpellingCommunicationAction::None,
        }
    }
}

impl Set<SpellingCommunicationAction> for SpellingJointAction {
    fn set(&mut self, m: SpellingCommunicationAction) {
        self.communication_action = m;
    }
}

impl Set<SpellingAction> for SpellingJointAction {
    fn set(&mut self, a: SpellingAction) {
        self.domain_action = a;
    }
}
