use self::ObstacleAvoidanceParameter::*;
use crate::common::av1d_map::AV1dRange;
use crate::mdp_traits::{
    ActionAvailability, ActionEnumerable, Cost, DCost, ExplicitTransition, GetNextStateFromPMass,
    GetNextStateMutFromImmut, InitialState, IsTerminal, PMass, StateEnumerable, StatesActions,
};
use crate::simple_av::vehicle_configuration::VehicleConfiguration;
use crate::simple_av::SimpleAVAction::*;
use crate::simple_av_obstacle_avoidance::action::ObstacleAvoidanceAction;
use crate::simple_av_obstacle_avoidance::lane::Lane;
use crate::simple_av_obstacle_avoidance::parameter::ObstacleAvoidanceParameter;
use crate::simple_av_obstacle_avoidance::state::ObstacleAvoidanceState;
use crate::simple_av_obstacle_avoidance::vehicle_configuration_lane::VehicleConfigurationLane;
use crate::strum::IntoEnumIterator;
use core::slice::Iter;
use itertools::iproduct;

pub struct ObstacleAvoidanceMDP {
    pub range: AV1dRange,
    all_states: Vec<ObstacleAvoidanceState>,
    all_actions: Vec<ObstacleAvoidanceAction>,
    start: ObstacleAvoidanceState,
    pub parameter: ObstacleAvoidanceParameter,
    pub(crate) collision_zone_lb: usize,
    pub(crate) collision_zone_ub: usize,
}

impl ObstacleAvoidanceMDP {
    pub fn new(
        maxy: usize,
        mindy: i32,
        maxdy: i32,
        start: ObstacleAvoidanceState,
        parameter: ObstacleAvoidanceParameter,
    ) -> ObstacleAvoidanceMDP {
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
        .map(|(ego, other)| ObstacleAvoidanceState::new(ego, other))
        .collect::<Vec<_>>();

        let all_actions = iproduct!([Keep, Accelerate, Decelerate].iter(), Lane::iter())
            .map(|(a, lane)| ObstacleAvoidanceAction {
                acceleration: *a,
                next_lane: lane,
            })
            .collect::<Vec<_>>();

        ObstacleAvoidanceMDP {
            range: AV1dRange::new(0, maxy, mindy, maxdy),
            all_states: all_states,
            all_actions: all_actions,
            start: start,
            parameter,
            collision_zone_lb: 11,
            collision_zone_ub: 19,
        }
    }
}

impl StatesActions for ObstacleAvoidanceMDP {
    type State = ObstacleAvoidanceState;
    type Action = ObstacleAvoidanceAction;
}

impl IsTerminal for ObstacleAvoidanceMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        let ego_vehicle = s.ego_vehicle;
        ego_vehicle.y >= self.range.maxy && ego_vehicle.lane == Lane::Center
    }
}

