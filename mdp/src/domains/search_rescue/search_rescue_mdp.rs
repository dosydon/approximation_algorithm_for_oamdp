use crate::mdp_traits::{
    ActionAvailability, ActionEnumerable, BuildFrom, Cost, DCost, ExplicitTransition,
    GetNextStateFromPMass, GetNextStateMutFromImmut, InitialState, IsTerminal, PMass, PMassMutFrom,
    StatesActions,
};
use crate::search_rescue::search_rescue_action::SearchRescueAction::*;
use crate::search_rescue::search_rescue_action::{get_di, get_dj};
use crate::search_rescue::search_rescue_partial_mdp::ObstacleCompatibility::*;
use crate::search_rescue::search_rescue_state::ObstacleStatus::*;
use crate::search_rescue::victim_status::VictimStatus::*;
use crate::search_rescue::{
    Coordinate, ObstacleCompatibility, SearchRescueAction, SearchRescueParameter,
    SearchRescuePartialMDP, SearchRescueState,
};
use core::slice::Iter;

#[derive(PartialEq, Debug, Clone)]
pub struct SearchRescueMDP {
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) is_obstacled: Vec<Vec<Option<usize>>>,
    all_actions: [SearchRescueAction; 4],
    pub(crate) victim_coordinate: Coordinate,
    base_coordinate: Coordinate,
    obstacle_compatibility: ObstacleCompatibility,
}

impl SearchRescueMDP {
    pub fn new(
        height: usize,
        width: usize,
        obstacles: Vec<(usize, usize)>,
        victim_coordinate: Coordinate,
        base_coordinate: Coordinate,
        obstacle_compatibility: ObstacleCompatibility,
    ) -> SearchRescueMDP {
        let mut is_obstacled = vec![vec![None; width]; height];
        for (id, (i, j)) in obstacles.into_iter().enumerate() {
            is_obstacled[i][j] = Some(id);
        }

        SearchRescueMDP {
            width: width,
            height: height,
            all_actions: [North, South, East, West],
            is_obstacled: is_obstacled,
            victim_coordinate: victim_coordinate,
            base_coordinate: base_coordinate,
            obstacle_compatibility: obstacle_compatibility,
        }
    }

    fn within_bound(&self, i: i64, j: i64) -> bool {
        (0 <= i) && (i < self.height as i64) && (j >= 0) && (j < self.width as i64)
    }
}

impl StatesActions for SearchRescueMDP {
    type State = SearchRescueState;
    type Action = SearchRescueAction;
}

impl IsTerminal for SearchRescueMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        s.victim_status == Handled
    }
}

impl ActionAvailability for SearchRescueMDP {}

impl ActionEnumerable for SearchRescueMDP {
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

impl PMass<f32> for SearchRescueMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        let dj = get_dj(a);
        let di = get_di(a);
        let next_coordinate = Coordinate::new(s.coordinate.i + di, s.coordinate.j + dj);

        if self.is_terminal(s) || !self.within_bound(s.coordinate.i + di, s.coordinate.j + dj) {
            vec![(*s, 1.0)]
        } else if let Some(id) =
            self.is_obstacled[next_coordinate.i as usize][next_coordinate.j as usize]
        {
            let mut new_status = s.obstacles_status;
            new_status[id] = Removed;
            vec![(
                SearchRescueState::from_coordinate(next_coordinate, s.victim_status, new_status),
                1.0,
            )]
        } else {
            if next_coordinate == self.victim_coordinate && s.victim_status == Unknown {
                vec![
                    (
                        SearchRescueState::from_coordinate(
                            next_coordinate,
                            NeedAmbulance,
                            s.obstacles_status,
                        ),
                        1.0 / 3.0,
                    ),
                    (
                        SearchRescueState::from_coordinate(
                            next_coordinate,
                            Handled,
                            s.obstacles_status,
                        ),
                        1.0 / 3.0,
                    ),
                    (
                        SearchRescueState::from_coordinate(
                            next_coordinate,
                            Hazard,
                            s.obstacles_status,
                        ),
                        1.0 / 3.0,
                    ),
                ]
            } else if next_coordinate == self.base_coordinate {
                match s.victim_status {
                    NeedAmbulance => vec![(
                        SearchRescueState::from_coordinate(
                            next_coordinate,
                            Handled,
                            s.obstacles_status,
                        ),
                        1.0,
                    )],
                    Hazard => vec![(
                        SearchRescueState::from_coordinate(
                            next_coordinate,
                            Handled,
                            s.obstacles_status,
                        ),
                        1.0,
                    )],
                    _ => vec![(
                        SearchRescueState::from_coordinate(
                            next_coordinate,
                            s.victim_status,
                            s.obstacles_status,
                        ),
                        1.0,
                    )],
                }
            } else {
                vec![(
                    SearchRescueState::from_coordinate(
                        next_coordinate,
                        s.victim_status,
                        s.obstacles_status,
                    ),
                    1.0,
                )]
            }
        }
    }
}

