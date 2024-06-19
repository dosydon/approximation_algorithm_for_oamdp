use core::fmt::Debug;

pub struct Arena<V: Debug> {
    pub nodes: Vec<V>,
}

impl<V: Debug> Arena<V> {
    pub fn new() -> Arena<V> {
        Arena { nodes: vec![] }
    }
    pub fn add_node(&mut self, v: V) -> usize {
        //         println!("{:?}", v);
        let new_id = self.nodes.len();
        self.nodes.push(v);
        new_id
    }
    pub fn get_node(&self, id: usize) -> &V {
        &(self.nodes[id])
    }
    pub fn get_node_mut(&mut self, id: usize) -> &mut V {
        &mut (self.nodes[id])
    }
    pub fn next_id(&self) -> usize {
        self.nodes.len()
    }
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}
