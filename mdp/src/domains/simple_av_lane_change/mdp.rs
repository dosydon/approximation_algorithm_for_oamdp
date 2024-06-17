use self::SimpleAVLaneChangeParameter::*;
use crate::mdp_traits::*;
use crate::simple_av::vehicle_configuration::VehicleConfiguration;
use crate::simple_av::SimpleAVAction::*;
use crate::simple_av_lane_change::action::SimpleAVLaneChangeAction;
use crate::simple_av_lane_change::action::Steering;
use crate::simple_av_lane_change::lane::Lane;
use crate::simple_av_lane_change::parameter::SimpleAVLaneChangeParameter;
use crate::simple_av_lane_change::state::SimpleAVLaneChangeState;
use crate::simple_av_lane_change::vehicle_configuration_lane::VehicleConfigurationLane;
use crate::strum::IntoEnumIterator;
use core::slice::Iter;
use itertools::iproduct;


pub struct SimpleAVLaneChangeMDP {
    maxy: usize,
    mindy: i32,
    maxdy: i32,
    all_states: Vec<SimpleAVLaneChangeState>,
    all_actions: Vec<SimpleAVLaneChangeAction>,
    start: SimpleAVLaneChangeState,
    parameter: SimpleAVLaneChangeParameter,
}

impl SimpleAVLaneChangeMDP {
    pub fn new(
        maxy: usize,
        mindy: i32,
        maxdy: i32,
        start: SimpleAVLaneChangeState,
        parameter: SimpleAVLaneChangeParameter,
    ) -> SimpleAVLaneChangeMDP {
        let all_states = iproduct!(
            (0..=maxy),
            (mindy..=maxdy),
            (0..=maxy),
            (mindy..=maxdy),
            Lane::iter()
        )
        .map(|(y, dy, yy, dyy, lane)| {
            (
                VehicleConfigurationLane::new(y, dy, lane),
                VehicleConfiguration::new(yy, dyy),
            )
        })
        .map(|(ego, other)| SimpleAVLaneChangeState::new(ego, other))
        .collect::<Vec<_>>();

        let all_actions = iproduct!([Keep, Accelerate, Decelerate].iter(), Steering::iter())
            .map(|(a, steering)| SimpleAVLaneChangeAction {
                acceleration: *a,
                steering: steering,
            })
            .collect::<Vec<_>>();

        SimpleAVLaneChangeMDP {
            maxy,
            mindy,
            maxdy,
            all_states: all_states,
            all_actions: all_actions,
            start: start,
            parameter,
        }
    }
}

impl StatesActions for SimpleAVLaneChangeMDP {
    type State = SimpleAVLaneChangeState;
    type Action = SimpleAVLaneChangeAction;
}

impl IsTerminal for SimpleAVLaneChangeMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        let ego_vehicle = s.ego_vehicle;
        match self.parameter {
            NotChanging => ego_vehicle.y >= self.maxy,
            _ => ego_vehicle.y >= self.maxy && ego_vehicle.lane == Lane::LeftCenter,
        }
    }
}

impl GetNextStateFromPMass for SimpleAVLaneChangeMDP {}

impl ActionEnumerable for SimpleAVLaneChangeMDP {
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

impl StateEnumerable for SimpleAVLaneChangeMDP {
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

impl InitialState for SimpleAVLaneChangeMDP {
    fn initial_state(&self) -> Self::State {
        self.start
    }
}

impl PMass<f32> for SimpleAVLaneChangeMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) {
            vec![(*s, 1.0)]
        } else {
            vec![
                (
                    SimpleAVLaneChangeState::new(
                        s.ego_vehicle.next(
                            a,
                            self.maxy,
                            self.mindy,
                            self.maxdy,
                            s.ego_vehicle.lane,
                        ),
                        s.other_vehicle
                            .next(&Keep, self.maxy, self.mindy, self.maxdy),
                    ),
                    0.95,
                ),
                (
                    SimpleAVLaneChangeState::new(
                        s.ego_vehicle.next(
                            a,
                            self.maxy,
                            self.mindy,
                            self.maxdy,
                            s.ego_vehicle.lane,
                        ),
                        s.other_vehicle
                            .next(&Accelerate, self.maxy, self.mindy, self.maxdy),
                    ),
                    0.05,
                ),
            ]
        }
    }
}

fn is_left_lane(ego: &Lane) -> bool {
    match ego {
        Lane::LeftLeft => true,
        Lane::LeftCenter => true,
        Lane::LeftRight => true,
        _ => false,
    }
}

// fn is_right_lane(ego: &Lane) -> bool {
//     match ego {
//         Lane::RightLeft => true,
//         Lane::RightCenter => true,
//         Lane::RightRight => true,
//         _ => false,
//     }
// }
//
// fn is_same_lane(ego: &Lane, other: &Lane) -> bool {
//     (is_left_lane(ego) && is_left_lane(other)) || (is_right_lane(ego) && is_right_lane(other))
// }

fn diff(ego: usize, other: usize) -> usize {
    if ego < other {
        other - ego
    } else {
        ego - other
    }
}

impl DCost for SimpleAVLaneChangeMDP {
    fn d_cost(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        let mut cost = 0.0;
        if self.is_terminal(st) {
            cost = 0.0;
        } else {
            if diff(stt.ego_vehicle.y, stt.other_vehicle.y) <= 2
                && stt.other_vehicle.y < self.maxy
                && is_left_lane(&stt.ego_vehicle.lane)
            {
                cost += 100.0;
            }

            if self.parameter == Yield
                && is_left_lane(&stt.ego_vehicle.lane)
                && stt.ego_vehicle.y >= stt.other_vehicle.y
            {
                cost += 10.0;
            }

            if !(st.ego_vehicle.lane == stt.ego_vehicle.lane) {
                cost += 1.0;
            }

            cost += match a.acceleration {
                Accelerate => 4.0,
                Decelerate => 4.0,
                Keep => 1.0,
                _ => panic!("not possible"),
            };
        }

        cost
    }
}

impl Cost for SimpleAVLaneChangeMDP {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        self.p_mass(s, a)
            .into_iter()
            .map(|(stt, prob)| prob * self.d_cost(s, a, &stt))
            .sum()
    }
}

impl ActionAvailability for SimpleAVLaneChangeMDP {}

impl ExplicitTransition for SimpleAVLaneChangeMDP {
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        self.p_mass(st, a)
            .into_iter()
            .filter(|&(next_state, _p)| next_state == *stt)
            .map(|(_next_state, p)| p)
            .sum()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::episode_runner::CostEpisodeIterator;
//     use crate::policy::tabular_policy::TabularPolicy;
//     use crate::value_iteration::value_iteration_ssp;
//     use rand::prelude::*;
//
//     #[test]
//     fn test_av_lane_change() {
//         let start = SimpleAVLaneChangeState::new(
//             VehicleConfigurationLane::new(0, 2, Lane::RightCenter),
//             VehicleConfiguration::new(0, 2),
//         );
//         let mdp = SimpleAVLaneChangeMDP::new(20, 0, 3, start, SimpleAVLaneChangeParameter::Yield);
//         println!("{:?}", mdp.num_states());
//         let value_table = value_iteration_ssp(&mdp);
//         let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
//         let mut rng = thread_rng();
//         for (s, _, _) in CostEpisodeIterator::from_initial_state(&mdp, &tabular_policy, &mut rng) {
//             println!("{:?}", s);
//         }
//     }
// }
//
