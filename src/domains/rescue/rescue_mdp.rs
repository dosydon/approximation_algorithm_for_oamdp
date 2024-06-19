use crate::common::coordinate2::Coordinate2;
use crate::mdp_traits::*;
use crate::rescue::rescue_action::RescueAction::*;
use crate::rescue::rescue_action::{get_di, get_dj};
use crate::rescue::ObstacleStatus::*;
use crate::rescue::{ObstacleStatus, RescueAction, RescueState};

use core::slice::Iter;
use itertools::iproduct;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, EnumIter)]
pub enum ObstacleCompatibility {
    Low,
    High,
}

#[derive(PartialEq, Debug, Clone)]
pub struct RescueMDP {
    height: usize,
    width: usize,
    is_obstacled: Vec<Vec<Option<usize>>>,
    all_states: Vec<RescueState>,
    all_actions: [RescueAction; 5],
    victim_coordinate: Coordinate2,
    obstacle_compatibility: ObstacleCompatibility,
}

impl GetNextStateFromPMass for RescueMDP {}

impl RescueMDP {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<(usize, usize)>,
        victim_coordinate: Coordinate2,
        obstacle_compatibility: ObstacleCompatibility,
    ) -> RescueMDP {
        let mut is_obstacled = vec![vec![None; width]; height];
        for (id, (i, j)) in obstacles.into_iter().enumerate() {
            is_obstacled[i][j] = Some(id);
        }

        RescueMDP {
            width: width,
            height: height,
            all_states: iproduct!(
                (0..height),
                (0..width),
                ObstacleStatus::iter(),
                ObstacleStatus::iter(),
                ObstacleStatus::iter()
            )
            .map(|(i, j, os0, os1, os2)| {
                RescueState::new(Coordinate2::new(i as i32, j as i32), [os0, os1, os2])
            })
            .collect::<Vec<_>>(),
            all_actions: [North, South, East, West, RemoveObstacle],
            is_obstacled: is_obstacled,
            victim_coordinate: victim_coordinate,
            obstacle_compatibility: obstacle_compatibility,
        }
    }

    fn within_bound(&self, i: i32, j: i32) -> bool {
        (i >= 0) && (i < self.height as i32) && (j >= 0) && (j < self.width as i32)
    }

    fn next(&self, s: &RescueState, a: &RescueAction) -> RescueState {
        let di = get_di(a);
        let dj = get_dj(a);
        let next_coordinate = Coordinate2::new(s.coordinate.i + di, s.coordinate.j + dj);
        if self.is_terminal(s) || !self.within_bound(next_coordinate.i, next_coordinate.j) {
            *s
        } else {
            RescueState::new(next_coordinate, s.obstacles_status)
        }
    }
}

impl ActionAvailability for RescueMDP {}

impl StatesActions for RescueMDP {
    type State = RescueState;
    type Action = RescueAction;
}

impl IsTerminal for RescueMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        s.coordinate == self.victim_coordinate
    }
}

impl StateEnumerable for RescueMDP {
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

impl ActionEnumerable for RescueMDP {
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

impl InitialState for RescueMDP {
    fn initial_state(&self) -> Self::State {
        RescueState::new(Coordinate2::new(2, 1), [NotRemoved, NotRemoved, NotRemoved])
    }
}

impl PMass<f32> for RescueMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if let Some(id) = self.is_obstacled[s.coordinate.i as usize][s.coordinate.j as usize] {
            if s.obstacles_status[id] == Removed {
                vec![(self.next(s, a), 1.0)]
            } else {
                let mut new_status = s.obstacles_status;
                new_status[id] = Removed;
                match a {
                    RemoveObstacle => match self.obstacle_compatibility {
                        ObstacleCompatibility::Low => {
                            vec![(RescueState::new(s.coordinate, new_status), 0.2), (*s, 0.8)]
                        }
                        ObstacleCompatibility::High => {
                            vec![(RescueState::new(s.coordinate, new_status), 0.8), (*s, 0.2)]
                        }
                    },
                    _ => vec![(*s, 1.0)],
                }
            }
        } else {
            vec![(self.next(s, a), 1.0)]
        }
    }
}

impl Cost for RescueMDP {
    fn cost(&self, s: &Self::State, _a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            1.0
        }
    }
}

impl DCost for RescueMDP {
    fn d_cost(&self, s: &Self::State, _a: &Self::Action, _ss: &Self::State) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            1.0
        }
    }
}

impl ExplicitTransition for RescueMDP {
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        if let Some(p) = self.p_mass(st, a).iter().find(|(s, _prob)| s == stt) {
            p.1
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::episode_runner::CostEpisodeIterator;
    use crate::policy::tabular_policy::TabularPolicy;
    use crate::value_iteration::value_iteration_ssp;
    use rand::thread_rng;

    #[test]
    fn test_rescue_p_mass() {
        let mdp = RescueMDP::new(
            3,
            3,
            vec![(0, 1), (1, 2), (2, 0)],
            Coordinate2::new(0, 2),
            ObstacleCompatibility::High,
        );
        let obstacles_status = [NotRemoved; 3];
        let obstacles_removed = [NotRemoved, NotRemoved, Removed];

        assert!(mdp.within_bound(1, 1));
        assert_eq!(
            vec![(
                RescueState::new(Coordinate2::new(1, 1), obstacles_status),
                1.0
            )],
            mdp.p_mass(
                &RescueState::new(Coordinate2::new(2, 1), obstacles_status),
                &North
            )
        );
        assert_eq!(
            vec![
                (
                    RescueState::new(Coordinate2::new(2, 0), obstacles_removed),
                    0.8
                ),
                (
                    RescueState::new(Coordinate2::new(2, 0), obstacles_status),
                    0.2
                )
            ],
            mdp.p_mass(
                &RescueState::new(Coordinate2::new(2, 0), obstacles_status),
                &RemoveObstacle
            )
        );
    }

    #[test]
    fn test_rescue_value_iteration() {
        let mdp = RescueMDP::new(
            3,
            3,
            vec![(0, 1), (1, 2), (2, 0)],
            Coordinate2::new(0, 2),
            ObstacleCompatibility::High,
        );
        let value_table = value_iteration_ssp(&mdp);
        let tabular_policy =
            TabularPolicy::<RescueState, RescueAction>::from_value_table_ssp(&mdp, &value_table);
        let mut rng = thread_rng();
        for (s, _, _, _) in CostEpisodeIterator::from_initial_state(&mdp, &tabular_policy, &mut rng)
        {
            println!("{:?}", s);
        }
    }
}