impl ActionEnumerable for ObstacleAvoidanceMDP {
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

impl StateEnumerable for ObstacleAvoidanceMDP {
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

impl InitialState for ObstacleAvoidanceMDP {
    fn initial_state(&self) -> Self::State {
        self.start
    }
}

impl PMass<f32> for ObstacleAvoidanceMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) {
            vec![(*s, 1.0)]
        } else {
            vec![
                (
                    ObstacleAvoidanceState::new(
                        s.ego_vehicle.next(
                            a,
                            self.range.maxy,
                            self.range.mindy,
                            self.range.maxdy,
                            s.ego_vehicle.lane,
                        ),
                        s.other_vehicle.next(
                            &Keep,
                            self.range.maxy,
                            self.range.mindy,
                            self.range.maxdy,
                        ),
                    ),
                    0.95,
                ),
                (
                    ObstacleAvoidanceState::new(
                        s.ego_vehicle.next(
                            a,
                            self.range.maxy,
                            self.range.mindy,
                            self.range.maxdy,
                            s.ego_vehicle.lane,
                        ),
                        s.other_vehicle.next(
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

impl ExplicitTransition for ObstacleAvoidanceMDP {}

impl Cost for ObstacleAvoidanceMDP {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        self.p_mass(s, a)
            .into_iter()
            .map(|(stt, prob)| prob * self.d_cost(s, a, &stt))
            .sum()
    }
}

impl ObstacleAvoidanceMDP {
    fn is_ego_vehicle_in_collision_zone(&self, stt: &ObstacleAvoidanceState) -> bool {
        self.collision_zone_lb <= stt.ego_vehicle.y && stt.ego_vehicle.y <= self.collision_zone_ub
    }

    fn is_other_vehicle_in_collision_zone(&self, stt: &ObstacleAvoidanceState) -> bool {
        self.collision_zone_lb <= stt.other_vehicle.y
            && stt.other_vehicle.y <= self.collision_zone_ub
    }
}

impl DCost for ObstacleAvoidanceMDP {
    fn d_cost(
        &self,
        st: &ObstacleAvoidanceState,
        a: &ObstacleAvoidanceAction,
        stt: &ObstacleAvoidanceState,
    ) -> f32 {
        let mut cost = 0.0;
        if self.is_terminal(st) {
            return 0.0;
        }

        if self.is_ego_vehicle_in_collision_zone(stt) {
            if stt.ego_vehicle.lane == Lane::Left {
                if (self.parameter == AwareYielding || self.parameter == AwareNotYielding)
                    && self.is_other_vehicle_in_collision_zone(stt)
                {
                    cost += 25.0;
                }
            }
            //             else {
            //                 cost += 25.0;
            //             }
        }

        if st.ego_vehicle.lane != Lane::Center {
            cost += 0.1;
        }

        cost += match a.acceleration {
            Accelerate => 1.1,
            Decelerate => 1.1,
            Keep => 1.0,
            _ => panic!("not possible"),
        };

        if self.parameter == AwareYielding
            && stt.ego_vehicle.y >= self.collision_zone_lb
            && stt.other_vehicle.y <= self.collision_zone_ub
        {
            cost += 25.0;
        }

        if self.parameter != AwareYielding && stt.ego_vehicle.y <= self.collision_zone_lb {
            cost += 1.0;
        }

        assert!(cost > 0.0, "{:?} {:?} {:?}", st, a, stt);
        cost
    }
}

// impl Cost for SimpleAVObstacleAvoidanceMDP {
//     fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
//         self.p_mass(s, a)
//             .into_iter()
//             .map(|(stt, prob)| prob * self.d_cost(s, a, &stt))
//             .sum()
//     }
// }

impl ActionAvailability for ObstacleAvoidanceMDP {}

impl GetNextStateFromPMass for ObstacleAvoidanceMDP {}
impl GetNextStateMutFromImmut for ObstacleAvoidanceMDP {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::episode_runner::CostEpisodeIterator;
//     use crate::mdp_traits::Builder;
//     use crate::policy::tabular_policy::TabularPolicy;
//     use crate::simple_av_obstacle_avoidance::ObstacleAvoidanceBuilder;
//     use crate::value_iteration::value_iteration_ssp;
//     use rand::prelude::*;
//
//     #[test]
//     fn test_av_obstacle_avoidance() {
//         let start = ObstacleAvoidanceState::new(
//             VehicleConfigurationLane::new(0, 3, Lane::Center),
//             VehicleConfiguration::new(0, 3),
//         );
//         let mdp = ObstacleAvoidanceMDP::new(30, 0, 4, start, AwareYielding);
//         let value_table = value_iteration_ssp(&mdp);
//         let policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
//
//         let mut rng = thread_rng();
//
//         for (s, _, _) in CostEpisodeIterator::from_initial_state(&mdp, &policy, &mut rng) {
//             println!("{:?}", s);
//         }
//     }
//
//     #[test]
//     fn test_av_obstacle_avoidance_collision_zone() {
//         let builder = ObstacleAvoidanceBuilder::new(30, 4);
//         let mdp = builder.build(AwareYielding);
//         let value_table = value_iteration_ssp(&mdp);
//         let policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
//
//         let mut rng = thread_rng();
//
//         for (s, _, _) in CostEpisodeIterator::from_initial_state(&mdp, &policy, &mut rng) {
//             println!("{:?}", s);
//         }
//     }
//
//     #[test]
//     fn test_av_obstacle_avoidance_aware_not_yielding() {
//         let builder = ObstacleAvoidanceBuilder::new(30, 4).set_collision_zone(12, 18);
//         let mdp = builder.build(AwareNotYielding);
//         let value_table = value_iteration_ssp(&mdp);
//         let policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
//
//         let mut rng = thread_rng();
//
//         for (s, _, _) in CostEpisodeIterator::from_initial_state(&mdp, &policy, &mut rng) {
//             println!("{:?}", s);
//         }
//     }
// }
