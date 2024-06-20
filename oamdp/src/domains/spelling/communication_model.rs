use mdp::common::coordinate2::Coordinate2;
use mdp::mdp_traits::ActionEnumerable;
use mdp::mdp_traits::BuildFrom;
use mdp::mdp_traits::ExplicitTransition;
use mdp::policy::policy_traits::Policy;
use mdp::policy::softmax_policy::SoftmaxPolicy;
use mdp::spelling::Letter;
use mdp::spelling::Letter::*;
use mdp::spelling::SpellingMDP;
use mdp::spelling::SpellingMDPBuilder;
use mdp::spelling::SpellingState;
use mdp::state_enumerable_wrapper::StateEnumerableWrapper;
use mdp::value_iteration::value_iteration_ssp;
use mdp::value_iteration::ValueTable;

use crate::traits::ProbSassGivenTheta;

use super::communication_action::SpellingCommunicationAction;
use super::joint_action::SpellingJointAction;

pub struct SpellingCommunicationModel<const NL: usize, const N: usize> {
    pub(crate) mdp_for_each_goal: [StateEnumerableWrapper<SpellingMDP<NL>>; N],
    pub assumed_policy: Vec<SoftmaxPolicy<ValueTable<SpellingState<NL>>>>,
    pub(crate) communication_cost: f32,
    pub(crate) targets: [[Letter; NL]; N],
    pub(crate) messages: Vec<SpellingCommunicationAction>,
    pub(crate) letter_locs: [(usize, usize); NL],
}

impl<const NL: usize, const N: usize> SpellingCommunicationModel<NL, N> {
    pub fn new(
        mdp_for_each_goal: [StateEnumerableWrapper<SpellingMDP<NL>>; N],
        assumed_policy: Vec<SoftmaxPolicy<ValueTable<SpellingState<NL>>>>,
        communication_cost: f32,
        targets: [[Letter; NL]; N],
        letter_locs: [(usize, usize); NL],
    ) -> Self {
        SpellingCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            communication_cost: communication_cost,
            targets: targets,
            messages: vec![
                SpellingCommunicationAction::None,
                SpellingCommunicationAction::Announce(A),
                SpellingCommunicationAction::Announce(R),
                SpellingCommunicationAction::Announce(M),
                SpellingCommunicationAction::Announce(S),
            ],
            letter_locs,
        }
    }

    fn loc_id(&self, s: &SpellingState<NL>) -> Option<usize> {
        for (i, loc) in self.letter_locs.iter().enumerate() {
            if s.coord == Coordinate2::new(loc.0 as i32, loc.1 as i32) {
                return Some(i);
            }
        }
        None
    }

    fn communication_probability(
        &self,
        id: usize,
        s: &SpellingState<NL>,
        a: &SpellingCommunicationAction,
        _ss: &SpellingState<NL>,
    ) -> f32 {
        if let Some(loc_id) = self.loc_id(s) {
            match a {
                SpellingCommunicationAction::Announce(l) => {
                    if self.targets[id][loc_id] == *l {
                        0.3
                    } else {
                        0.05
                    }
                }
                SpellingCommunicationAction::None => 0.5,
            }
        } else {
            match a {
                SpellingCommunicationAction::Announce(_) => 0.025,
                SpellingCommunicationAction::None => 0.9,
            }
        }
    }
}

impl<'a, const NL: usize, const N: usize> ProbSassGivenTheta<SpellingState<NL>, SpellingJointAction>
    for &'a SpellingCommunicationModel<NL, N>
{
    fn prob_sass_given_theta(
        self,
        id: usize,
        s: &SpellingState<NL>,
        a: &SpellingJointAction,
        ss: &SpellingState<NL>,
    ) -> f32 {
        self.mdp_for_each_goal[id]
            .enumerate_actions()
            .map(|aa| {
                self.assumed_policy[id].get_probability(s, aa, &self.mdp_for_each_goal[id])
                    * self.mdp_for_each_goal[id].p(s, aa, ss)
                    * self.communication_probability(id, s, &a.communication_action, ss)
            })
            .sum()
    }
}

impl<'a, const NL: usize, const N: usize> ProbSassGivenTheta<SpellingState<NL>, SpellingJointAction>
    for &'a mut SpellingCommunicationModel<NL, N>
{
    fn prob_sass_given_theta(
        self,
        id: usize,
        s: &SpellingState<NL>,
        a: &SpellingJointAction,
        ss: &SpellingState<NL>,
    ) -> f32 {
        self.mdp_for_each_goal[id]
            .enumerate_actions()
            .map(|aa| {
                self.assumed_policy[id].get_probability(s, aa, &self.mdp_for_each_goal[id])
                    * self.mdp_for_each_goal[id].p(s, aa, ss)
                    * self.communication_probability(id, s, &a.communication_action, ss)
            })
            .sum()
    }
}

impl SpellingCommunicationModel<4, 3> {
    pub fn from_targets(
        builder: &SpellingMDPBuilder<4>,
        targets: [[Letter; 4]; 3],
        communication_cost: f32,
        beta: f32,
    ) -> SpellingCommunicationModel<4, 3> {
        let mdp_for_each_goal = [
            builder.build_from(&targets[0]),
            builder.build_from(&targets[1]),
            builder.build_from(&targets[2]),
        ];

        let mut assumed_policy = vec![];
        for i in 0..3 {
            let vt = value_iteration_ssp(&mdp_for_each_goal[i]);
            let policy = SoftmaxPolicy::new(beta, vt);
            assumed_policy.push(policy);
        }

        SpellingCommunicationModel::new(
            mdp_for_each_goal,
            assumed_policy,
            communication_cost,
            targets,
            [(0, 0), (0, 4), (4, 0), (4, 4)],
        )
    }
}
