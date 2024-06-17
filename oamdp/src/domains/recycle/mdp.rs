use std::slice::Iter;

use mdp::mdp_traits::*;

use strum::IntoEnumIterator;

use super::{action::RecycleAction, location::Location, state::RecycleState};
pub struct RecycleMDP<const K: usize> {
    pub(crate) target: [Location; 3],
    initial_locs: [Location; K],
    pub(crate) kinds: [usize; K],
    all_actions: Vec<RecycleAction>,
    all_states: Vec<RecycleState<K>>,
    success_prob: f32,
}

fn cartesian_product(current: Vec<Location>, depth: usize, n: usize) -> Vec<Vec<Location>> {
    if depth == n {
        return vec![current];
    }
    let mut result = Vec::new();
    for loc in Location::iter() {
        let mut next_current = current.clone();
        next_current.push(loc);
        result.extend(cartesian_product(next_current, depth + 1, n));
    }
    result
}

fn enumerate_states<const K: usize>() -> Vec<RecycleState<K>> {
    cartesian_product(Vec::new(), 0, K)
        .into_iter()
        .map(|locs| {
            let mut arr = [Location::Trash; K];
            for (i, loc) in locs.into_iter().enumerate() {
                arr[i] = loc;
            }
            RecycleState::new(arr)
        })
        .collect()
}

fn enumerate_actions(n: usize) -> Vec<RecycleAction> {
    let mut result = Vec::new();
    for i in 0..n {
        for loc in [Location::Compost, Location::Recycle, Location::Trash].iter() {
            result.push(RecycleAction::Moveto(i, *loc));
        }
        result.push(RecycleAction::PickUp(i));
    }
    result
}

impl<const K: usize> RecycleMDP<K> {
    pub fn new(
        target: [Location; 3],
        initial_locs: [Location; K],
        kinds: [usize; K],
        success_prob: f32,
    ) -> Self {
        let actions = enumerate_actions(K);
        let all_states = enumerate_states();
        //         let all_states = iproduct!(Location::iter(), Location::iter(), Location::iter())
        //             .map(|(a, b, c)| RecycleState::new([a, b, c]))
        //             .collect();

        RecycleMDP {
            target,
            initial_locs: initial_locs,
            kinds: kinds,
            all_actions: actions,
            all_states: all_states,
            success_prob: success_prob,
        }
    }
}
//
// impl RecycleMDP<5> {
//     pub fn new5(
//         target: [Location; 3],
//         initial_locs: [Location; 5],
//         kinds: [usize; 5],
//         success_prob: f32,
//     ) -> Self {
//         let actions = vec![
//             RecycleAction::Moveto(0, Location::Compost),
//             RecycleAction::Moveto(0, Location::Recycle),
//             RecycleAction::Moveto(0, Location::Trash),
//             RecycleAction::Moveto(1, Location::Compost),
//             RecycleAction::Moveto(1, Location::Recycle),
//             RecycleAction::Moveto(1, Location::Trash),
//             RecycleAction::Moveto(2, Location::Compost),
//             RecycleAction::Moveto(2, Location::Recycle),
//             RecycleAction::Moveto(2, Location::Trash),
//             RecycleAction::Moveto(3, Location::Compost),
//             RecycleAction::Moveto(3, Location::Recycle),
//             RecycleAction::Moveto(3, Location::Trash),
//             RecycleAction::Moveto(4, Location::Compost),
//             RecycleAction::Moveto(4, Location::Recycle),
//             RecycleAction::Moveto(4, Location::Trash),
//             RecycleAction::PickUp(0),
//             RecycleAction::PickUp(1),
//             RecycleAction::PickUp(2),
//             RecycleAction::PickUp(3),
//             RecycleAction::PickUp(4),
//         ];
//         let all_states = iproduct!(
//             Location::iter(),
//             Location::iter(),
//             Location::iter(),
//             Location::iter(),
//             Location::iter()
//         )
//         .map(|(a, b, c, d, e)| RecycleState::new([a, b, c, d, e]))
//         .collect();
//
//         RecycleMDP {
//             target,
//             initial_locs: initial_locs,
//             kinds: kinds,
//             all_actions: actions,
//             all_states: all_states,
//             success_prob: success_prob,
//         }
//     }
// }

