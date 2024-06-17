use crate::common::av1d_map::AV1dRange;
use crate::mdp_traits::{
    ActionAvailability, ActionEnumerable, BuildFrom, Cost, DCost, ExplicitTransition,
    GetNextStateFromPMass, InitialState, IsTerminal, PMass, PreferredSuccessor, StateEnumerable,
    StatesActions,
};
use crate::simple_av::vehicle_configuration::VehicleConfiguration;
use crate::simple_av::SimpleAVAction::*;
use crate::simple_av::SimpleAVParameter::*;
use crate::simple_av::{
    SimpleAVAction, SimpleAVParameter, SimpleAVPartialMDP, SimpleAVVehicleInFrontState,
};
use core::slice::Iter;
use itertools::iproduct;

pub struct SimpleAVVehicleInFrontPartialMDP {
    pub(in crate::domains::simple_av) range: AV1dRange,
    pub(in crate::domains::simple_av) start: SimpleAVVehicleInFrontState,
}

impl SimpleAVVehicleInFrontPartialMDP {
    pub fn new(
        miny: usize,
        maxy: usize,
        mindy: i32,
        maxdy: i32,
        start: SimpleAVVehicleInFrontState,
    ) -> SimpleAVVehicleInFrontPartialMDP {
        SimpleAVVehicleInFrontPartialMDP {
            range: AV1dRange::new(miny, maxy, mindy, maxdy),
            start: start,
        }
    }
}

pub struct SimpleAVVehicleInFrontMDP {
    pub range: AV1dRange,
    pub(in crate::domains::simple_av) all_states: Vec<SimpleAVVehicleInFrontState>,
    pub(in crate::domains::simple_av) all_actions: Vec<SimpleAVAction>,
    pub(in crate::domains::simple_av) start: SimpleAVVehicleInFrontState,
    pub parameter: SimpleAVParameter,
}

impl SimpleAVVehicleInFrontMDP {
    pub fn new(
        miny: usize,
        maxy: usize,
        mindy: i32,
        maxdy: i32,
        start: SimpleAVVehicleInFrontState,
        parameter: SimpleAVParameter,
    ) -> SimpleAVVehicleInFrontMDP {
        let all_states = iproduct!(
            (miny..=maxy),
            (mindy..=maxdy),
            (miny..=maxy),
            (mindy..=maxdy)
        )
        .map(|(y, dy, yy, dyy)| {
            (
                VehicleConfiguration::new(y, dy),
                VehicleConfiguration::new(yy, dyy),
            )
        })
        .map(|(ego, infront)| SimpleAVVehicleInFrontState::new(ego, infront))
        .collect::<Vec<_>>();

        SimpleAVVehicleInFrontMDP {
            range: AV1dRange::new(0, maxy, mindy, maxdy),
            all_states: all_states,
            all_actions: vec![Keep, Accelerate, Decelerate],
            start: start,
            parameter: parameter,
        }
    }

    fn d_cost(
        &self,
        st: &SimpleAVVehicleInFrontState,
        a: &SimpleAVAction,
        stt: &SimpleAVVehicleInFrontState,
    ) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else if stt.ego_vehicle.y + 1 >= stt.vehicle_in_front.y {
            200.0
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

impl GetNextStateFromPMass for SimpleAVVehicleInFrontMDP {}

impl ActionAvailability for SimpleAVVehicleInFrontMDP {}

impl StatesActions for SimpleAVVehicleInFrontMDP {
    type State = SimpleAVVehicleInFrontState;
    type Action = SimpleAVAction;
}

impl IsTerminal for SimpleAVVehicleInFrontMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        let ego_vehicle = s.ego_vehicle;
        match self.parameter {
            NonYield(dest, lb, ub) => {
                ego_vehicle.y >= dest && ego_vehicle.dy >= lb && ego_vehicle.dy <= ub
            }
            YouHaveLightOff(dest, lb, ub) => {
                ego_vehicle.y >= dest && ego_vehicle.dy >= lb && ego_vehicle.dy <= ub
            }
            Stopping(lb, ub) => ego_vehicle.y >= lb && ego_vehicle.y <= ub && ego_vehicle.dy == 0,
        }
    }
}

