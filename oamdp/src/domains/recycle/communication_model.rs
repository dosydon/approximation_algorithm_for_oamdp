use mdp::{
    mdp_traits::{ActionEnumerable, ExplicitTransition},
    policy::{policy_traits::GetActionProbability, softmax_policy::SoftmaxPolicy},
    value_iteration::{value_iteration_ssp, ValueTable},
};

use crate::traits::ProbSassGivenTheta;

use super::{
    action::{RecycleCommunicationAction, RecycleJointAction},
    builder::RecycleMDPBuilder,
    example::enumerate_communication_actions,
    location::Location,
    mdp::RecycleMDP,
    state::RecycleState,
};
pub struct RecycleCommunicationModel<const K: usize> {
    mdp_for_each_goal: Vec<RecycleMDP<K>>,
    pub assumed_policy: Vec<SoftmaxPolicy<ValueTable<RecycleState<K>>>>,
    pub(crate) communication_cost: f32,
    pub(crate) targets: Vec<[Location; 3]>,
    pub(crate) messages: Vec<RecycleCommunicationAction>,
    pub(crate) kinds: [usize; K],
}

impl<const K: usize> RecycleCommunicationModel<K> {
    pub fn new(
        mdp_for_each_goal: Vec<RecycleMDP<K>>,
        assumed_policy: Vec<SoftmaxPolicy<ValueTable<RecycleState<K>>>>,
        communication_cost: f32,
        targets: Vec<[Location; 3]>,
        kinds: [usize; K],
    ) -> Self {
        RecycleCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            communication_cost: communication_cost,
            targets: targets,
            messages: enumerate_communication_actions(),
            kinds: kinds,
        }
    }

    fn number_of_announcements_consistent(&self, id: usize, object_id: usize) -> usize {
        let mut count = 0;
        for a in self.messages.iter() {
            match a {
                RecycleCommunicationAction::Announce(loc) => {
                    if self.targets[id][self.kinds[object_id]] == *loc {
                        count += 1;
                    }
                }
                _ => (),
            }
        }
        count
    }

    fn object_id_in_hand(&self, s: &RecycleState<K>) -> Option<usize> {
        for i in 0..K {
            if s.locs[i] == Location::InHand {
                return Some(i);
            }
        }
        None
    }

    fn communication_probability(
        &self,
        id: usize,
        s: &RecycleState<K>,
        a: &RecycleCommunicationAction,
        _ss: &RecycleState<K>,
    ) -> f32 {
        if let Some(object_id) = self.object_id_in_hand(s) {
            match a {
                RecycleCommunicationAction::Announce(loc) => {
                    let count = self.number_of_announcements_consistent(id, object_id);
                    if self.targets[id][self.kinds[object_id]] == *loc {
                        0.4 / count as f32
                    } else {
                        0.1 / (self.messages.len() - 1 - count) as f32
                    }
                }
                RecycleCommunicationAction::None => 0.5,
            }
        } else {
            match a {
                RecycleCommunicationAction::Announce(_) => 0.0,
                RecycleCommunicationAction::None => 1.0,
            }
        }
    }
}

impl<'a, const K: usize> ProbSassGivenTheta<RecycleState<K>, RecycleJointAction>
    for &'a RecycleCommunicationModel<K>
{
    fn prob_sass_given_theta(
        self,
        id: usize,
        s: &RecycleState<K>,
        a: &RecycleJointAction,
        ss: &RecycleState<K>,
    ) -> f32 {
        self.mdp_for_each_goal[id]
            .enumerate_actions()
            .map(|aa| {
                self.assumed_policy[id].get_action_probability(s, aa, &self.mdp_for_each_goal[id])
                    * self.mdp_for_each_goal[id].p(s, aa, ss)
                    * self.communication_probability(id, s, &a.communication_action, ss)
            })
            .sum()
    }
}

