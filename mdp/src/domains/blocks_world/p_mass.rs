use super::change_at::change_at;
use super::location::Location::*;
use super::BlocksWorldAction::{self, *};
use super::{BlocksWorldMDPN, BlocksWorldStateN};
use crate::mdp_traits::{IsTerminal, PMass, PMassMutFrom, PreferredSuccessor};

use super::contains::contains;

impl<const N: usize> BlocksWorldMDPN<N> {
    fn p_mass_non_terminal(
        &self,
        s: &BlocksWorldStateN<N>,
        a: &BlocksWorldAction,
    ) -> Vec<(BlocksWorldStateN<N>, f32)> {
        match a {
            PickUp(b) => {
                if contains(&s.locations, OnHold) || contains(&s.locations, On(*b)) {
                    vec![(BlocksWorldStateN::new(s.locations), 1.0)]
                } else {
                    vec![(
                        BlocksWorldStateN::new(change_at(s.locations, b.id, OnHold)),
                        1.0,
                    )]
                }
            }
            PutDown(b, l) => {
                if s.locations[b.id] == OnHold {
                    if contains(&s.locations, On(*b)) || On(*b) == *l {
                        vec![(BlocksWorldStateN::new(s.locations), 1.0)]
                    } else {
                        if *l == OnTable {
                            vec![(
                                BlocksWorldStateN::new(change_at(s.locations, b.id, OnTable)),
                                1.0,
                            )]
                        } else if contains(&s.locations, *l) {
                            vec![(BlocksWorldStateN::new(s.locations), 1.0)]
                        } else {
                            if self.epsilon > 0.0 {
                                if *l == OnTable {
                                    vec![(
                                        BlocksWorldStateN::new(change_at(s.locations, b.id, *l)),
                                        1.0,
                                    )]
                                } else {
                                    vec![
                                        (
                                            BlocksWorldStateN::new(change_at(
                                                s.locations,
                                                b.id,
                                                *l,
                                            )),
                                            1.0 - self.epsilon,
                                        ),
                                        (
                                            BlocksWorldStateN::new(change_at(
                                                s.locations,
                                                b.id,
                                                OnTable,
                                            )),
                                            self.epsilon,
                                        ),
                                    ]
                                }
                            } else {
                                vec![(
                                    BlocksWorldStateN::new(change_at(s.locations, b.id, *l)),
                                    1.0,
                                )]
                            }
                        }
                    }
                } else {
                    vec![(BlocksWorldStateN::new(s.locations), 1.0)]
                }
            }
        }
    }
}

impl<const N: usize> PMass<f32> for BlocksWorldMDPN<N> {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) {
            vec![(BlocksWorldStateN::new(s.locations), 1.0)]
        } else {
            self.p_mass_non_terminal(s, a)
        }
    }
}

impl<const N: usize> PMassMutFrom<f32> for BlocksWorldMDPN<N> {}

