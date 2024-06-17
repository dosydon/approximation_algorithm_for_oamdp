use self::TrackStatus::*;
use crate::mdp_traits::*;
use crate::race_track::race_track_action::RaceTrackAction::*;
use crate::race_track::race_track_action::*;
use crate::race_track::race_track_state::RaceTrackState::*;
use crate::race_track::race_track_state::*;
use core::slice::Iter;
use log::debug;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(PartialEq, Debug, Clone, Copy)]
enum TrackStatus {
    Wall,
    Goal,
    Start,
    Blank,
    Error,
    Pothole,
    Unsafe,
}

#[derive(Clone)]
pub struct RaceTrackMDP {
    pub h: usize,
    pub w: usize,
    goals: HashSet<(usize, usize)>,
    starts: HashSet<(usize, usize)>,
    all_actions: Vec<RaceTrackAction>,
    track: Vec<Vec<TrackStatus>>,
    p_slip: f64,
    cost_when_safe: f64,
}

impl RaceTrackMDP {
    fn new(
        h: usize,
        w: usize,
        goals: HashSet<(usize, usize)>,
        starts: HashSet<(usize, usize)>,
        track: Vec<Vec<TrackStatus>>,
    ) -> RaceTrackMDP {
        RaceTrackMDP {
            h: h,
            w: w,
            goals: goals,
            starts: starts,
            all_actions: vec![
                North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest, Keep,
            ],
            track: track,
            p_slip: 0.0,
            cost_when_safe: 1.0,
        }
    }

    pub fn from_file(filename: &str) -> RaceTrackMDP {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let wl = lines.next().unwrap().unwrap();
        let w = wl.trim().parse::<usize>().unwrap() + 2;
        let h = lines.next().unwrap().unwrap().parse::<usize>().unwrap() + 2;
        let mut track = vec![];

        track.push(vec![TrackStatus::Wall; w as usize]);
        for line in lines {
            let mut row = vec![TrackStatus::Wall];
            if let Ok(ip) = line {
                //                 println!("{}", ip);
                for c in ip.chars() {
                    let s = match c {
                        'X' => TrackStatus::Wall,
                        'x' => TrackStatus::Wall,
                        'G' => TrackStatus::Goal,
                        'S' => TrackStatus::Start,
                        ' ' => TrackStatus::Blank,
                        'o' => TrackStatus::Error,
                        '.' => TrackStatus::Unsafe,
                        _ => panic!("unexpected character {}", c),
                    };
                    row.push(s);
                }
            }
            row.push(TrackStatus::Wall);
            track.push(row);
        }
        track.push(vec![TrackStatus::Wall; w]);
        //         for row in track.iter() {
        //             println!("{:?}", row);
        //         }

        let mut goals = HashSet::new();
        let mut starts = HashSet::new();
        for (y, row) in track.iter().enumerate() {
            for (x, s) in row.iter().enumerate() {
                match s {
                    TrackStatus::Goal => {
                        goals.insert((x, h - y - 1));
                    }
                    TrackStatus::Start => {
                        starts.insert((x, h - y - 1));
                    }
                    _ => (),
                }
            }
        }

        RaceTrackMDP::new(h, w, goals, starts, track)
    }

    pub fn set_p_slip(mut self, p_slip: f64) -> RaceTrackMDP {
        self.p_slip = p_slip;
        self
    }

    pub fn set_cost_when_safe(mut self, cost_when_safe: f64) -> RaceTrackMDP {
        self.cost_when_safe = cost_when_safe;
        self
    }

    fn get_status(&self, x: usize, y: usize) -> TrackStatus {
        //         println!("{} {}", x, y);
        assert!(self.within_bound(x, y));
        self.track[self.h - y - 1][x]
    }

    pub fn is_unsafe(&self, x: usize, y: usize) -> bool {
        match self.get_status(x, y) {
            Unsafe => true,
            _ => false,
        }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        match self.get_status(x, y) {
            Wall => true,
            _ => false,
        }
    }

    pub fn cost_when_safe(&self) -> f64 {
        self.cost_when_safe
    }

