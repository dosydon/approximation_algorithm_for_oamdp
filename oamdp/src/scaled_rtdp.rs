use std::fmt::Debug;
use std::hash::Hash;

use mdp::heuristic::{HeuristicWithMDP, HeuristicWithMDPMut};
use mdp::into_inner::Inner;
use mdp::mdp_traits::{Cost, PMassMut, StatesActions};

use rtdp::rtdp::RTDP;

use crate::oamdp_d::{VState, OAMDPD};
pub struct ScaledRTDP<S: Eq + PartialEq + Debug + Copy + Clone + Hash, H> {
    alpha: f32,
    rtdp: RTDP<S, H>,
}

impl<S: Eq + PartialEq + Debug + Clone + Hash + Copy, H> ScaledRTDP<S, H> {
    pub fn new(alpha: f32, vt: RTDP<S, H>) -> ScaledRTDP<S, H> {
        ScaledRTDP {
            alpha: alpha,
            rtdp: vt,
        }
    }
}

impl<S: Eq + PartialEq + Debug + Clone + Hash + Copy, H, M: StatesActions> HeuristicWithMDP<M>
    for ScaledRTDP<S, H>
where
    M::State: Inner<Result = S>,
{
    fn h_with(&self, s: &<M as StatesActions>::State, _mdp: &M) -> f32 {
        self.alpha * self.rtdp.vt.get_value(&s.inner())
    }
}

impl<OM, M, A: Eq + PartialEq + Debug + Clone + Hash + Copy, H, const N: usize>
    HeuristicWithMDPMut<OAMDPD<OM, M, A, N>> for ScaledRTDP<M::State, H>
where
    M: StatesActions + PMassMut<f32> + Cost,
    H: HeuristicWithMDPMut<M>,
{
    fn h_with_mut(&mut self, s: &VState<M::State, N>, mdp: &mut OAMDPD<OM, M, A, N>) -> f32 {
        self.alpha * self.rtdp.get_value_ssp_mut(&s.inner(), &mut mdp.oamdp.mdp)
    }
}