impl InitialState for SearchRescueMDP {
    fn initial_state(&self) -> Self::State {
        SearchRescueState::from_coordinate(
            self.base_coordinate,
            Unknown,
            [NotRemoved, NotRemoved, NotRemoved, NotRemoved],
        )
    }
}

impl ExplicitTransition for SearchRescueMDP {}

impl Cost for SearchRescueMDP {
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        self.p_mass(s, a)
            .into_iter()
            .map(|(stt, prob)| prob * self.d_cost(s, a, &stt))
            .sum()
    }
}

impl PMassMutFrom<f32> for SearchRescueMDP {}
impl GetNextStateFromPMass for SearchRescueMDP {}
impl GetNextStateMutFromImmut for SearchRescueMDP {}

impl DCost for SearchRescueMDP {
    fn d_cost(
        &self,
        st: &SearchRescueState,
        _a: &SearchRescueAction,
        stt: &SearchRescueState,
    ) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else if let Some(id) =
            self.is_obstacled[stt.coordinate.i as usize][stt.coordinate.j as usize]
        {
            if st.obstacles_status[id] == Removed {
                1.0
            } else {
                match self.obstacle_compatibility {
                    Low => 10.0,
                    High => 1.0,
                }
            }
        } else {
            1.0
        }
    }
}

impl<'a> BuildFrom<&'a SearchRescueParameter, SearchRescueMDP> for SearchRescuePartialMDP {
    fn build_from(&self, parameter: &'a SearchRescueParameter) -> SearchRescueMDP {
        SearchRescueMDP::new(
            self.height,
            self.width,
            self.obstacles.clone(),
            self.victim_coordinate,
            parameter.base_coordinate,
            parameter.obstacle_compatibility,
        )
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::{
        episode_runner::EpisodeRunner, policy::tabular_policy::TabularPolicy,
        state_enumerable_wrapper::StateEnumerableWrapper, value_iteration::value_iteration_ssp,
    };

    use super::*;
    #[test]
    fn test_search_rescue_p_mass() {
        let mdp = SearchRescueMDP::new(
            5,
            5,
            vec![(4, 1)],
            Coordinate::new(0, 4),
            Coordinate::new(4, 2),
            High,
        );
        let obstacles_status = [NotRemoved; 4];

        assert_eq!(
            vec![(SearchRescueState::new(3, 2, Unknown, obstacles_status), 1.0)],
            mdp.p_mass(
                &SearchRescueState::new(4, 2, Unknown, obstacles_status),
                &North
            )
        );
        assert_eq!(
            vec![(SearchRescueState::new(4, 2, Unknown, obstacles_status), 1.0)],
            mdp.p_mass(
                &SearchRescueState::new(3, 2, Unknown, obstacles_status),
                &South
            )
        );
        assert_eq!(
            vec![
                (
                    SearchRescueState::new(0, 4, NeedAmbulance, obstacles_status),
                    1.0 / 3.0
                ),
                (
                    SearchRescueState::new(0, 4, Handled, obstacles_status),
                    1.0 / 3.0
                ),
                (
                    SearchRescueState::new(0, 4, Hazard, obstacles_status),
                    1.0 / 3.0
                ),
            ],
            mdp.p_mass(
                &SearchRescueState::new(0, 3, Unknown, obstacles_status),
                &East
            )
        );
        assert_eq!(
            vec![(SearchRescueState::new(4, 2, Handled, obstacles_status), 1.0)],
            mdp.p_mass(
                &SearchRescueState::new(3, 2, NeedAmbulance, obstacles_status),
                &South
            )
        );
    }

    #[test]
    fn test_search_rescue_value_iteration() {
        let mdp = SearchRescueMDP::new(
            5,
            5,
            vec![(4, 1)],
            Coordinate::new(0, 4),
            Coordinate::new(4, 2),
            High,
        );
        let mdp = StateEnumerableWrapper::new(mdp);
        let vt = value_iteration_ssp(&mdp);
        let policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        let mut rng = thread_rng();
        let mut runner = EpisodeRunner::new(&mdp, &policy, mdp.initial_state());
        for (s, _a, _c, _ss) in runner.into_iter_with(&mut rng) {
            println!("{:?}", s);
        }
    }
}
