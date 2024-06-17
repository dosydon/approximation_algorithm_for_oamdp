use crate::{mdp_traits::BuildFrom, state_enumerable_wrapper::StateEnumerableWrapper};

use super::{
    blocks_world_partial_mdp::BlocksWorldPartialMDPN, BlocksWorldMDPN, LetterManager, Location,
};

impl<'a, const N: usize> BuildFrom<&'a [Location; N], BlocksWorldMDPN<N>>
    for BlocksWorldPartialMDPN<N>
{
    fn build_from(&self, goal: &'a [Location; N]) -> BlocksWorldMDPN<N> {
        BlocksWorldMDPN::new(self.start, *goal, self.epsilon, self.letters)
    }
}

impl<'a, const N: usize> BuildFrom<&'a [char; N], BlocksWorldMDPN<N>>
    for BlocksWorldPartialMDPN<N>
{
    fn build_from(&self, goal: &'a [char; N]) -> BlocksWorldMDPN<N> {
        let lm = LetterManager::new(self.letters);
        let goal = lm.letters_to_goal(*goal);
        BlocksWorldMDPN::new(self.start, goal, self.epsilon, self.letters)
    }
}

impl<'a, const N: usize> BuildFrom<&'a [char; N], StateEnumerableWrapper<BlocksWorldMDPN<N>>>
    for BlocksWorldPartialMDPN<N>
{
    fn build_from(&self, goal: &'a [char; N]) -> StateEnumerableWrapper<BlocksWorldMDPN<N>> {
        let lm = LetterManager::new(self.letters);
        let goal = lm.letters_to_goal(*goal);
        StateEnumerableWrapper::new(BlocksWorldMDPN::new(
            self.start,
            goal,
            self.epsilon,
            self.letters,
        ))
    }
}
