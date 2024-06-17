use crate::mdp_traits::*;
use core::slice::Iter;

use mdp_derive::*;
use std::collections::{HashSet, VecDeque};

#[derive(
    ActionAvailability,
    ActionEnumerable,
    Cost,
    DCost,
    ExplicitTransition,
    ExplicitTransitionMut,
    PMass32,
    InitialState,
    PreferredSuccessor,
    StatesActions,
    Rsa,
)]
pub struct StateEnumerableWrapper<M: StatesActions> {
    pub mdp: M,
    all_states: Vec<M::State>,
}

impl<M: IsTerminal> IsTerminal for StateEnumerableWrapper<M> {
    fn is_terminal(&self, s: &Self::State) -> bool {
        self.mdp.is_terminal(s)
    }
}

impl<M: GetNextState> GetNextState for StateEnumerableWrapper<M> {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Self::State {
        self.mdp.get_next_state(s, a, rng)
    }
}

impl<M: GetNextState> GetNextStateMutFromImmut for StateEnumerableWrapper<M> {}

impl<
        M: StatesActions
            + InitialState
            + PMass<f32>
            + ActionEnumerable
            + ActionAvailability
            + IsTerminal,
    > StateEnumerableWrapper<M>
{
    pub fn new(mdp: M) -> StateEnumerableWrapper<M> {
        let mut queue = VecDeque::new();
        let mut states = HashSet::new();
        queue.push_back(mdp.initial_state());
        states.insert(mdp.initial_state());

        while let Some(s) = queue.pop_front() {
            if mdp.is_terminal(&s) {
                continue;
            }
            for a in mdp.enumerate_actions() {
                if mdp.action_available(&s, a) {
                    for (ss, _p) in mdp.p_mass(&s, a) {
                        if !states.contains(&ss) {
                            //                             info!("{:?} {:?} -> {:?}", s, a, ss);
                            queue.push_back(ss);
                            states.insert(ss);
                        }
                    }
                }
            }
        }

        StateEnumerableWrapper {
            mdp: mdp,
            all_states: states.into_iter().collect(),
        }
    }
}

impl<M: StatesActions> StateEnumerable for StateEnumerableWrapper<M> {
    fn enumerate_states(&self) -> Iter<Self::State> {
        self.all_states.iter()
    }
    fn num_states(&self) -> usize {
        self.all_states.len()
    }
    fn id_to_state(&self, id: usize) -> &Self::State {
        &(self.all_states[id])
    }
}

impl<M: RenderTo> RenderTo for StateEnumerableWrapper<M> {
    fn render_to(&self, s: &Self::State, path: &str) {
        self.mdp.render_to(s, path)
    }
}

impl<M: StatesActions + DisplayState<M::State>> DisplayState<M::State>
    for StateEnumerableWrapper<M>
{
    fn display(&self, s: &M::State) {
        self.mdp.display(s)
    }
}

impl<M: PMassMut<f32>> PMassMut<f32> for StateEnumerableWrapper<M> {
    type Distribution = M::Distribution;
    fn p_mass_mut(&mut self, s: &M::State, a: &M::Action) -> Self::Distribution {
        self.mdp.p_mass_mut(s, a)
    }
}

impl<M: PMassMut<f64>> PMassMut<f64> for StateEnumerableWrapper<M> {
    type Distribution = M::Distribution;
    fn p_mass_mut(&mut self, s: &M::State, a: &M::Action) -> Self::Distribution {
        self.mdp.p_mass_mut(s, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid_world::*;
    use crate::race_track::*;
    use crate::value_iteration::*;
    use assert_approx_eq::assert_approx_eq;
    use log::debug;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_state_explicating_wrapper_grid_world_num_states() {
        let mdp = GridWorldMDP::default();
        let wrapped = StateEnumerableWrapper::new(GridWorldMDP::default());
        assert_eq!(wrapped.num_states(), mdp.num_states());
    }

    #[test]
    fn test_state_explicating_wrapper_race_track() {
        init();
        let wrapped = StateEnumerableWrapper::new(
            RaceTrackMDP::from_file("data/tracks/small.track").set_p_slip(0.1),
        );
        assert_eq!(238, wrapped.num_states());
        let vt = value_iteration_ssp(&wrapped);
        for s in wrapped.enumerate_states() {
            println!("{:?} {}", s, vt.get_value(&s));
        }
        assert_approx_eq!(7.48011, vt.get_value(&wrapped.initial_state()));
    }

    #[test]
    fn test_state_explicating_wrapper_race_track_medium() {
        init();
        let wrapped = StateEnumerableWrapper::new(
            RaceTrackMDP::from_file("data/tracks/medium.track").set_p_slip(0.1),
        );
        assert_eq!(2218, wrapped.num_states());
        let vt = value_iteration_ssp(&wrapped);
        for s in wrapped.enumerate_states() {
            debug!("{:?} {}", s, vt.get_value(&s));
        }
        assert_approx_eq!(9.202637, vt.get_value(&wrapped.initial_state()));
        //         assert_approx_eq!(7.48011, vt.get_value(&wrapped.initial_state()));
    }
}