impl ActionEnumerable for SimpleAVVehicleInFrontMDP {
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

impl StateEnumerable for SimpleAVVehicleInFrontMDP {
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

impl InitialState for SimpleAVVehicleInFrontMDP {
    fn initial_state(&self) -> Self::State {
        self.start
    }
}

impl PMass<f32> for SimpleAVVehicleInFrontMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) {
            vec![(*s, 1.0)]
        } else {
            vec![
                (
                    SimpleAVVehicleInFrontState::new(
                        s.ego_vehicle
                            .next(a, self.range.maxy, self.range.mindy, self.range.maxdy),
                        s.vehicle_in_front.next(
                            &Keep,
                            self.range.maxy,
                            self.range.mindy,
                            self.range.maxdy,
                        ),
                    ),
                    0.90,
                ),
                (
                    SimpleAVVehicleInFrontState::new(
                        s.ego_vehicle
                            .next(a, self.range.maxy, self.range.mindy, self.range.maxdy),
                        s.vehicle_in_front.next(
                            &Decelerate,
                            self.range.maxy,
                            self.range.mindy,
                            self.range.maxdy,
                        ),
                    ),
                    0.05,
                ),
                (
                    SimpleAVVehicleInFrontState::new(
                        s.ego_vehicle
                            .next(a, self.range.maxy, self.range.mindy, self.range.maxdy),
                        s.vehicle_in_front.next(
                            &Accelerate,
                            self.range.maxy,
                            self.range.mindy,
                            self.range.maxdy,
                        ),
                    ),
                    0.05,
                ),
            ]
        }
    }
}

// impl GenerativeMDPMut for SimpleAVVehicleInFrontMDP {
//     fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
//         self.p_mass(s, a)
//     }
// }

impl ExplicitTransition for SimpleAVVehicleInFrontMDP {}

impl Cost for SimpleAVVehicleInFrontMDP {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        self.p_mass(s, a)
            .into_iter()
            .map(|(stt, prob)| prob * self.d_cost(s, a, &stt))
            .sum()
    }
}

impl BuildFrom<SimpleAVParameter, SimpleAVVehicleInFrontMDP> for SimpleAVPartialMDP {
    fn build_from(&self, parameter: SimpleAVParameter) -> SimpleAVVehicleInFrontMDP {
        let new_start = SimpleAVVehicleInFrontState::new(
            self.start,
            VehicleConfiguration::new(self.start.y + 12, self.start.dy),
        );
        SimpleAVVehicleInFrontMDP::new(
            self.miny, self.maxy, self.mindy, self.maxdy, new_start, parameter,
        )
    }
}

impl BuildFrom<SimpleAVParameter, SimpleAVVehicleInFrontMDP> for SimpleAVVehicleInFrontPartialMDP {
    fn build_from(&self, parameter: SimpleAVParameter) -> SimpleAVVehicleInFrontMDP {
        SimpleAVVehicleInFrontMDP::new(
            self.range.miny,
            self.range.maxy,
            self.range.mindy,
            self.range.maxdy,
            self.start,
            parameter,
        )
    }
}

impl DCost for SimpleAVVehicleInFrontMDP {
    fn d_cost(
        &self,
        st: &SimpleAVVehicleInFrontState,
        a: &SimpleAVAction,
        stt: &SimpleAVVehicleInFrontState,
    ) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else if stt.ego_vehicle.y + 1 >= stt.vehicle_in_front.y {
            200.0
        } else {
            match a {
                Accelerate => 4.0,
                Decelerate => 4.0,
                Keep => 1.0,
                _ => panic!("Unexpected Pattern"),
            }
        }
    }
}

impl PreferredSuccessor for SimpleAVVehicleInFrontMDP {
    fn preferred_successor(&self, s: &Self::State, a: &Self::Action) -> Self::State {
        if self.is_terminal(s) {
            *s
        } else {
            SimpleAVVehicleInFrontState::new(
                s.ego_vehicle
                    .next(a, self.range.maxy, self.range.mindy, self.range.maxdy),
                s.vehicle_in_front.next(
                    &Accelerate,
                    self.range.maxy,
                    self.range.mindy,
                    self.range.maxdy,
                ),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_terminal() {
        let start = SimpleAVVehicleInFrontState::new(
            VehicleConfiguration::new(0, 2),
            VehicleConfiguration::new(3, 2),
        );
        let problem = SimpleAVVehicleInFrontMDP::new(0, 10, -3, 3, start, NonYield(8, 2, 3));
        assert!(problem.is_terminal(&SimpleAVVehicleInFrontState::new(
            VehicleConfiguration::new(8, 2),
            VehicleConfiguration::new(10, 2),
        )));
        assert!(!problem.is_terminal(&SimpleAVVehicleInFrontState::new(
            VehicleConfiguration::new(4, 2),
            VehicleConfiguration::new(4, 0),
        )));
    }
}
