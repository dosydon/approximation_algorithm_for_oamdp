use mdp::{
    into_inner::Inner,
    mdp_traits::{DisplayState, StatesActions},
    spelling::{SpellingMDP, SpellingMDPE, SpellingState},
};

use crate::{
    oamdp::{oamdp::OAMDP, BeliefState},
    observer_model::{ImplicitCommunicationModel, SoftmaxModel},
    traits::BeliefOverGoal,
};
use std::fmt::Debug;
use std::hash::Hash;

use super::communication_model::SpellingCommunicationModel;

impl<P, A: Eq + Copy + Debug + Hash, const N: usize, const NL: usize>
    DisplayState<BeliefState<SpellingState<NL>, N>>
    for OAMDP<ImplicitCommunicationModel<P, SpellingMDP<NL>, N>, SpellingMDP<NL>, A, N>
where
    Self: StatesActions<State = BeliefState<SpellingState<NL>, N>, Action = A>,
{
    fn display(&self, s: &BeliefState<SpellingState<NL>, N>) {
        let b = s.get_belief_over_goal();
        for i in 0..N {
            println!(
                "{:?}: {}",
                self.assumed_model.mdp_for_each_goal[i].goal, b[i]
            );
        }
        self.mdp.display(&s.inner());
    }
}

impl<P, A: Eq + Copy + Debug + Hash, const N: usize, const NL: usize>
    DisplayState<BeliefState<SpellingState<NL>, N>>
    for OAMDP<ImplicitCommunicationModel<P, SpellingMDPE<NL>, N>, SpellingMDPE<NL>, A, N>
where
    Self: StatesActions<State = BeliefState<SpellingState<NL>, N>, Action = A>,
{
    fn display(&self, s: &BeliefState<SpellingState<NL>, N>) {
        let b = s.get_belief_over_goal();
        for i in 0..N {
            println!(
                "{:?}: {}",
                self.assumed_model.mdp_for_each_goal[i].mdp.goal, b[i]
            );
        }
        self.mdp.display(&s.inner());
    }
}

impl<A: Eq + Copy + Debug + Hash, const NL: usize, const N: usize>
    DisplayState<BeliefState<SpellingState<NL>, N>>
    for OAMDP<SpellingCommunicationModel<NL, N>, SpellingMDP<NL>, A, N>
where
    Self: StatesActions<State = BeliefState<SpellingState<NL>, N>, Action = A>,
{
    fn display(&self, s: &BeliefState<SpellingState<NL>, N>) {
        let b = s.get_belief_over_goal();
        for i in 0..N {
            println!(
                "{:?}: {}",
                self.assumed_model.mdp_for_each_goal[i].mdp.goal, b[i]
            );
        }
        self.mdp.display(&s.inner());
    }
}

impl<A: Eq + Copy + Debug + Hash, const NL: usize, const N: usize>
    DisplayState<BeliefState<SpellingState<NL>, N>>
    for OAMDP<SpellingCommunicationModel<NL, N>, SpellingMDPE<NL>, A, N>
where
    Self: StatesActions<State = BeliefState<SpellingState<NL>, N>, Action = A>,
{
    fn display(&self, s: &BeliefState<SpellingState<NL>, N>) {
        let b = s.get_belief_over_goal();
        for i in 0..N {
            println!(
                "{:?}: {}",
                self.assumed_model.mdp_for_each_goal[i].mdp.goal, b[i]
            );
        }
        self.mdp.display(&s.inner());
    }
}