impl<const N: usize> PreferredSuccessor for BlocksWorldMDPN<N> {
    fn preferred_successor(&self, s: &Self::State, a: &Self::Action) -> Self::State {
        if self.is_terminal(s) {
            BlocksWorldStateN::new(s.locations)
        } else {
            match a {
                PickUp(b) => {
                    if contains(&s.locations, OnHold) || contains(&s.locations, On(*b)) {
                        BlocksWorldStateN::new(s.locations)
                    } else {
                        BlocksWorldStateN::new(change_at(s.locations, b.id, OnHold))
                    }
                }
                PutDown(b, l) => {
                    if s.locations[b.id] == OnHold {
                        if contains(&s.locations, On(*b)) || On(*b) == *l {
                            BlocksWorldStateN::new(s.locations)
                        } else {
                            if *l == OnTable {
                                BlocksWorldStateN::new(change_at(s.locations, b.id, OnTable))
                            } else if contains(&s.locations, *l) {
                                BlocksWorldStateN::new(s.locations)
                            } else {
                                BlocksWorldStateN::new(change_at(s.locations, b.id, *l))
                            }
                        }
                    } else {
                        BlocksWorldStateN::new(s.locations)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks_world::Block;
    use crate::blocks_world::BlocksWorldState;

    #[test]
    fn test_p_mass_blocks_world() {
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
        let s = BlocksWorldState::new([OnTable, OnTable, On(Block::new(1)), OnTable]);
        let ss = BlocksWorldState::new([OnHold, OnTable, On(Block::new(1)), OnTable]);
        assert_eq!(
            vec![(ss, 1.0)],
            blocks_world.p_mass(&s, &PickUp(Block::new(0)))
        );
        assert_eq!(
            vec![(s, 1.0)],
            blocks_world.p_mass(&s, &PickUp(Block::new(1)))
        );
        assert_eq!(
            vec![(ss, 1.0)],
            blocks_world.p_mass(&ss, &PickUp(Block::new(0)))
        );
        assert_eq!(
            vec![(ss, 1.0)],
            blocks_world.p_mass(&ss, &PickUp(Block::new(1)))
        );

        let putdown =
            BlocksWorldState::new([On(Block::new(2)), OnTable, On(Block::new(1)), OnTable]);
        assert_eq!(
            vec![(putdown, 1.0)],
            blocks_world.p_mass(&ss, &PutDown(Block::new(0), On(Block::new(2))))
        );
        assert_eq!(
            vec![(ss, 1.0)],
            blocks_world.p_mass(&ss, &PutDown(Block::new(0), On(Block::new(1))))
        );
        assert_eq!(
            vec![(ss, 1.0)],
            blocks_world.p_mass(&ss, &PutDown(Block::new(3), On(Block::new(2))))
        );
        assert_eq!(
            vec![(ss, 1.0)],
            blocks_world.p_mass(&ss, &PutDown(Block::new(0), On(Block::new(0))))
        );
    }

    #[test]
    fn test_p_mass_blocks_world6() {
        let b0 = Block::new(0);
        let b1 = Block::new(1);
        let b2 = Block::new(2);
        let b3 = Block::new(3);
        let b4 = Block::new(4);
        let blocks_world = BlocksWorldMDPN::new(
            [OnTable, OnTable, OnTable, OnTable, OnTable, OnTable],
            [OnTable, On(b0), On(b1), On(b2), On(b3), On(b4)],
            0.0,
            ['A', 'B', 'C', 'M', 'S', 'R'],
        );
        let s = BlocksWorldStateN::new([OnTable, OnTable, OnTable, OnTable, OnTable, OnTable]);
        let ss = BlocksWorldStateN::new([OnTable, OnHold, OnTable, OnTable, OnTable, OnTable]);
        assert_eq!(vec![(ss, 1.0)], blocks_world.p_mass(&s, &PickUp(b1)));
        /* assert_eq!(vec![(s, 1.0)], blocks_world.p_mass(&s, &PickUp(Block::new(1))));                       */
        /* assert_eq!(vec![(ss, 1.0)], blocks_world.p_mass(&ss, &PickUp(Block::new(0))));                     */
        /* assert_eq!(vec![(ss, 1.0)], blocks_world.p_mass(&ss, &PickUp(Block::new(1))));                     */

        /* let putdown = BlocksWorldState::new([On(Block::new(2)), OnTable, On(Block::new(1)), OnTable]);     */
        /* assert_eq!(                                                                                        */
        /*     vec![(putdown, 1.0)],                                                                          */
        /*     blocks_world.p_mass(&ss, &PutDown(Block::new(0), On(Block::new(2))))                           */
        /* );                                                                                                 */
        /* assert_eq!(vec![(ss, 1.0)], blocks_world.p_mass(&ss, &PutDown(Block::new(0), On(Block::new(1))))); */
        /* assert_eq!(vec![(ss, 1.0)], blocks_world.p_mass(&ss, &PutDown(Block::new(3), On(Block::new(2))))); */
        /* assert_eq!(vec![(ss, 1.0)], blocks_world.p_mass(&ss, &PutDown(Block::new(0), On(Block::new(0))))); */
    }

    #[test]
    fn test_preferred_successor_blocks_world() {
        let blocks_world = BlocksWorldMDPN::new(
            [OnTable, OnTable, On(Block::new(1)), OnTable],
            [
                On(Block::new(3)),
                On(Block::new(0)),
                OnTable,
                On(Block::new(2)),
            ],
            0.5,
            ['A', 'M', 'S', 'R'],
        );
        let s = BlocksWorldState::new([OnTable, OnTable, On(Block::new(1)), OnTable]);
        let ss = BlocksWorldState::new([OnHold, OnTable, On(Block::new(1)), OnTable]);
        assert_eq!(
            ss,
            blocks_world.preferred_successor(&s, &PickUp(Block::new(0)))
        );
        assert_eq!(
            s,
            blocks_world.preferred_successor(&s, &PickUp(Block::new(1)))
        );
        assert_eq!(
            ss,
            blocks_world.preferred_successor(&ss, &PickUp(Block::new(0)))
        );
        assert_eq!(
            ss,
            blocks_world.preferred_successor(&ss, &PickUp(Block::new(1)))
        );

        let putdown =
            BlocksWorldState::new([On(Block::new(2)), OnTable, On(Block::new(1)), OnTable]);
        assert_eq!(
            putdown,
            blocks_world.preferred_successor(&ss, &PutDown(Block::new(0), On(Block::new(2))))
        );
        assert_eq!(
            ss,
            blocks_world.preferred_successor(&ss, &PutDown(Block::new(0), On(Block::new(1))))
        );
        assert_eq!(
            ss,
            blocks_world.preferred_successor(&ss, &PutDown(Block::new(3), On(Block::new(2))))
        );
        assert_eq!(
            ss,
            blocks_world.preferred_successor(&ss, &PutDown(Block::new(0), On(Block::new(0))))
        );
    }
}
