use crate::aostar::AOStar;
use mdp::heuristic::HeuristicWithMDPMut;
use mdp::mdp_traits::{ActionEnumerable, StatesActions};
use std::collections::HashMap;

impl<M: StatesActions + ActionEnumerable, H: HeuristicWithMDPMut<M>> AOStar<M, H> {
    pub fn to_value_table(&self) -> HashMap<M::State, f32> {
        let mut table = HashMap::new();
        for node in self.arena.nodes.iter() {
            table.insert(node.s, node.f);
        }
        table
    }
}
