use mdp::mdp_traits::DisplayState;

use crate::domains::recycle::Location;

use super::{RecycleCommunicationModel, RecycleMDP, RecycleState};
use mdp::{into_inner::Inner, mdp_traits::StatesActions};

use crate::{
    oamdp::{oamdp::OAMDP, BeliefState},
    traits::BeliefOverGoal,
};
use std::fmt::Debug;
use std::hash::Hash;

impl<const K: usize> DisplayState<RecycleState<K>> for RecycleMDP<K> {
    fn display(&self, s: &RecycleState<K>) {
        for i in 0..K {
            println!(
                "Item {} of Kind {} -> {:?}: currently at {}",
                i,
                self.kinds[i],
                self.target[self.kinds[i]],
                match s.locs[i] {
                    Location::Compost => "Compost",
                    Location::Recycle => "Recycle",
                    Location::Trash => "Trash",
                    Location::InHand => "InHand",
                }
            );
        }
    }
}

impl<A: Eq + Copy + Debug + Hash, const N: usize, const K: usize>
    DisplayState<BeliefState<RecycleState<K>, N>>
    for OAMDP<RecycleCommunicationModel<K>, RecycleMDP<K>, A, N>
where
    Self: StatesActions<State = BeliefState<RecycleState<K>, N>, Action = A>,
{
    fn display(&self, s: &BeliefState<RecycleState<K>, N>) {
        let b = s.get_belief_over_goal();
        for i in 0..N {
            println!("{:?}: {}", self.assumed_model.targets[i], b[i].into_inner());
        }
        self.mdp.display(&s.inner());
    }
}