impl<const K: usize> StatesActions for RecycleMDP<K> {
    type State = RecycleState<K>;
    type Action = RecycleAction;
}

impl<const K: usize> IsTerminal for RecycleMDP<K> {
    fn is_terminal(&self, s: &Self::State) -> bool {
        for i in 0..K {
            if s.locs[i] != self.target[self.kinds[i]] {
                return false;
            }
        }
        true
    }
}

// impl<const K: usize> DisplayState<RecycleState<K>> for RecycleMDP<K> {
//     fn display(&self, s: &RecycleState<K>) {
//         println!("{:?}", s);
//     }
// }

impl<const K: usize> StateEnumerable for RecycleMDP<K> {
    fn enumerate_states(&self) -> Iter<Self::State> {
        self.all_states.iter()
    }

    fn num_states(&self) -> usize {
        self.all_states.len()
    }

    fn id_to_state(&self, id: usize) -> &Self::State {
        &self.all_states[id]
    }
}

impl<const K: usize> ActionEnumerable for RecycleMDP<K> {
    fn enumerate_actions(&self) -> Iter<Self::Action> {
        self.all_actions.iter()
    }

    fn num_actions(&self) -> usize {
        self.all_actions.len()
    }

    fn id_to_action(&self, id: usize) -> &Self::Action {
        &self.all_actions[id]
    }
}

impl<const K: usize> ExplicitTransition for RecycleMDP<K> {}

fn is_hand_full<const K: usize>(s: &RecycleState<K>) -> bool {
    s.locs.iter().any(|l| *l == Location::InHand)
}

impl<const K: usize> PMass<f32> for RecycleMDP<K> {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Self::Distribution {
        match a {
            RecycleAction::Moveto(i, loc) => {
                if s.locs[*i] == Location::InHand {
                    let mut result = vec![];
                    for dest in [Location::Compost, Location::Recycle, Location::Trash].iter() {
                        if dest != loc {
                            if self.success_prob < 1.0 {
                                result.push((s.change(*i, *dest), 0.5 * (1.0 - self.success_prob)));
                            }
                        } else {
                            result.push((s.change(*i, *dest), self.success_prob));
                        }
                    }
                    result
                } else {
                    vec![(s.clone(), 1.0)]
                }
            }
            RecycleAction::PickUp(i) => {
                if is_hand_full(s) {
                    vec![(s.clone(), 1.0)]
                } else {
                    vec![(s.change(*i, Location::InHand), 1.0)]
                }
            }
        }
    }
}

impl<const K: usize> GetNextStateFromPMass for RecycleMDP<K> {}
impl<const K: usize> GetNextStateMutFromImmut for RecycleMDP<K> {}

impl<const K: usize> Cost for RecycleMDP<K> {
    fn cost(&self, s: &Self::State, _a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            1.0
        }
    }
}

impl<const K: usize> DCost for RecycleMDP<K> {
    fn d_cost(&self, s: &Self::State, a: &Self::Action, _ss: &Self::State) -> f32 {
        self.cost(s, a)
    }
}

impl<const K: usize> ActionAvailability for RecycleMDP<K> {
    fn action_available(&self, _s: &Self::State, _a: &Self::Action) -> bool {
        true
    }
}

impl<const K: usize> InitialState for RecycleMDP<K> {
    fn initial_state(&self) -> Self::State {
        RecycleState::new(self.initial_locs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mdp::{
        episode_runner::CostEpisodeIterator, policy::tabular_policy::TabularPolicy,
        value_iteration::value_iteration_ssp,
    };
    use rand::thread_rng;

    #[test]
    fn test_value_iteration() {
        let mdp = RecycleMDP::new(
            [Location::Compost, Location::Recycle, Location::Trash],
            [Location::Trash; 5],
            [0, 1, 2, 0, 1],
            1.0,
        );
        for s in mdp.enumerate_states() {
            println!("{:?}", s);
        }
        let vt = value_iteration_ssp(&mdp);
        let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        let mut rng = thread_rng();
        for (s, a, _, c) in CostEpisodeIterator::from_initial_state(&mdp, &tabular_policy, &mut rng)
        {
            println!("{:?} {:?} {:?}", s, a, c);
        }
    }
}
