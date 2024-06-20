use mdp::{into_inner::Inner, simple_av_obstacle_avoidance::ObstacleAvoidanceAction};

use crate::traits::Set;

use super::communication_action::ObstacleAvoidanceCommunicationAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObstacleAvoidanceJointAction {
    pub(crate) domain_action: ObstacleAvoidanceAction,
    pub(crate) communication_action: ObstacleAvoidanceCommunicationAction,
}

impl ObstacleAvoidanceJointAction {
    pub fn new(
        domain_action: ObstacleAvoidanceAction,
        communication_action: ObstacleAvoidanceCommunicationAction,
    ) -> Self {
        ObstacleAvoidanceJointAction {
            domain_action,
            communication_action,
        }
    }
}

impl Inner for ObstacleAvoidanceJointAction {
    type Result = ObstacleAvoidanceAction;

    fn inner(self: &ObstacleAvoidanceJointAction) -> Self::Result {
        self.domain_action
    }
}

impl
    From<(
        ObstacleAvoidanceAction,
        ObstacleAvoidanceCommunicationAction,
    )> for ObstacleAvoidanceJointAction
{
    fn from(
        value: (
            ObstacleAvoidanceAction,
            ObstacleAvoidanceCommunicationAction,
        ),
    ) -> Self {
        ObstacleAvoidanceJointAction {
            domain_action: value.0,
            communication_action: value.1,
        }
    }
}

impl From<ObstacleAvoidanceAction> for ObstacleAvoidanceJointAction {
    fn from(a: ObstacleAvoidanceAction) -> Self {
        ObstacleAvoidanceJointAction {
            domain_action: a,
            communication_action: ObstacleAvoidanceCommunicationAction::None,
        }
    }
}

impl Set<ObstacleAvoidanceCommunicationAction> for ObstacleAvoidanceJointAction {
    fn set(&mut self, m: ObstacleAvoidanceCommunicationAction) {
        self.communication_action = m;
    }
}

impl Set<ObstacleAvoidanceAction> for ObstacleAvoidanceJointAction {
    fn set(&mut self, a: ObstacleAvoidanceAction) {
        self.domain_action = a;
    }
}
