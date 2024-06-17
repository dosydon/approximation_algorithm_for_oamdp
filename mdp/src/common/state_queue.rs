use std::collections::HashSet;
use std::hash::Hash;
use std::iter::IntoIterator;

pub struct StateQueue<S: Hash + PartialEq + Eq + Copy> {
    vec: Vec<S>,
    set: HashSet<S>,
}

impl<S: Hash + PartialEq + Eq + Copy> StateQueue<S> {
    pub fn new() -> Self {
        StateQueue {
            vec: Vec::new(),
            set: HashSet::new(),
        }
    }

    pub fn push(&mut self, s: S) {
        self.set.insert(s);
        self.vec.push(s);
    }

    pub fn pop(&mut self) -> Option<S> {
        self.vec.pop()
    }

    pub fn contains(&self, s: &S) -> bool {
        self.set.contains(s)
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl<S: Hash + PartialEq + Eq + Copy> IntoIterator for StateQueue<S> {
    type Item = S;
    type IntoIter = std::vec::IntoIter<S>;
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}
