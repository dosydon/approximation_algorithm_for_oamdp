use mdp::mdp_traits::StatesActions;

pub(crate) trait UpperBoundWithMDPMut<M: StatesActions> {
    fn u_with_mut(&mut self, s: &M::State, mdp: &mut M) -> f32;
}
