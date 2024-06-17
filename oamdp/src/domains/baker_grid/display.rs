use mdp::{
    baker_grid::{BakerGridMDP, BakerGridState, GridAndGoals},
    into_inner::Inner,
    mdp_traits::{DisplayState, StatesActions},
};

use crate::{
    oamdp::{oamdp::OAMDP, BeliefState},
    observer_model::SoftmaxModel,
    traits::BeliefOverGoal,
};
use std::fmt::Debug;
use std::hash::Hash;

use super::communication_model::BakerCommunicationModel;

impl<A: Eq + Copy + Debug + Hash, const N: usize> DisplayState<BeliefState<BakerGridState, N>>
    for OAMDP<SoftmaxModel<BakerGridMDP, N>, BakerGridMDP, A, N>
where
    Self: StatesActions<State = BeliefState<BakerGridState, N>, Action = A>,
{
    fn display(&self, s: &BeliefState<BakerGridState, N>) {
        let env = self.mdp.grid2d.clone();
        let possible_goals: Vec<_> = self
            .assumed_model
            .mdp_for_each_goal
            .iter()
            .map(|mdp| mdp.goal)
            .map(|s| (s.i as usize, s.j as usize))
            .collect();
        let grid_and_goals = GridAndGoals::new(
            env,
            possible_goals,
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
            ],
        );
        println!("{:?}", s.get_belief_over_goal());
        grid_and_goals.display(&s.inner());
    }
}

impl<A: Eq + Copy + Debug + Hash, const N: usize> DisplayState<BeliefState<BakerGridState, N>>
    for OAMDP<BakerCommunicationModel<N>, BakerGridMDP, A, N>
where
    Self: StatesActions<State = BeliefState<BakerGridState, N>, Action = A>,
{
    fn display(&self, s: &BeliefState<BakerGridState, N>) {
        let env = self.mdp.grid2d.clone();
        let possible_goals: Vec<_> = self
            .assumed_model
            .mdp_for_each_goal
            .iter()
            .map(|mdp| mdp.goal)
            .map(|s| (s.i as usize, s.j as usize))
            .collect();
        let labels = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ];
        let grid_and_goals = GridAndGoals::new(env, possible_goals, labels.clone());
        let b = s.get_belief_over_goal();
        for i in 0..N {
            println!(
                "Belief over goal {} {:?}: {:?}",
                labels[i],
                self.assumed_model.communication_model.shapes[i],
                b[i].into_inner(),
            );
        }
        grid_and_goals.display(&s.inner());
    }
}
