use crate::into_inner::InnerMost;
use crate::mdp_traits::{
    ActionAvailability, ActionEnumerable, BuildFrom, CostFromDCost, DCost, ExplicitTransition,
    GetNextStateFromPMass, InitialState, IsTerminal, PMass, StateEnumerable, StatesActions,
};
use crate::simple_av::action::SimpleAVAction::*;
use crate::simple_av::succ::usize_succ_bound;
use crate::simple_av::vehicle_configuration::VehicleConfiguration;
use crate::simple_av::SimpleAVAction;
use core::slice::Iter;
use itertools::iproduct;
use mdp_derive::InnerMost;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SimpleAVPedestrianParameter {
    Yield,
    NonYield,
    Far,
    FastPedestrian,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, InnerMost)]
pub struct SimpleAVPedestrianState {
    pub ego_vehicle: VehicleConfiguration,
    pub pedestrian_configuration: usize,
}

impl SimpleAVPedestrianState {
    pub fn y(&self) -> usize {
        self.ego_vehicle.y
    }

    pub fn dy(&self) -> i32 {
        self.ego_vehicle.dy
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }
}

pub struct SimpleAVPedestrianPartialMDP {
    pub(in crate::domains::simple_av) maxy: usize,
    pub(in crate::domains::simple_av) max_pedestrian: usize,
    pub(in crate::domains::simple_av) mindy: i32,
    pub(in crate::domains::simple_av) maxdy: i32,
    pub(in crate::domains::simple_av) start: SimpleAVPedestrianState,
}

impl SimpleAVPedestrianPartialMDP {
    pub fn new(maxy: usize, max_pedestrian: usize, maxdy: i32) -> SimpleAVPedestrianPartialMDP {
        let start = SimpleAVPedestrianState::new(VehicleConfiguration::new(0, 3), 0);

        SimpleAVPedestrianPartialMDP {
            maxy: maxy,
            max_pedestrian: max_pedestrian,
            mindy: 0,
            maxdy: maxdy,
            start: start,
        }
    }
}

impl<'a> BuildFrom<&'a SimpleAVPedestrianParameter, SimpleAVPedestrianMDP>
    for SimpleAVPedestrianPartialMDP
{
    fn build_from(&self, parameter: &'a SimpleAVPedestrianParameter) -> SimpleAVPedestrianMDP {
        SimpleAVPedestrianMDP::new(
            self.maxy,
            self.max_pedestrian,
            self.mindy,
            self.maxdy,
            self.start,
            *parameter,
        )
    }
}

impl SimpleAVPedestrianState {
    pub fn new(
        ego_vehicle: VehicleConfiguration,
        pedestrian_configuration: usize,
    ) -> SimpleAVPedestrianState {
        SimpleAVPedestrianState {
            ego_vehicle: ego_vehicle,
            pedestrian_configuration: pedestrian_configuration,
        }
    }
}

pub struct SimpleAVPedestrianMDP {
    pub(in crate::domains::simple_av) maxy: usize,
    max_pedestrian: usize,
    mindy: i32,
    maxdy: i32,
    all_states: Vec<SimpleAVPedestrianState>,
    all_actions: Vec<SimpleAVAction>,
    start: SimpleAVPedestrianState,
    parameter: SimpleAVPedestrianParameter,
}

impl SimpleAVPedestrianMDP {
    pub fn new(
        maxy: usize,
        max_pedestrian: usize,
        mindy: i32,
        maxdy: i32,
        start: SimpleAVPedestrianState,
        parameter: SimpleAVPedestrianParameter,
    ) -> SimpleAVPedestrianMDP {
        let all_states = iproduct!((0..=maxy), (mindy..=maxdy), (0..=max_pedestrian))
            .map(|(y, dy, pedestrian)| {
                SimpleAVPedestrianState::new(VehicleConfiguration::new(y, dy), pedestrian)
            })
            .collect::<Vec<_>>();

        SimpleAVPedestrianMDP {
            maxy,
            max_pedestrian,
            mindy,
            maxdy,
            all_states: all_states,
            all_actions: vec![Keep, Accelerate, Decelerate, Stop, Accelerate2],
            //             all_actions: vec![Keep, Accelerate, Decelerate, Stop, Accelerate2],
            start: start,
            parameter,
        }
    }
}