    fn within_bound(&self, x: usize, y: usize) -> bool {
        x < self.w && y < self.h
    }

    fn success(&self, st: &RaceTrackStateInner, at: &RaceTrackAction) -> RaceTrackStateInner {
        let dx = st.dx + get_ddx(*at);
        let dy = st.dy + get_ddy(*at);
        debug!("success {:?} {:?} {} {}", st, at, dx, dy);
        let m = 2 * (dx.abs() + dy.abs());
        if m == 0 {
            return RaceTrackStateInner::new(st.x, st.y, 0, 0);
        }
        for d in 0..=m {
            let x2 = (st.x as f32 + dx as f32 * (d as f32) / (m as f32)).round() as usize;
            let y2 = (st.y as f32 + dy as f32 * (d as f32) / (m as f32)).round() as usize;
            assert!(self.within_bound(x2, y2));
            debug!("x2:{} y2:{}", x2, y2,);

            if self.get_status(x2, y2) == TrackStatus::Wall
                || self.get_status(x2, y2) == TrackStatus::Pothole
            {
                return RaceTrackStateInner::new(x2, y2, 0, 0);
            }
            if self.get_status(x2, y2) == TrackStatus::Goal {
                return RaceTrackStateInner::new(x2, y2, dx, dy);
            }
        }
        RaceTrackStateInner::new(next(st.x, dx), next(st.y, dy), dx, dy)
    }
}

fn next(x: usize, dx: i32) -> usize {
    //     println!("x:{} dx:{}", x, dx);
    assert!(x as i32 + dx >= 0);
    (x as i32 + dx) as usize
}

impl StatesActions for RaceTrackMDP {
    type State = RaceTrackState;
    type Action = RaceTrackAction;
}

impl IsTerminal for RaceTrackMDP {
    fn is_terminal(&self, s: &Self::State) -> bool {
        match s {
            RaceTrackState::Dummy => false,
            RaceTrackState::Wrapper(inner) => self.goals.contains(&(inner.x, inner.y)),
        }
    }
}

impl ActionEnumerable for RaceTrackMDP {
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

fn add_outcome(outcomes: &mut Vec<(RaceTrackState, f32)>, outcome: RaceTrackState, outcome_p: f32) {
    let mut flag = true;
    for (s, p) in outcomes.iter_mut() {
        if *s == outcome {
            *p += outcome_p;
            flag = false;
        }
    }

    if flag {
        outcomes.push((outcome, outcome_p));
    }
}

fn add_outcome_64(
    outcomes: &mut Vec<(RaceTrackState, f64)>,
    outcome: RaceTrackState,
    outcome_p: f64,
) {
    let mut flag = true;
    for (s, p) in outcomes.iter_mut() {
        if *s == outcome {
            *p += outcome_p;
            flag = false;
        }
    }

    if flag {
        outcomes.push((outcome, outcome_p));
    }
}

impl Cost for RaceTrackMDP {
    fn cost(&self, s: &Self::State, _a: &Self::Action) -> f32 {
        match s {
            RaceTrackState::Dummy => 0.0,
            RaceTrackState::Wrapper(inner) => match self.get_status(inner.x, inner.y) {
                Wall => 10.0,
                Goal => 0.0,
                Pothole => 100.0,
                _ => 1.0,
            },
        }
    }
}

impl DCost for RaceTrackMDP {
    fn d_cost(&self, s: &Self::State, a: &Self::Action, _stt: &Self::State) -> f32 {
        self.cost(s, a)
    }
}

impl InitialState for RaceTrackMDP {
    fn initial_state(&self) -> Self::State {
        Dummy
    }
}

impl PMass<f32> for RaceTrackMDP {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        match s {
            Dummy => {
                let n = self.starts.len() as f32;
                self.starts
                    .iter()
                    .map(|(x, y)| (Wrapper(RaceTrackStateInner::new(*x, *y, 0, 0)), 1.0 / n))
                    .collect()
            }
            Wrapper(inner) => match self.get_status(inner.x, inner.y) {
                Goal => vec![(*s, 1.0)],
                Wall => {
                    let ddx = get_ddx(*a);
                    let ddy = get_ddy(*a);
                    vec![(
                        Wrapper(RaceTrackStateInner::new(
                            next(inner.x, ddx),
                            next(inner.y, ddy),
                            ddx,
                            ddy,
                        )),
                        1.0,
                    )]
                }
                Pothole => {
                    let ddx = get_ddx(*a);
                    let ddy = get_ddy(*a);
                    vec![(
                        Wrapper(RaceTrackStateInner::new(
                            next(inner.x, ddx),
                            next(inner.y, ddy),
                            ddx,
                            ddy,
                        )),
                        1.0,
                    )]
                }
                _ => {
                    let mut outcomes = vec![];
                    add_outcome(
                        &mut outcomes,
                        Wrapper(self.success(&inner, a)),
                        1.0 - self.p_slip as f32,
                    );
                    if self.p_slip > 0.0 {
                        add_outcome(
                            &mut outcomes,
                            Wrapper(self.success(&inner, &Keep)),
                            self.p_slip as f32,
                        );
                    }
                    outcomes
                }
            },
        }
    }
}

