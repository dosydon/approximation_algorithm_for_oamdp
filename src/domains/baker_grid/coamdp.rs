use mdp::{
    baker_grid::{BakerGridAction, BakerGridMDP},
    mdp_traits::{ActionEnumerable, StatesActions},
};

use std::slice::Iter;

use crate::traits::DomainAction;
use crate::{
    oamdp::oamdp::OAMDP,
    traits::{EnumerateDomainAction, EnumerateMessage, Message},
};

use super::{
    communication_model::BakerCommunicationModel, BakerCommunicationAction, BakerJointAction,
};

impl<const N: usize> DomainAction
    for OAMDP<BakerCommunicationModel<N>, BakerGridMDP, BakerJointAction, N>
where
    Self: StatesActions<Action = BakerJointAction>,
{
    type DomainAction = BakerGridAction;
}

impl<const N: usize> EnumerateDomainAction
    for OAMDP<BakerCommunicationModel<N>, BakerGridMDP, BakerJointAction, N>
where
    Self: StatesActions<Action = BakerJointAction>,
{
    fn enumerate_domain_actions(&self) -> Iter<Self::DomainAction> {
        self.mdp.enumerate_actions()
    }
}

impl<const N: usize> Message
    for OAMDP<BakerCommunicationModel<N>, BakerGridMDP, BakerJointAction, N>
where
    Self: StatesActions<Action = BakerJointAction>,
{
    type Message = BakerCommunicationAction;
}

impl<const N: usize> EnumerateMessage
    for OAMDP<BakerCommunicationModel<N>, BakerGridMDP, BakerJointAction, N>
where
    Self: StatesActions<Action = BakerJointAction>,
{
    fn enumerate_message(&self) -> std::slice::Iter<Self::Message> {
        self.assumed_model
            .communication_model
            .communication_actions
            .iter()
    }
}

#[cfg(test)]
mod tests {

    use mdp::mdp_traits::Build;

    use crate::{domains::baker_grid::BakerCOAMDPBuilder, traits::EnumerateMessage};

    #[test]
    fn test_enumerate_messages() {
        let builder = BakerCOAMDPBuilder::new(1);
        let oamdp = builder.build();
        assert_eq!(oamdp.enumerate_message().collect::<Vec<_>>().len(), 2);
    }
}
