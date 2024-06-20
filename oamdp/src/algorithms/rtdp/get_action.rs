use mdp::{
    heuristic::HeuristicWithMDPMut,
    into_inner::Inner,
    mdp_traits::{ActionEnumerable, Cost, PMassMut, StatesActions},
    policy::policy_traits::GetActionMut,
};
use rand::seq::SliceRandom;
use std::fmt::Debug;
use std::hash::Hash;

use crate::{
    oamdp::{oamdp::OAMDP, BeliefState},
    oamdp_d::{VState, OAMDPD},
    traits::BeliefOverGoal,
};

use super::RTDP_OAMDP;

impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
    GetActionMut<BeliefState<M::State, N>, OAMDP<OM, M, A, N>> for RTDP_OAMDP<OM, M, A, H, N>
where
    OAMDPD<OM, M, A, N>: StatesActions<State = VState<M::State, N>, Action = A>
        + PMassMut<f32>
        + Cost
        + ActionEnumerable,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>, Action = A>,
    H: HeuristicWithMDPMut<OAMDPD<OM, M, A, N>>,
{
    fn get_action_mut(
        &mut self,
        bs: &BeliefState<M::State, N>,
        _oamdp: &mut OAMDP<OM, M, A, N>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<A> {
        let pairs = self
            .oamdp_d
            .translator
            .get_corner_and_lambdas(&bs.get_belief_over_goal());

        //         for (v, _w) in &pairs {
        //             if !self.rtdp.is_solved.contains(&VState::new(bs.inner(), *v)) {
        //                 panic!("{:?} is not labeld", VState::new(bs.inner(), *v));
        //                 //                 println!("Not solved: {:?}", VState::new(bs.inner(), *v));
        //             }
        //         }

        if let Ok(pair) = pairs.choose_weighted(rng, |(_v, w)| *w) {
            let vs = VState::new(bs.inner(), pair.0);

            self.rtdp.get_action_mut(&vs, &mut self.oamdp_d, rng)
        } else {
            panic!("{:?}", pairs);
        }
    }
}

// impl<OM, M: StatesActions, A: PartialEq + Eq + Copy + Clone + Debug + Hash, H, const N: usize>
//     GetActionMut<BeliefState<M::State, N>, OAMDP<OM, M, A, N>> for RTDP_OAMDP<OM, M, A, H, N>
// where
//     OAMDPD<OM, M, A, N>: StatesActions<State = VState<M::State, N>, Action = A>
//         + PMassMut<f32>
//         + Cost
//         + ActionEnumerable,
//     OAMDP<OM, M, A, N>:
//         StatesActions<State = BeliefState<M::State, N>, Action = A> + PMassMut<f32> + Cost,
//     H: HeuristicWithMDPMut<OAMDPD<OM, M, A, N>>,
// {
//     fn get_action_mut(
//         &mut self,
//         bs: &BeliefState<M::State, N>,
//         _oamdp: &mut OAMDP<OM, M, A, N>,
//         _rng: &mut rand::rngs::ThreadRng,
//     ) -> Option<A> {
//         let mut best_a = None;
//         let mut best_value = 1e+8;
//         //         println!("{:?}", bs);
//
//         for a_id in 0..self.oamdp_d.num_actions() {
//             let a = *self.oamdp_d.id_to_action(a_id);
//             let qsa = self.get_qsa_ssp_mut(bs, &a);
//             //             println!("{:?} {:?}", a, qsa);
//             if qsa < best_value {
//                 best_a = Some(a);
//                 best_value = qsa;
//             }
//         }
//         best_a
//     }
// }
//
