use rand::rngs::ThreadRng;
use rand::Rng;

use crate::mdp_traits::{GetNextState, GetNextStateMutFromImmut};

use super::change_at::change_at;
use super::location::Location::*;
use super::{contains::contains, BlocksWorldMDPN};
use super::{BlocksWorldAction::*, BlocksWorldStateN};

impl<const N: usize> GetNextState for BlocksWorldMDPN<N> {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut ThreadRng,
    ) -> Self::State {
        match a {
            PickUp(b) => {
                if contains(&s.locations, OnHold) || contains(&s.locations, On(*b)) {
                    BlocksWorldStateN::new(s.locations)
                } else {
                    BlocksWorldStateN::new(change_at(s.locations, b.id, OnHold))
                }
            }
            PutDown(b, l) => {
                if s.locations[b.id] != OnHold {
                    BlocksWorldStateN::new(s.locations)
                } else {
                    if contains(&s.locations, On(*b)) || On(*b) == *l {
                        BlocksWorldStateN::new(s.locations)
                    } else {
                        if *l == OnTable {
                            BlocksWorldStateN::new(change_at(s.locations, b.id, OnTable))
                        } else if contains(&s.locations, *l) {
                            BlocksWorldStateN::new(s.locations)
                        } else {
                            let r = rng.gen_range(0.0, 1.0);
                            if r < self.epsilon {
                                BlocksWorldStateN::new(change_at(s.locations, b.id, OnTable))
                            } else {
                                BlocksWorldStateN::new(change_at(s.locations, b.id, *l))
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<const N: usize> GetNextStateMutFromImmut for BlocksWorldMDPN<N> {}