impl<'a, const K: usize> ProbSassGivenTheta<RecycleState<K>, RecycleJointAction>
    for &'a mut RecycleCommunicationModel<K>
{
    fn prob_sass_given_theta(
        self,
        id: usize,
        s: &RecycleState<K>,
        a: &RecycleJointAction,
        ss: &RecycleState<K>,
    ) -> f32 {
        self.mdp_for_each_goal[id]
            .enumerate_actions()
            .map(|aa| {
                self.assumed_policy[id].get_action_probability(s, aa, &self.mdp_for_each_goal[id])
                    * self.mdp_for_each_goal[id].p(s, aa, ss)
                    * self.communication_probability(id, s, &a.communication_action, ss)
            })
            .sum()
    }
}

impl<const K: usize> RecycleCommunicationModel<K> {
    pub fn from_targets(
        targets: Vec<[Location; 3]>,
        communication_cost: f32,
        builder: &RecycleMDPBuilder<K>,
    ) -> Self {
        let mut mdp_for_each_goal = Vec::new();
        for target in targets.iter() {
            mdp_for_each_goal.push(builder.build(*target));
        }

        let mut assumed_policy = vec![];
        for i in 0..mdp_for_each_goal.len() {
            let vt = value_iteration_ssp(&mdp_for_each_goal[i]);
            let policy = SoftmaxPolicy::new(0.3, vt);
            assumed_policy.push(policy);
        }

        RecycleCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            communication_cost: communication_cost,
            targets: targets,
            messages: enumerate_communication_actions(),
            kinds: builder.kinds,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use assert_approx_eq::assert_approx_eq;
    use mdp::{
        episode_runner::EpisodeRunner, mdp_traits::InitialState,
        policy::tabular_policy::TabularPolicy, value_iteration::value_iteration_ssp,
    };
    use rand::thread_rng;

    #[test]
    fn test_value_iteration() {
        let mdp = RecycleMDP::new(
            [Location::Compost, Location::Recycle, Location::Trash],
            [Location::Trash, Location::Trash, Location::Trash],
            [0, 1, 2],
            0.8,
        );
        let vt = value_iteration_ssp(&mdp);
        let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        let mut runner = EpisodeRunner::new(&mdp, &tabular_policy, mdp.initial_state());
        let mut rng = thread_rng();
        for (s, a, c, ss) in runner.into_iter_with(&mut rng) {
            println!("{:?} {:?} {:?} {:?}", s, a, c, ss);
        }
        assert_approx_eq!(0.5, vt.get_value(&mdp.initial_state()));
    }

    #[test]
    fn test_recycle_communication_model() {
        let mdp0 = RecycleMDP::new(
            [Location::Compost, Location::Recycle, Location::Trash],
            [Location::Trash, Location::Trash, Location::Trash],
            [0, 1, 2],
            0.5,
        );
        let vt0 = value_iteration_ssp(&mdp0);
        let policy0 = SoftmaxPolicy::new(0.3, vt0);

        let mdp1 = RecycleMDP::new(
            [Location::Compost, Location::Compost, Location::Trash],
            [Location::Trash, Location::Trash, Location::Trash],
            [0, 1, 2],
            0.9,
        );
        let vt1 = value_iteration_ssp(&mdp1);
        let policy1 = SoftmaxPolicy::new(0.3, vt1);

        let cm = RecycleCommunicationModel::new(
            vec![mdp0, mdp1],
            vec![policy0, policy1],
            0.1,
            vec![
                [Location::Compost, Location::Recycle, Location::Trash],
                [Location::Compost, Location::Compost, Location::Trash],
            ],
            [0, 1, 2],
        );

        let s = RecycleState::new([Location::Trash, Location::InHand, Location::Trash]);
        let ss = RecycleState::new([Location::Trash, Location::Compost, Location::Trash]);
        println!(
            "{}",
            cm.communication_probability(0, &s, &RecycleCommunicationAction::None, &ss)
        );
        println!(
            "{}",
            cm.communication_probability(
                0,
                &s,
                &RecycleCommunicationAction::Announce(Location::Compost),
                &ss
            )
        );
        println!(
            "{}",
            cm.communication_probability(
                0,
                &s,
                &RecycleCommunicationAction::Announce(Location::Recycle),
                &ss
            )
        );
        println!(
            "{}",
            cm.communication_probability(
                0,
                &s,
                &RecycleCommunicationAction::Announce(Location::Trash),
                &ss
            )
        );
    }
}
