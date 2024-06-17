use mdp::{
    baker_grid::{baker_grid_reset::BakerGridResetMDP, BakerGridState, GridAndGoals},
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

impl<A: Eq + Copy + Debug + Hash, const N: usize> DisplayState<BeliefState<BakerGridState, N>>
    for OAMDP<SoftmaxModel<BakerGridResetMDP, N>, BakerGridResetMDP, A, N>
where
    Self: StatesActions<State = BeliefState<BakerGridState, N>, Action = A>,
{
    fn display(&self, s: &BeliefState<BakerGridState, N>) {
        let env = self.mdp.mdp.grid2d.clone();
        let possible_goals: Vec<_> = self
            .assumed_model
            .mdp_for_each_goal
            .iter()
            .map(|mdp| mdp.mdp.goal)
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
