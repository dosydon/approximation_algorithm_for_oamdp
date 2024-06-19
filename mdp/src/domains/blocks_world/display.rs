use super::BlocksWorldMDPN;
use super::BlocksWorldStateN;
use super::Location;
use crate::blocks_world::location::Location::*;

pub(crate) fn get_heights<const N: usize>(locations: &[Location; N]) -> [usize; N] {
    let mut heights = [0; N];
    for _ in 0..N {
        for i in 0..N {
            match locations[i] {
                OnTable => {
                    heights[i] = 0;
                }
                On(b) => {
                    heights[i] = heights[b.id] + 1;
                }
                OnHold => {
                    heights[i] = N;
                }
            }
        }
    }
    heights
}

fn get_columns<const N: usize>(locations: &[Location; N]) -> [usize; N] {
    let mut columns = [0; N];
    let mut is_column_set = [false; N];
    let mut column_id = 0;
    for _ in 0..N {
        for i in 0..N {
            if is_column_set[i] {
                continue;
            }
            match locations[i] {
                OnTable => {
                    columns[i] = column_id;
                    column_id += 1;
                    is_column_set[i] = true;
                }
                On(b) => {
                    if is_column_set[b.id] {
                        columns[i] = columns[b.id];
                        is_column_set[i] = true;
                    }
                }
                OnHold => {
                    columns[i] = N;
                    is_column_set[i] = true;
                }
            }
        }
    }
    columns
}

impl<const N: usize> BlocksWorldMDPN<N> {
    pub fn display(&self, s: &BlocksWorldStateN<N>) {
        let heights = get_heights(&s.locations);
        let columns = get_columns(&s.locations);
        for i in 0..=N {
            for j in 0..=N {
                let mut printed = false;
                for k in 0..N {
                    if heights[k] == (N - i) && columns[k] == j {
                        print!("{}", self.letters[k]);
                        printed = true;
                    }
                }
                if !printed {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks_world::block::Block;

    #[test]
    fn test_get_heights() {
        let b0 = Block::new(0);
        let b1 = Block::new(1);
        let b2 = Block::new(2);
        let s = BlocksWorldStateN::new([OnTable, On(b0), On(b1), On(b2)]);
        println!("{:?}", get_heights(&s.locations));
        assert_eq!(get_heights(&s.locations), [0, 1, 2, 3]);

        let ss = BlocksWorldStateN::new([OnTable, On(b0), OnTable, On(b2)]);
        assert_eq!(get_heights(&ss.locations), [0, 1, 0, 1]);
        println!("{:?}", get_heights(&ss.locations));
    }

    #[test]
    fn test_get_columns() {
        let b0 = Block::new(0);
        let b1 = Block::new(1);
        let b2 = Block::new(2);
        let s = BlocksWorldStateN::new([OnTable, On(b0), On(b1), On(b2)]);
        assert_eq!(get_columns(&s.locations), [0, 0, 0, 0]);

        let ss = BlocksWorldStateN::new([OnTable, On(b0), OnTable, On(b2)]);
        assert_eq!(get_columns(&ss.locations), [0, 0, 1, 1]);

        let s = BlocksWorldStateN::new([OnTable, On(b2), OnTable, OnHold]);
        println!("{:?}", get_columns(&s.locations));
    }

    #[test]
    fn test_display() {
        let b0 = Block::new(0);
        let b1 = Block::new(1);
        let b2 = Block::new(2);
        let _b3 = Block::new(2);
        let mdp = BlocksWorldMDPN::new(
            [OnTable, OnTable, OnTable, OnTable],
            [OnTable, OnTable, OnTable, OnTable],
            0.0,
            ['A', 'M', 'S', 'R'],
        );

        let s = BlocksWorldStateN::new([OnTable, On(b0), On(b1), On(b2)]);
        mdp.display(&s);
        let ss = BlocksWorldStateN::new([OnTable, On(b0), OnTable, On(b2)]);
        mdp.display(&ss);

        let sss = BlocksWorldStateN::new([OnTable, On(b0), OnTable, OnHold]);
        mdp.display(&sss);
    }
}
