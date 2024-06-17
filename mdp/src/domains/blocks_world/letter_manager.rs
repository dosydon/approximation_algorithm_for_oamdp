use itertools::Itertools;

use super::{
    display::get_heights,
    Block,
    Location::{self, *},
};

pub struct LetterManager<const N: usize> {
    pub letters: [char; N],
}

impl<const N: usize> LetterManager<N> {
    pub fn new(letters: [char; N]) -> LetterManager<N> {
        LetterManager { letters: letters }
    }

    pub fn id_to_letter(&self, id: usize) -> char {
        self.letters[id]
    }

    pub fn letter_to_block(&self, letter: char) -> Block {
        Block::new(self.letter_to_id(letter))
    }

    fn letter_to_id(&self, letter: char) -> usize {
        self.letters
            .iter()
            .position(|&l| l == letter)
            .expect("Letter not found")
    }

    pub fn letters_to_goal(&self, letters: [char; N]) -> [Location; N] {
        let mut goal = [OnTable; N];
        for i in 0..(N - 1) {
            goal[self.letter_to_id(letters[i])] = On(self.letter_to_block(letters[i + 1]));
        }
        goal
    }

    pub fn str_to_locations(&self, letters: &str) -> [Location; N] {
        let mut loc = [OnTable; N];
        for part in letters.split(' ') {
            for (c1, c2) in part.chars().zip(part.chars().skip(1)) {
                loc[self.letter_to_id(c1)] = On(self.letter_to_block(c2));
            }
        }
        loc
    }

    pub fn goal_to_string(&self, goal: &[Location; N]) -> String {
        let heights = get_heights(&goal);
        (0..N)
            .sorted_by_key(|i| heights[*i])
            .rev()
            .map(|i| self.letters[i])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks_world_letter_manager() {
        let lm = LetterManager::new(['A', 'M', 'S', 'R']);

        assert_eq!(lm.letter_to_block('A'), Block::new(0));
        assert_eq!(
            lm.str_to_locations("A MS R"),
            [OnTable, On(Block::new(2)), OnTable, OnTable]
        );
        assert_eq!(
            lm.str_to_locations("RAMS"),
            [
                On(Block::new(1)),
                On(Block::new(2)),
                OnTable,
                On(Block::new(0))
            ]
        );
        println!("{}", lm.goal_to_string(&lm.str_to_locations("RAMS")));
    }
}
