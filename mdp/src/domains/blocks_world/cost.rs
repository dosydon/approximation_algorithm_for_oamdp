use super::BlocksWorldAction::*;
use super::BlocksWorldMDPN;
use crate::mdp_traits::CostFromDCost;
use crate::mdp_traits::{DCost, IsTerminal};

impl<const N: usize> CostFromDCost for BlocksWorldMDPN<N> {}

impl<const N: usize> DCost for BlocksWorldMDPN<N> {
    fn d_cost(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(stt) {
            0.0
        } else {
            match a {
                PickUp(b) => {
                    if let Some(bb) = self.heavy {
                        if (b == &bb) & (st != stt) {
                            3.0
                        } else {
                            1.0
                        }
                    } else {
                        1.0
                    }
                }
                _ => 1.0,
            }
        }
    }
}

// impl<const N: usize> DCostNonTerminal for BlocksWorldMDPN<N> {
//     fn d_cost_non_terminal(&self, _st: &Self::State, _a: &Self::Action, _stt: &Self::State) -> f32 {
//         1.0
//     }
// }