impl PMassMutFrom<f32> for RaceTrackMDP {}

impl PMass<f64> for RaceTrackMDP {
    type Distribution = Vec<(Self::State, f64)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f64)> {
        match s {
            Dummy => {
                let n = self.starts.len() as f64;
                self.starts
                    .iter()
                    .map(|(x, y)| (Wrapper(RaceTrackStateInner::new(*x, *y, 0, 0)), 1.0 / n))
                    .collect()
            }
            Wrapper(inner) => match self.get_status(inner.x, inner.y) {
                Goal => vec![(*s, 1.0)],
                Wall => {
                    let ddx = get_ddx(*a);
                    let ddy = get_ddy(*a);
                    vec![(
                        Wrapper(RaceTrackStateInner::new(
                            next(inner.x, ddx),
                            next(inner.y, ddy),
                            ddx,
                            ddy,
                        )),
                        1.0,
                    )]
                }
                Pothole => {
                    let ddx = get_ddx(*a);
                    let ddy = get_ddy(*a);
                    vec![(
                        Wrapper(RaceTrackStateInner::new(
                            next(inner.x, ddx),
                            next(inner.y, ddy),
                            ddx,
                            ddy,
                        )),
                        1.0,
                    )]
                }
                _ => {
                    let mut outcomes = vec![];
                    add_outcome_64(
                        &mut outcomes,
                        Wrapper(self.success(&inner, a)),
                        1.0_f64 - self.p_slip,
                    );
                    if self.p_slip > 0.0 {
                        add_outcome_64(
                            &mut outcomes,
                            Wrapper(self.success(&inner, &Keep)),
                            self.p_slip,
                        );
                    }
                    outcomes
                }
            },
        }
    }
}

impl PMassMut<f64> for RaceTrackMDP {
    type Distribution = Vec<(Self::State, f64)>;
    fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f64)> {
        PMass::<f64>::p_mass(self, s, a)
    }
}

impl ActionAvailability for RaceTrackMDP {
    fn action_available(&self, s: &Self::State, a: &Self::Action) -> bool {
        match s {
            Dummy => true,
            Wrapper(inner) => {
                //                 let dx = inner.dx + get_ddx(*a);
                //                 let dy = inner.dy + get_ddy(*a);
                let dx = get_ddx(*a);
                let dy = get_ddy(*a);
                let x = inner.x as i32 + dx;
                let y = inner.y as i32 + dy;
                if x < 0 || x >= self.w as i32 || y < 0 || y >= self.h as i32 {
                    return false;
                }

                if self.get_status(inner.x, inner.y) == Wall
                    && self.get_status(x as usize, y as usize) == Wall
                {
                    return false;
                }

                if self.get_status(inner.x, inner.y) == Pothole
                    && self.get_status(x as usize, y as usize) == Pothole
                {
                    return false;
                }

                return true;
            }
        }
    }
}

impl ExplicitTransition for RaceTrackMDP {}

impl ExplicitTransitionMutFrom for RaceTrackMDP {}

