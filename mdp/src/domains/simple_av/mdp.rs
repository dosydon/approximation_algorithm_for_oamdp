use crate::mdp_traits::{
    ActionAvailability, ActionEnumerable, BuildFrom, Cost, ExplicitTransition,
    GetNextStateFromPMass, InitialState, IsTerminal, PMass, StateEnumerable, StatesActions,
};
use crate::simple_av::action::SimpleAVAction::*;
use crate::simple_av::parameter::SimpleAVParameter::*;
use crate::simple_av::{SimpleAVAction, SimpleAVParameter, SimpleAVPartialMDP, SimpleAVState};
use core::slice::Iter;
use itertools::iproduct;

pub struct SimpleAVMDP {
    pub(in crate::domains::simple_av) maxy: usize,
    mindy: i32,
    maxdy: i32,
    all_states: Vec<SimpleAVState>,
    all_actions: Vec<SimpleAVAction>,
    start: SimpleAVState,
    parameter: SimpleAVParameter,
}

impl SimpleAVMDP {
    pub fn new(
        miny: usize,
        maxy: usize,
        mindy: i32,
        maxdy: i32,
        start: SimpleAVState,
        parameter: SimpleAVParameter,
    ) -> SimpleAVMDP {
        let all_states = iproduct!((miny..=maxy), (mindy..=maxdy))
            .map(|(y, dy)| SimpleAVState::new(y, dy))
            .collect::<Vec<_>>();

        SimpleAVMDP {
            maxy,
            mindy,
            maxdy,
            all_states: all_states,
            all_actions: vec![Accelerate, Decelerate, Keep],
            start: start,
            parameter: parameter,
        }
    }

    pub fn next(&self, current: &SimpleAVState, a: &SimpleAVAction) -> SimpleAVState {
        current.next(a, self.maxy, self.mindy, self.maxdy)
    }
}

impl GetNextStateFromPMass for SimpleAVMDP {}

impl ActionAvailability for SimpleAVMDP {}

impl StatesActions for SimpleAVMDP {
    type State = SimpleAVState;
    type Action = SimpleAVAction;
}

impl IsTerminal for SimpleAVMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        match self.parameter {
            NonYield(dest, lb, ub) => s.y >= dest && s.dy >= lb && s.dy <= ub,
            YouHaveLightOff(dest, lb, ub) => s.y >= dest && s.dy >= lb && s.dy <= ub,
            Stopping(lb, ub) => s.y >= lb && s.y <= ub && s.dy == 0,
        }
    }
}

impl ActionEnumerable for SimpleAVMDP {
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

impl StateEnumerable for SimpleAVMDP {
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

impl InitialState for SimpleAVMDP {
    fn initial_state(&self) -> Self::State {
        self.start
    }
}

impl PMass<f32> for SimpleAVMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) {
            vec![(*s, 1.0)]
        } else {
            vec![(s.next(a, self.maxy, self.mindy, self.maxdy), 1.0)]
        }
    }
}

// impl PMassMut<f32> for SimpleAVMDP {
//     type Distribution = Vec<(Self::State, f32)>;
//     fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
//         self.p_mass(s, a)
//     }
// }

impl ExplicitTransition for SimpleAVMDP {}

impl Cost for SimpleAVMDP {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            match a {
                Accelerate => 4.0,
                Decelerate => 4.0,
                Keep => 1.0,
                _ => panic!("not possible"),
            }
        }
    }
}

impl BuildFrom<SimpleAVParameter, SimpleAVMDP> for SimpleAVPartialMDP {
    fn build_from(&self, parameter: SimpleAVParameter) -> SimpleAVMDP {
        SimpleAVMDP::new(
            self.miny, self.maxy, self.mindy, self.maxdy, self.start, parameter,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p_mass() {
        let problem = SimpleAVMDP::new(0, 10, -3, 3, SimpleAVState::new(0, 2), NonYield(8, 2, 3));
        assert_eq!(
            vec![(SimpleAVState::new(0, 1), 1.0)],
            problem.p_mass(&SimpleAVState::new(0, 0), &Accelerate)
        );
    }
}