impl SimpleAVPedestrianMDP {
    pub fn default() -> SimpleAVPedestrianMDP {
        SimpleAVPedestrianMDP::new(
            30,
            10,
            0,
            4,
            SimpleAVPedestrianState::new(VehicleConfiguration::new(0, 3), 0),
            SimpleAVPedestrianParameter::Yield,
        )
    }

    #[allow(non_snake_case)]
    pub fn isHit(&self, s: &SimpleAVPedestrianState) -> bool {
        match self.parameter {
            SimpleAVPedestrianParameter::Yield => {
                17 <= s.ego_vehicle.y
                    && s.ego_vehicle.y <= 21
                    && 4 <= s.pedestrian_configuration
                    && s.pedestrian_configuration <= 8
            }
            SimpleAVPedestrianParameter::NonYield => false,
            SimpleAVPedestrianParameter::Far => {
                17 <= s.ego_vehicle.y
                    && s.ego_vehicle.y <= 21
                    && 7 <= s.pedestrian_configuration
                    && s.pedestrian_configuration <= 10
            }
            SimpleAVPedestrianParameter::FastPedestrian => {
                17 <= s.ego_vehicle.y
                    && s.ego_vehicle.y <= 21
                    && 4 <= s.pedestrian_configuration
                    && s.pedestrian_configuration <= 8
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn nonYield() -> SimpleAVPedestrianMDP {
        SimpleAVPedestrianMDP::new(
            30,
            10,
            0,
            4,
            SimpleAVPedestrianState::new(VehicleConfiguration::new(0, 3), 0),
            SimpleAVPedestrianParameter::Yield,
        )
    }

    pub fn far() -> SimpleAVPedestrianMDP {
        SimpleAVPedestrianMDP::new(
            30,
            10,
            0,
            4,
            SimpleAVPedestrianState::new(VehicleConfiguration::new(0, 3), 0),
            SimpleAVPedestrianParameter::Far,
        )
    }

    #[allow(non_snake_case)]
    pub fn initialState(&self) -> SimpleAVPedestrianState {
        self.initial_state()
    }

    #[allow(non_snake_case)]
    pub fn getNextState(
        &self,
        s: &SimpleAVPedestrianState,
        a: SimpleAVAction,
        dPedestrian: i32,
    ) -> SimpleAVPedestrianState {
        SimpleAVPedestrianState {
            ego_vehicle: s.ego_vehicle.next(&a, self.maxy, self.mindy, self.maxdy),
            pedestrian_configuration: usize_succ_bound(
                s.pedestrian_configuration,
                dPedestrian,
                self.max_pedestrian,
            ),
        }
    }

    #[allow(non_snake_case)]
    pub fn isTerminal(&self, s: &SimpleAVPedestrianState) -> bool {
        self.is_terminal(s)
    }
}

impl StatesActions for SimpleAVPedestrianMDP {
    type State = SimpleAVPedestrianState;
    type Action = SimpleAVAction;
}

impl IsTerminal for SimpleAVPedestrianMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        s.ego_vehicle.y >= self.maxy && s.ego_vehicle.dy == 3
    }
}

impl ActionEnumerable for SimpleAVPedestrianMDP {
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

impl StateEnumerable for SimpleAVPedestrianMDP {
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

impl ActionAvailability for SimpleAVPedestrianMDP {}

impl InitialState for SimpleAVPedestrianMDP {
    fn initial_state(&self) -> Self::State {
        self.start
    }
}

impl PMass<f32> for SimpleAVPedestrianMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if self.is_terminal(s) {
            vec![(*s, 1.0)]
        } else {
            let pedestrian_configuration = [
                (
                    usize_succ_bound(s.pedestrian_configuration, 1, self.max_pedestrian),
                    0.9,
                ),
                (s.pedestrian_configuration, 0.1),
            ];
            let ego_vehicle = [(
                s.ego_vehicle.next(a, self.maxy, self.mindy, self.maxdy),
                1.0,
            )];
            let result = iproduct!(ego_vehicle.iter(), pedestrian_configuration.iter())
                .map(|((vehicle, vp), (pedestrian, pp))| {
                    (
                        SimpleAVPedestrianState::new(*vehicle, *pedestrian),
                        (*vp) * (*pp),
                    )
                })
                .collect();
            //             println!("{:?} {:?} {:?}", s, a, result);
            result
        }
    }
}

impl GetNextStateFromPMass for SimpleAVPedestrianMDP {}

// impl GenerativeMDPMut for SimpleAVPedestrianMDP {
//     fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
//         self.p_mass(s, a)
//     }
// }

impl ExplicitTransition for SimpleAVPedestrianMDP {}

impl DCost for SimpleAVPedestrianMDP {
    fn d_cost(&self, st: &Self::State, _a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else {
            //             let mut cost = ((self.maxy - stt.ego_vehicle.y) as f32).sqrt();
            //             let mut cost = 0.0;
            //             cost += match a {
            //                 Accelerate => 2.0,
            //                 Decelerate => 2.0,
            //                 Keep => 1.0,
            //                 Stop => 2.0,
            //                 Accelerate2 => 2.0,
            //             };
            let mut cost = 1.0;
            if stt.ego_vehicle.dy == 0 || stt.ego_vehicle.dy == 1 || stt.ego_vehicle.dy == 4 {
                cost += 5.0;
            }
            if self.isHit(&stt) {
                cost += 1000.0;
            }

            cost
        }
    }
}

impl CostFromDCost for SimpleAVPedestrianMDP {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::episode_runner::EpisodeRunner;
    use crate::policy::tabular_policy::TabularPolicy;
    use crate::value_iteration::value_iteration_ssp;
    use rand::prelude::*;

