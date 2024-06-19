use std::slice::Iter;

use super::StatesActions;

pub trait ActionEnumerable: StatesActions {
    fn enumerate_actions(&self) -> Iter<Self::Action>;
    fn num_actions(&self) -> usize;
    fn id_to_action(&self, id: usize) -> &Self::Action;
}
