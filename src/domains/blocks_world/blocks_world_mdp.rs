use crate::blocks_world::blocks_world_action::BlocksWorldAction::*;
use crate::blocks_world::location::Location::*;
use crate::blocks_world::{Block, BlocksWorldAction, BlocksWorldStateN, Location};
use crate::mdp_traits::*;
use core::slice::Iter;

#[derive(PartialEq, Debug, Clone)]
pub struct BlocksWorldMDPN<const NB: usize> {
    pub start: [Location; NB],
    pub goal: [Location; NB],
    pub(in crate::domains::blocks_world) epsilon: f32,
    pub(in crate::domains::blocks_world) all_actions: Vec<BlocksWorldAction>,
    pub(in crate::domains::blocks_world) heavy: Option<Block>,
    pub letters: [char; NB],
}

impl<const NB: usize> BlocksWorldMDPN<NB> {
    pub fn new(
        start: [Location; NB],
        goal: [Location; NB],
        epsilon: f32,
        letters: [char; NB],
    ) -> BlocksWorldMDPN<NB> {
        let mut all_actions = vec![];
        for b_id in 0..NB {
            all_actions.push(PickUp(Block::new(b_id)));
            all_actions.push(PutDown(Block::new(b_id), OnTable));
            for l_id in 0..NB {
                all_actions.push(PutDown(Block::new(b_id), On(Block::new(l_id))));
            }
        }
        BlocksWorldMDPN {
            start: start,
            goal: goal,
            epsilon: epsilon,
            all_actions: all_actions,
            heavy: None,
            letters: letters,
        }
    }
}

impl<const N: usize> ActionAvailability for BlocksWorldMDPN<N> {}

impl<const N: usize> StatesActions for BlocksWorldMDPN<N> {
    type State = BlocksWorldStateN<N>;
    type Action = BlocksWorldAction;
}

impl<const N: usize> IsTerminal for BlocksWorldMDPN<N> {
    fn is_terminal(&self, s: &Self::State) -> bool {
        s.locations == self.goal
    }
}

impl<const N: usize> InitialState for BlocksWorldMDPN<N> {
    fn initial_state(&self) -> BlocksWorldStateN<N> {
        BlocksWorldStateN::new(self.start)
    }
}

impl<const N: usize> ActionEnumerable for BlocksWorldMDPN<N> {
    fn enumerate_actions(&self) -> Iter<Self::Action> {
        self.all_actions.iter()
    }

    fn num_actions(&self) -> usize {
        self.all_actions.len()
    }

    fn id_to_action(&self, id: usize) -> &Self::Action {
        &(self.all_actions[id])
    }
}

impl<const N: usize> RenderTo for BlocksWorldMDPN<N> {
    fn render_to(&self, _s: &Self::State, _path: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        blocks_world::{BlocksWorldState, LetterManager},
        state_enumerable_wrapper::StateEnumerableWrapper,
    };

    #[test]
    fn test_initial_state_blocks_world() {
        let lm = LetterManager::new(['A', 'M', 'S', 'R']);
        let blocks_world = BlocksWorldMDPN::new(
            lm.str_to_locations("A SM R"),
            lm.str_to_locations("MARS"),
            0.0,
            lm.letters,
        );
        assert_eq!(
            BlocksWorldState::new([OnTable, OnTable, On(Block::new(1)), OnTable]),
            blocks_world.initial_state()
        );
    }

    #[test]
    fn test_cost_blocks_world() {
        let lm = LetterManager::new(['A', 'M', 'S', 'R']);
        let mut blocks_world = BlocksWorldMDPN::new(
            lm.str_to_locations("A SM R"),
            lm.str_to_locations("MARS"),
            0.0,
            lm.letters,
        );
        blocks_world.heavy = Some(Block::new(0));
        let s = BlocksWorldState::new(lm.str_to_locations("A SM R"));
        assert_eq!(blocks_world.cost(&s, &PickUp(Block::new(1))), 1.0);
        assert_eq!(blocks_world.cost(&s, &PickUp(Block::new(0))), 3.0);
    }

    #[test]
    fn test_p_blocks_world() {
        let lm = LetterManager::new(['A', 'M', 'S', 'R']);
        let blocks_world = BlocksWorldMDPN::new(
            lm.str_to_locations("A SM R"),
            lm.str_to_locations("MARS"),
            0.1,
            lm.letters,
        );
        let s = BlocksWorldState::new([OnTable, OnTable, On(Block::new(1)), OnTable]);
        let ss = BlocksWorldState::new([OnHold, OnTable, On(Block::new(1)), OnTable]);
        assert_eq!(1.0, blocks_world.p(&s, &PickUp(Block::new(0)), &ss));
        assert_eq!(1.0, blocks_world.p(&s, &PickUp(Block::new(1)), &s));
        assert_eq!(1.0, blocks_world.p(&ss, &PickUp(Block::new(0)), &ss));
        assert_eq!(1.0, blocks_world.p(&ss, &PickUp(Block::new(2)), &ss));

        let putdown =
            BlocksWorldState::new([On(Block::new(2)), OnTable, On(Block::new(1)), OnTable]);
        assert_eq!(
            0.9,
            blocks_world.p(&ss, &PutDown(Block::new(0), On(Block::new(2))), &putdown)
        );
        assert_eq!(
            1.0,
            blocks_world.p(&ss, &PutDown(Block::new(1), On(Block::new(2))), &ss)
        );
        assert_eq!(
            1.0,
            blocks_world.p(&ss, &PutDown(Block::new(0), On(Block::new(0))), &ss)
        );
    }

    #[test]
    fn test_num_states() {
        let blocks_world = BlocksWorldMDPN::new(
            [OnTable, OnTable, On(Block::new(1)), OnTable],
            [
                On(Block::new(3)),
                On(Block::new(0)),
                OnTable,
                On(Block::new(2)),
            ],
            0.0,
            ['A', 'M', 'S', 'R'],
        );
        let wrapper = StateEnumerableWrapper::new(blocks_world);
        assert_eq!(125, wrapper.num_states());
    }
}