    #[test]
    fn test_av_pedestrian() {
        let start = SimpleAVPedestrianState::new(VehicleConfiguration::new(0, 3), 0);
        let mdp =
            SimpleAVPedestrianMDP::new(30, 10, 0, 4, start, SimpleAVPedestrianParameter::Yield);
        println!("{:?}", mdp.num_states());
        let value_table = value_iteration_ssp(&mdp);
        let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
        let mut rng = thread_rng();
        let mut runner = EpisodeRunner::from_initial_state(&mdp, &tabular_policy);
        for (s, a, _ss, r) in runner.into_iter_with(&mut rng) {
            println!("{:?} {:?} {:?}", s, a, r);
        }
        //         let result = runner.monte_carlo_evaluation(&mdp, &tabular_policy, &mut rng);
        //         println!("{:?}", result);
    }

    #[test]
    fn test_av_pedestrian_far() {
        let start = SimpleAVPedestrianState::new(VehicleConfiguration::new(0, 3), 0);
        let mdp = SimpleAVPedestrianMDP::new(30, 10, 0, 4, start, SimpleAVPedestrianParameter::Far);
        println!("{:?}", mdp.num_states());
        //         let value_table = value_iteration_ssp(&mdp);
        //         let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
        //         let mut rng = thread_rng();
        //         let runner = EpisodeRunner::new();
        //         let result = runner.monte_carlo_evaluation(&mdp, &tabular_policy, &mut rng);
        //         println!("{:?}", result);
    }

    #[test]
    fn test_av_pedestrian_fast_pedestiran() {
        let start = SimpleAVPedestrianState::new(VehicleConfiguration::new(0, 3), 0);
        let mdp = SimpleAVPedestrianMDP::new(
            30,
            10,
            0,
            4,
            start,
            SimpleAVPedestrianParameter::FastPedestrian,
        );
        println!("{:?}", mdp.num_states());
        //         let value_table = value_iteration_ssp(&mdp);
        //         let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
        //         let mut rng = thread_rng();
        //         let runner = EpisodeRunner::new();
        //         let result = runner.monte_carlo_evaluation(&mdp, &tabular_policy, &mut rng);
        //         println!("{:?}", result);
    }
}
