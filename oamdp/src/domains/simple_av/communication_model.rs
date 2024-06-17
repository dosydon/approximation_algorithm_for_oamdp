use mdp::mdp_traits::BuildFrom;
use mdp::policy::policy_traits::Policy;
use mdp::simple_av::{SimpleAVParameter, SimpleAVPartialMDP, SimpleAVVehicleInFrontState};
use mdp::value_iteration::{value_iteration_ssp, ValueTable};
use mdp::{policy::softmax_policy::SoftmaxPolicy, simple_av::SimpleAVVehicleInFrontMDP};

use crate::traits::ProbSassGivenTheta;

use super::joint_action::AVJointAction;
use super::AVCommunicationAction;

pub struct AVCommunicationModel<const N: usize> {
    mdp_for_each_goal: [SimpleAVVehicleInFrontMDP; N],
    pub assumed_policy: Vec<SoftmaxPolicy<ValueTable<SimpleAVVehicleInFrontState>>>,
    pub(crate) communication_cost: f32,
    pub(crate) targets: [SimpleAVParameter; N],
    pub(crate) messages: Vec<AVCommunicationAction>,
}

impl<const N: usize> AVCommunicationModel<N> {
    pub fn new(
        mdp_for_each_goal: [SimpleAVVehicleInFrontMDP; N],
        assumed_policy: Vec<SoftmaxPolicy<ValueTable<SimpleAVVehicleInFrontState>>>,
        communication_cost: f32,
        targets: [SimpleAVParameter; N],
    ) -> Self {
        AVCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            communication_cost: communication_cost,
            targets: targets,
            messages: vec![AVCommunicationAction::Flash, AVCommunicationAction::None],
        }
    }

    fn communication_probability(
        &self,
        id: usize,
        _s: &SimpleAVVehicleInFrontState,
        a: &AVCommunicationAction,
        _ss: &SimpleAVVehicleInFrontState,
    ) -> f32 {
        match self.targets[id] {
            SimpleAVParameter::NonYield(_, _, _) => match a {
                AVCommunicationAction::Flash => 0.1,
                AVCommunicationAction::None => 0.9,
            },
            SimpleAVParameter::YouHaveLightOff(_, _, _) => match a {
                AVCommunicationAction::Flash => 0.3,
                AVCommunicationAction::None => 0.7,
            },
            SimpleAVParameter::Stopping(lb, _) => {
                if lb <= 10 {
                    match a {
                        AVCommunicationAction::Flash => 0.3,
                        AVCommunicationAction::None => 0.7,
                    }
                } else {
                    match a {
                        AVCommunicationAction::Flash => 0.1,
                        AVCommunicationAction::None => 0.9,
                    }
                }
            }
        }
    }
}

impl AVCommunicationModel<3> {
    pub fn from_targets(
        partial_mdp: &SimpleAVPartialMDP,
        targets: [SimpleAVParameter; 3],
        communication_cost: f32,
    ) -> AVCommunicationModel<3> {
        let mdp_for_each_goal = [
            partial_mdp.build_from(targets[0]),
            partial_mdp.build_from(targets[1]),
            partial_mdp.build_from(targets[2]),
        ];

        let mut assumed_policy = vec![];
        for i in 0..3 {
            let vt = value_iteration_ssp(&mdp_for_each_goal[i]);
            let policy = SoftmaxPolicy::new(0.1, vt);
            assumed_policy.push(policy);
        }

        AVCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            communication_cost: communication_cost,
            targets: targets,
            messages: vec![AVCommunicationAction::Flash, AVCommunicationAction::None],
        }
    }
}

impl<'a, const N: usize> ProbSassGivenTheta<SimpleAVVehicleInFrontState, AVJointAction>
    for &'a AVCommunicationModel<N>
{
    fn prob_sass_given_theta(
        self,
        id: usize,
        s: &SimpleAVVehicleInFrontState,
        a: &AVJointAction,
        ss: &SimpleAVVehicleInFrontState,
    ) -> f32 {
        self.assumed_policy[id].get_probability(s, &a.av_action, &self.mdp_for_each_goal[id])
            * self.communication_probability(id, s, &a.communication_action, ss)
    }
}

#[cfg(test)]
mod tests {

    use mdp::simple_av::{SimpleAVState, VehicleConfiguration};

    use super::*;

    #[test]
    fn test_from_targets() {
        let targets = [
            SimpleAVParameter::NonYield(35, 2, 3),
            SimpleAVParameter::Stopping(10, 13),
            SimpleAVParameter::YouHaveLightOff(35, 2, 3),
        ];
        let communication_cost = 0.0;
        let partial_mdp = SimpleAVPartialMDP::new(0, 40, -2, 5, SimpleAVState::new(0, 2));
        let model = AVCommunicationModel::from_targets(&partial_mdp, targets, communication_cost);

        assert_eq!(model.targets[0], SimpleAVParameter::NonYield(35, 2, 3));
        assert_eq!(model.targets[1], SimpleAVParameter::Stopping(10, 13));
        assert_eq!(
            model.targets[2],
            SimpleAVParameter::YouHaveLightOff(35, 2, 3)
        );
        assert_eq!(model.communication_cost, 0.0);

        let s = SimpleAVVehicleInFrontState::new(
            VehicleConfiguration::new(0, 2),
            VehicleConfiguration { y: 5, dy: 2 },
        );
        let ss = SimpleAVVehicleInFrontState::new(
            VehicleConfiguration::new(2, 2),
            VehicleConfiguration { y: 7, dy: 2 },
        );
        assert_eq!(
            model.communication_probability(0, &s, &AVCommunicationAction::Flash, &ss),
            0.1
        );
        assert_eq!(
            model.communication_probability(2, &s, &AVCommunicationAction::Flash, &ss),
            0.3
        );
    }
}
