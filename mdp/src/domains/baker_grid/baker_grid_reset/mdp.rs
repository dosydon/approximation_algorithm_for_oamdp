use crate::{
    baker_grid::{mdp::add_outcome, BakerGridAction, BakerGridMDP, BakerGridState},
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, ExplicitTransition,
        GetNextStateFromPMass, GetNextStateMutFromImmut, InitialState, IsTerminal, PMass,
        StateEnumerable, StatesActions,
    },
};

#[derive(PartialEq, Debug, Clone)]
pub struct BakerGridResetMDP {
    pub mdp: BakerGridMDP,
    reset_prob: f32,
    reset_states: Vec<BakerGridState>,
}

impl BakerGridResetMDP {
    pub fn new(
        mdp: BakerGridMDP,
        reset_prob: f32,
        reset_states: Vec<BakerGridState>,
    ) -> BakerGridResetMDP {
        BakerGridResetMDP {
            mdp: mdp,
            reset_prob: reset_prob,
            reset_states: reset_states,
        }
    }

    pub fn width(&self) -> usize {
        self.mdp.width()
    }

    pub fn height(&self) -> usize {
        self.mdp.height()
    }
}

impl StatesActions for BakerGridResetMDP {
    type State = BakerGridState;
    type Action = BakerGridAction;
}

impl IsTerminal for BakerGridResetMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        self.mdp.is_terminal(s)
    }
}

impl StateEnumerable for BakerGridResetMDP {
    fn enumerate_states(&self) -> std::slice::Iter<Self::State> {
        self.mdp.enumerate_states()
    }

    fn num_states(&self) -> usize {
        self.mdp.num_states()
    }

    fn id_to_state(&self, id: usize) -> &Self::State {
        self.mdp.id_to_state(id)
    }
}

impl ActionEnumerable for BakerGridResetMDP {
    fn enumerate_actions(&self) -> std::slice::Iter<Self::Action> {
        self.mdp.enumerate_actions()
    }

    fn num_actions(&self) -> usize {
        self.mdp.num_actions()
    }

    fn id_to_action(&self, id: usize) -> &Self::Action {
        self.mdp.id_to_action(id)
    }
}

impl ActionAvailability for BakerGridResetMDP {
    fn action_available(&self, s: &Self::State, a: &Self::Action) -> bool {
        self.mdp.action_available(s, a)
    }
}

impl InitialState for BakerGridResetMDP {
    fn initial_state(&self) -> Self::State {
        self.mdp.initial_state()
    }
}

impl Cost for BakerGridResetMDP {
    fn cost(&self, s: &Self::State, _a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            1.0
        }
    }
}

impl DCost for BakerGridResetMDP {
    fn d_cost(&self, s: &Self::State, _a: &Self::Action, _ss: &Self::State) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            1.0
        }
    }
}

impl PMass<f32> for BakerGridResetMDP {
    type Distribution = Vec<(BakerGridState, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Self::Distribution {
        if self.is_terminal(s) {
            vec![(*s, 1.0)]
        } else {
            if self.reset_states.len() > 0 {
                if self.reset_states.contains(s) {
                    let mut result = self
                        .mdp
                        .p_mass(s, a)
                        .into_iter()
                        .map(|(ss, p)| (ss, p * (1.0 - self.reset_prob)))
                        .collect();
                    add_outcome(&mut result, self.initial_state(), self.reset_prob);
                    result
                } else {
                    self.mdp.p_mass(s, a)
                }
            } else {
                let mut result = self
                    .mdp
                    .p_mass(s, a)
                    .into_iter()
                    .map(|(ss, p)| (ss, p * (1.0 - self.reset_prob)))
                    .collect();
                add_outcome(&mut result, self.initial_state(), self.reset_prob);
                result
            }
        }
    }
}

impl GetNextStateFromPMass for BakerGridResetMDP {}
impl GetNextStateMutFromImmut for BakerGridResetMDP {}
impl ExplicitTransition for BakerGridResetMDP {}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::{
        episode_runner::EpisodeRunner, mdp_traits::GetProbability,
        policy::tabular_policy::TabularPolicy, value_iteration::value_iteration_ssp,
    };

    use super::*;
    #[test]
    fn test_baker_grid_reset_p_mass() {
        let mdp = BakerGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            BakerGridState::new(4, 4),
        )
        .set_prob_veering(0.1);

        assert_eq!(
            vec![
                (BakerGridState::new(2, 2), 0.9),
                (BakerGridState::new(1, 2), 0.05),
                (BakerGridState::new(2, 1), 0.05)
            ],
            mdp.p_mass(&BakerGridState::new(2, 1), &BakerGridAction::East)
        );

        let mdp = BakerGridResetMDP::new(mdp, 0.1, vec![]);
        assert_eq!(
            0.1,
            mdp.p_mass(&BakerGridState::new(2, 1), &BakerGridAction::East)
                .get_probability(&BakerGridState::new(4, 0))
        );
        //         let vt = value_iteration_ssp(&mdp);
        //         println!("{}", vt.get_value(&mdp.initial_state()))
    }

    #[test]
    fn test_baker_grid_reset_value_iteration() {
        let mdp = BakerGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            BakerGridState::new(4, 4),
        )
        .set_prob_veering(0.1);

        let mdp = BakerGridResetMDP::new(mdp, 0.1, vec![]);
        assert_eq!(
            0.1,
            mdp.p_mass(&BakerGridState::new(2, 1), &BakerGridAction::East)
                .get_probability(&BakerGridState::new(4, 0))
        );
        let vt = value_iteration_ssp(&mdp);
        println!("{}", vt.get_value(&mdp.initial_state()));

        let mut rng = thread_rng();
        let policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        let mut runner = EpisodeRunner::new(&mdp, &policy, mdp.initial_state());
        for (s, a, _ss, c) in runner.into_iter_with(&mut rng) {
            println!("{:?} {:?} {:?}", s, a, c)
        }
    }
}
