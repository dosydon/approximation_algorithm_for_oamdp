use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;

use crate::observer_model::ImplicitCommunicationModel;
use crate::traits::BeliefOverGoal;

use mdp::blocks_world::{BlocksWorldAction, BlocksWorldMDPN, BlocksWorldStateN, LetterManager};
use mdp::into_inner::Inner;
use mdp::mdp_traits::*;
use mdp::state_enumerable_wrapper::StateEnumerableWrapper;
use std::fmt::Debug;
use std::hash::Hash;

impl<
        P,
        A: Copy + Eq + Debug + Hash + Inner<Result = BlocksWorldAction>,
        const N: usize,
        const NB: usize,
    > DisplayState<BeliefState<BlocksWorldStateN<NB>, N>>
    for OAMDP<ImplicitCommunicationModel<P, BlocksWorldMDPN<NB>, N>, BlocksWorldMDPN<NB>, A, N>
where
    Self: StatesActions<Action = A> + ActionEnumerable,
{
    fn display(&self, s: &BeliefState<BlocksWorldStateN<NB>, N>) {
        let b = s.get_belief_over_goal();
        for i in 0..N {
            let mdp = self.assumed_model.get_mdp_for_goal(i);
            let lm = LetterManager::new(mdp.letters);
            println!(
                "Belief over goal {}: {}",
                lm.goal_to_string(&mdp.goal),
                b[i].into_inner()
            );
        }
        self.mdp.display(&s.inner());
    }
}

impl<
        P,
        A: Copy + Eq + Debug + Hash + Inner<Result = BlocksWorldAction>,
        const N: usize,
        const NB: usize,
    > DisplayState<BeliefState<BlocksWorldStateN<NB>, N>>
    for OAMDP<
        ImplicitCommunicationModel<P, StateEnumerableWrapper<BlocksWorldMDPN<NB>>, N>,
        StateEnumerableWrapper<BlocksWorldMDPN<NB>>,
        A,
        N,
    >
where
    Self: StatesActions<Action = A> + ActionEnumerable,
{
    fn display(&self, s: &BeliefState<BlocksWorldStateN<NB>, N>) {
        let b = s.get_belief_over_goal();
        for i in 0..N {
            let mdp = &self.assumed_model.get_mdp_for_goal(i).mdp;
            let lm = LetterManager::new(mdp.letters);
            println!(
                "Belief over goal {}: {}",
                lm.goal_to_string(&mdp.goal),
                b[i].into_inner()
            );
        }
        self.mdp.mdp.display(&s.inner());
    }
}