impl PreferredSuccessor for RaceTrackMDP {
    fn preferred_successor(&self, s: &Self::State, a: &Self::Action) -> Self::State {
        match s {
            Dummy => {
                assert!(self.starts.len() == 1);
                let coord = self.starts.iter().nth(0).unwrap();
                Wrapper(RaceTrackStateInner::new(coord.0, coord.1, 0, 0))
            }
            Wrapper(inner) => match self.get_status(inner.x, inner.y) {
                Goal => *s,
                Wall => {
                    let ddx = get_ddx(*a);
                    let ddy = get_ddy(*a);
                    Wrapper(RaceTrackStateInner::new(
                        next(inner.x, ddx),
                        next(inner.y, ddy),
                        ddx,
                        ddy,
                    ))
                }
                Pothole => {
                    let ddx = get_ddx(*a);
                    let ddy = get_ddy(*a);
                    Wrapper(RaceTrackStateInner::new(
                        next(inner.x, ddx),
                        next(inner.y, ddy),
                        ddx,
                        ddy,
                    ))
                }
                _ => Wrapper(self.success(&inner, a)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_race_track_success() {
        init();
        let mdp = RaceTrackMDP::from_file("data/tracks/small.track");
        assert_eq!(
            mdp.success(&RaceTrackStateInner::new(4, 2, 0, 1), &Keep),
            RaceTrackStateInner::new(4, 3, 0, 1)
        );
        assert_eq!(
            mdp.success(&RaceTrackStateInner::new(4, 2, 0, 2), &Keep),
            RaceTrackStateInner::new(4, 4, 0, 2)
        );
        assert_eq!(
            mdp.success(&RaceTrackStateInner::new(4, 2, 2, 2), &Keep),
            RaceTrackStateInner::new(6, 4, 0, 0)
        );
    }

    #[test]
    fn test_race_track_p_mass() {
        init();
        let mdp = RaceTrackMDP::from_file("data/tracks/small.track").set_p_slip(0.1);
        assert_eq!(
            PMass::<f32>::p_mass(&mdp, &Dummy, &Keep),
            vec![(Wrapper(RaceTrackStateInner::new(4, 2, 0, 0)), 1.0)]
        );
        assert_eq!(
            PMass::<f32>::p_mass(&mdp, &Wrapper(RaceTrackStateInner::new(4, 2, 0, 0)), &North),
            vec![
                (Wrapper(RaceTrackStateInner::new(4, 3, 0, 1)), 0.9),
                (Wrapper(RaceTrackStateInner::new(4, 2, 0, 0)), 0.1)
            ]
        );
        assert_eq!(
            PMass::<f32>::p_mass(
                &mdp,
                &Wrapper(RaceTrackStateInner::new(4, 8, 0, 0)),
                &NorthWest
            ),
            vec![
                (Wrapper(RaceTrackStateInner::new(4, 9, -0, 0)), 0.9),
                (Wrapper(RaceTrackStateInner::new(4, 8, 0, 0)), 0.1)
            ]
        );
    }

    #[test]
    fn test_race_track_cost() {
        init();
        let mdp = RaceTrackMDP::from_file("data/tracks/small.track");
        assert_eq!(
            mdp.cost(&Wrapper(RaceTrackStateInner::new(4, 2, 0, 1)), &Keep),
            1.0
        );
        assert_eq!(
            mdp.cost(&Wrapper(RaceTrackStateInner::new(4, 2, 2, 2)), &Keep),
            1.0
        );
        assert_eq!(
            mdp.cost(&Wrapper(RaceTrackStateInner::new(6, 4, 0, 0)), &West),
            10.0
        );
    }

    #[test]
    fn test_race_track_from_file_small() {
        let mdp = RaceTrackMDP::from_file("data/tracks/small.track");
        assert_eq!(mdp.h, 12);
        assert_eq!(mdp.w, 8);
    }

    #[test]
    fn test_race_track_from_file_lexi() {
        let mdp = RaceTrackMDP::from_file("data/tracks/lexi/lexi1.track");
        assert_eq!(mdp.h, 12);
        assert_eq!(mdp.w, 11);
    }
}
