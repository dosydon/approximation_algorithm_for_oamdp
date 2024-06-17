use super::letter::Letter;
use crate::{common::coordinate2::Coordinate2, mdp_traits::ToVarName};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct SpellingState<const N: usize> {
    pub coord: Coordinate2,
    pub(crate) letters: [Letter; N],
}

impl<const N: usize> SpellingState<N> {
    pub fn new(coord: Coordinate2, letters: [Letter; N]) -> Self {
        SpellingState {
            coord: coord,
            letters: letters,
        }
    }
}

impl ToVarName for SpellingState<4> {
    fn to_var_name(&self) -> String {
        format!(
            "SpellingState_{}_{}_{}_{}_{}_{}",
            self.coord.i,
            self.coord.j,
            self.letters[0].to_char(),
            self.letters[1].to_char(),
            self.letters[2].to_char(),
            self.letters[3].to_char()
        )
    }
}
