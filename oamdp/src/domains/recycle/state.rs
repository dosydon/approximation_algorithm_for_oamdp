use super::location::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecycleState<const L: usize> {
    pub(crate) locs: [Location; L],
}

impl<const L: usize> RecycleState<L> {
    pub fn new(locs: [Location; L]) -> Self {
        Self { locs: locs }
    }

    pub(crate) fn change(&self, i: usize, loc: Location) -> Self {
        let mut new_locs = self.locs.clone();
        new_locs[i] = loc;
        Self { locs: new_locs }
    }
}
