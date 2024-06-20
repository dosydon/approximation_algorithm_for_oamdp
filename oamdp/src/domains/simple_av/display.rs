use mdp::{
    into_inner::Inner,
    mdp_traits::{DisplayState, StatesActions},
    simple_av::{SimpleAVAction, SimpleAVVehicleInFrontMDP, SimpleAVVehicleInFrontState},
};
use std::fmt::Debug;
use std::hash::Hash;

use crate::{
    oamdp::{oamdp::OAMDP, BeliefState},
    traits::BeliefOverGoal,
};

use super::communication_model::AVCommunicationModel;

impl<A: Eq + Copy + Debug + Hash + Inner<Result = SimpleAVAction>, const N: usize>
    DisplayState<BeliefState<SimpleAVVehicleInFrontState, N>>
    for OAMDP<AVCommunicationModel<N>, SimpleAVVehicleInFrontMDP, A, N>
where
    Self: StatesActions<Action = A>,
{
    fn display(&self, s: &BeliefState<SimpleAVVehicleInFrontState, N>) {
        let b = s.get_belief_over_goal();
        println!("{:?}", b);
        self.mdp.display(&s.inner());
    }
}
