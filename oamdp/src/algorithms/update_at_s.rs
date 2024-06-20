use mdp::mdp_traits::*;
use std::f32::MAX;

use std::fmt::Debug;
use std::hash::Hash;

use crate::algorithms::grid_value_function_ssp::GridValueFunctionSSP;
use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;

use crate::algorithms::assoc_belief_point::AssocBeliefPointN;
use crate::algorithms::regular_grid_belief_points::RegularGridBeliefPoints;

pub(crate) fn update_at_s<OM, M, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>(
    oamdp: &mut OAMDP<OM, M, A, N>,
    s: &M::State,
    vf: &mut GridValueFunctionSSP<M::State, AssocBeliefPointN<A, N>, N>,
) -> f32
where
    M: IsTerminal,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>, Action = A>
        + PMassMut<f32>
        + Cost
        + ActionEnumerable,
{
    let mut delta = 0.0;
    if oamdp.mdp.is_terminal(&s) {
        return 0.0;
    }
    unsafe {
        let grbp = vf.table.get_mut(&s).unwrap()
            as *mut RegularGridBeliefPoints<AssocBeliefPointN<A, N>, N>;

        for b in (*grbp).grid.values_mut() {
            //                 println!("s: {:?} b {:?} v:{:?}", s, b.b, b.v);
            let mut best_future_value = MAX;
            let mut best_action = None;

            for a_id in 0..oamdp.num_actions() {
                let a = *oamdp.id_to_action(a_id);
                let qsa = vf.qsa_ssp_mut(s, &b.b, &a, oamdp);

                //                     println!("s: {:?} b {:?} v:{:?} qsa:{:?}", s, b.b, b.v, qsa);
                if qsa < best_future_value {
                    best_future_value = qsa;
                    best_action = Some(a);
                }
            }

            let residual = (*grbp).update_value(&b.b, best_future_value, best_action);
            if residual > delta {
                delta = residual;
            }
        }
    }
    delta
}
