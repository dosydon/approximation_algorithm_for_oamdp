use crate::grid_world::grid_status::GridStatus;
use crate::grid_world::GridWorldAction::*;
use crate::grid_world::GridWorldMDP;
use crate::grid_world::GridWorldState;
use itertools::iproduct;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

impl GridWorldMDP {
    pub fn from_file(filename: &str) -> GridWorldMDP {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let wl = lines.next().unwrap().unwrap();
        let w = wl.trim().parse::<i64>().unwrap();
        let h = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
        let mut grids = vec![];

        for line in lines {
            let mut row = vec![];
            if let Ok(ip) = line {
                //                 println!("{}", ip);
                for c in ip.chars() {
                    let s = match c {
                        'x' => GridStatus::Wall,
                        'G' => GridStatus::Goal,
                        'S' => GridStatus::Start,
                        '.' => GridStatus::Blank,
                        '@' => GridStatus::Watery,
                        x => panic!("unexpected character{}", x),
                    };
                    row.push(s);
                }
            }
            //             println!("{}", row.len());
            grids.push(row);
        }

        let mut initial_state = GridWorldState::new(0, 0);
        for (y, row) in grids.iter().enumerate() {
            for (x, s) in row.iter().enumerate() {
                match s {
                    GridStatus::Start => initial_state = GridWorldState::new(x as i64, y as i64),
                    _ => (),
                }
            }
        }

        let mut grid = GridWorldMDP {
            h: h,
            w: w,
            initial_state: initial_state,
            all_states: vec![],
            all_actions: [AttemptUp, AttemptRight, AttemptDown, AttemptLeft],
            grids: grids,
        };
        grid.all_states = iproduct!((0..w), (0..h))
            .filter(|(x, y)| grid.is_valid_cordinate(*x, *y))
            .map(|(x, y)| GridWorldState { x, y })
            .collect();
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grid_world_from_file() {
        let mdp = GridWorldMDP::from_file("data/gws/map1.gw");
        assert_eq!(mdp.h, 10);
        assert_eq!(mdp.w, 5);
    }

    #[test]
    fn test_grid_world_from_file_map10() {
        let mdp = GridWorldMDP::from_file("data/gws/map10.gw");
        assert_eq!(mdp.h, 11);
        assert_eq!(mdp.w, 70);
    }
}
