use mdp::{
    mdp_traits::BuildFrom,
    policy::{policy_traits::GetActionProbability, softmax_policy::SoftmaxPolicy},
    simple_av_obstacle_avoidance::{
        ObstacleAvoidanceBuilder, ObstacleAvoidanceMDP, ObstacleAvoidanceParameter,
        ObstacleAvoidanceState,
    },
    value_iteration::{value_iteration_ssp, ValueTable},
};

use crate::traits::ProbSassGivenTheta;

use super::{
    communication_action::ObstacleAvoidanceCommunicationAction,
    joint_action::ObstacleAvoidanceJointAction,
};

pub struct ObstacleAvoidanceCommunicationModel<const N: usize> {
    mdp_for_each_goal: [ObstacleAvoidanceMDP; N],
    pub assumed_policy: Vec<SoftmaxPolicy<ValueTable<ObstacleAvoidanceState>>>,
    pub(crate) communication_cost: f32,
    pub targets: [ObstacleAvoidanceParameter; N],
    pub(crate) messages: Vec<ObstacleAvoidanceCommunicationAction>,
}

impl<const N: usize> ObstacleAvoidanceCommunicationModel<N> {
    pub fn new(
        mdp_for_each_goal: [ObstacleAvoidanceMDP; N],
        assumed_policy: Vec<SoftmaxPolicy<ValueTable<ObstacleAvoidanceState>>>,
        communication_cost: f32,
        targets: [ObstacleAvoidanceParameter; N],
    ) -> Self {
        ObstacleAvoidanceCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            communication_cost: communication_cost,
            targets: targets,
            messages: vec![
                ObstacleAvoidanceCommunicationAction::Acknowledge,
                ObstacleAvoidanceCommunicationAction::None,
            ],
        }
    }

    fn communication_probability(
        &self,
        id: usize,
        s: &ObstacleAvoidanceState,
        a: &ObstacleAvoidanceCommunicationAction,
    ) -> f32 {
        if s.ego_vehicle.y < 8 {
            match self.targets[id] {
                ObstacleAvoidanceParameter::AwareNotYielding => match a {
                    ObstacleAvoidanceCommunicationAction::Acknowledge => 0.3,
                    ObstacleAvoidanceCommunicationAction::None => 0.7,
                },
                ObstacleAvoidanceParameter::AwareYielding => match a {
                    ObstacleAvoidanceCommunicationAction::Acknowledge => 0.3,
                    ObstacleAvoidanceCommunicationAction::None => 0.7,
                },
                ObstacleAvoidanceParameter::NotAwareNotYielding => match a {
                    ObstacleAvoidanceCommunicationAction::Acknowledge => 0.1,
                    ObstacleAvoidanceCommunicationAction::None => 0.9,
                },
            }
        } else {
            match a {
                ObstacleAvoidanceCommunicationAction::Acknowledge => 0.1,
                ObstacleAvoidanceCommunicationAction::None => 0.9,
            }
        }
    }
}

impl ObstacleAvoidanceCommunicationModel<3> {
    pub fn from_targets(
        partial_mdp: &ObstacleAvoidanceBuilder,
        targets: [ObstacleAvoidanceParameter; 3],
        communication_cost: f32,
    ) -> ObstacleAvoidanceCommunicationModel<3> {
        let mdp_for_each_goal = [
            partial_mdp.build_from(&targets[0]),
            partial_mdp.build_from(&targets[1]),
            partial_mdp.build_from(&targets[2]),
        ];

        let mut assumed_policy = vec![];
        for i in 0..3 {
            let vt = value_iteration_ssp(&mdp_for_each_goal[i]);
            let policy = SoftmaxPolicy::new(0.1, vt);
            assumed_policy.push(policy);
        }

        ObstacleAvoidanceCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            communication_cost: communication_cost,
            targets: targets,
            messages: vec![
                ObstacleAvoidanceCommunicationAction::Acknowledge,
                ObstacleAvoidanceCommunicationAction::None,
            ],
        }
    }
}

impl<'a, const N: usize> ProbSassGivenTheta<ObstacleAvoidanceState, ObstacleAvoidanceJointAction>
    for &'a ObstacleAvoidanceCommunicationModel<N>
{
    fn prob_sass_given_theta(
        self,
        id: usize,
        s: &ObstacleAvoidanceState,
        a: &ObstacleAvoidanceJointAction,
        _ss: &ObstacleAvoidanceState,
    ) -> f32 {
        self.assumed_policy[id].get_action_probability(
            s,
            &a.domain_action,
            &self.mdp_for_each_goal[id],
        ) * self.communication_probability(id, s, &a.communication_action)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use mdp::{
        simple_av::VehicleConfiguration,
        simple_av_obstacle_avoidance::{Lane, VehicleConfigurationLane},
    };

    #[test]
    fn test_obstacle_avoidance_communication_model() {
        let start = ObstacleAvoidanceState::new(
            VehicleConfigurationLane::new(0, 3, Lane::Center),
            VehicleConfiguration::new(0, 2),
        );
        let targets = [
            ObstacleAvoidanceParameter::AwareNotYielding,
            ObstacleAvoidanceParameter::AwareYielding,
            ObstacleAvoidanceParameter::NotAwareNotYielding,
        ];
        let builder = ObstacleAvoidanceBuilder::new(30, 4)
            .set_collision_zone(12, 18)
            .set_start_state(start);
        let communication_model =
            ObstacleAvoidanceCommunicationModel::from_targets(&builder, targets, 1.0);

        let s = ObstacleAvoidanceState::new(
            VehicleConfigurationLane::new(0, 3, Lane::Center),
            VehicleConfiguration::new(0, 2),
        );
        let a = ObstacleAvoidanceCommunicationAction::Acknowledge;

        println!(
            "{}",
            communication_model.communication_probability(0, &s, &a)
        );
    }
}
