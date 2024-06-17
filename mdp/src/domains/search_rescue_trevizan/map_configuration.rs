
use crate::domains::search_rescue_trevizan::cell_status::CellStatus;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use rand::prelude::*;
use super::cell_status::CellStatus::*;

#[serde_as]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct MapConfiguration<const N: usize> {
    #[serde_as(as = "[[_; N]; N]")]
    pub cells: [[CellStatus; N]; N],
    pub agent_pos: (i32, i32),
}

fn hamming_distance(a: (i32, i32), b: (i32, i32)) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

impl<const N: usize> MapConfiguration<N> {
    pub fn random_instance(d: usize, density_r: f32, rng: &mut ThreadRng) -> Self {
        let starting_point = Self::random_position(rng);
        let points_at_d = Self::enumerate_points_hamming_distance(d, starting_point);
        let survivor_point = points_at_d.choose(rng).unwrap();
        let mut cells = [[NoSurvivor; N]; N];
        cells[survivor_point.0 as usize][survivor_point.1 as usize] = Survivor;
        
        let mut uncertain_points = 0.0;
        while uncertain_points <= ((N * N) as f32) * density_r {
            let random_point = Self::random_position(rng);
            match cells[random_point.0 as usize][random_point.1 as usize] {
                Survivor => (),
                NoSurvivor => {
                    cells[random_point.0 as usize][random_point.1 as usize] = *[ProbLow, ProbMedium, ProbHigh].choose(rng).unwrap();
                    uncertain_points += 1.0;
                },
                _ => (),
            }
        }
        
        MapConfiguration::<N> {
            cells: cells,
            agent_pos: starting_point
        }
    }
    pub fn random_position(rng: &mut ThreadRng) -> (i32, i32) {
        let i = rng.gen_range(0, N);
        let j = rng.gen_range(0, N);
        (i as i32, j as i32)
    }
    
    fn enumerate_points_hamming_distance(d: usize, p: (i32, i32)) -> Vec<(i32, i32)> {
        let mut results = vec![];
        for i in 0..N {
            for j in 0..N {
                if hamming_distance(p, (i as i32, j as i32)) == d {
                    results.push((i as i32, j as i32));
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::MapConfiguration;
    use rand::thread_rng;

    #[test]
    fn test_random_position() {
        let mut rng = thread_rng();
        for _ in 0..10 {
            let pos = MapConfiguration::<4>::random_position(&mut rng);
            assert!(pos.0 >= 0);
            assert!(pos.0 < 4);
            assert!(pos.1 >= 0);
            assert!(pos.1 < 4);
        }
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance((0, 1), (1, 0)), 2);
        assert_eq!(hamming_distance((0, 0), (1, 0)), 1);
        assert_eq!(hamming_distance((3, 2), (1, 0)), 4);
    }

    #[test]
    fn test_enumerate_points_hamming_distance() {
        let points_1 = MapConfiguration::<4>::enumerate_points_hamming_distance(1, (0, 0));
//         println!("{:?}", points_1);
        assert_eq!(points_1.len(), 2);

        let points_2 = MapConfiguration::<4>::enumerate_points_hamming_distance(2, (0, 0));
//         println!("{:?}", points_2);
        assert_eq!(points_2.len(), 3);
    }

    #[test]
    fn test_random_instance() {
        let mut rng = thread_rng();
        let instance = MapConfiguration::<4>::random_instance(2, 0.5, &mut rng);
        println!("{:?}", instance);
    }
}