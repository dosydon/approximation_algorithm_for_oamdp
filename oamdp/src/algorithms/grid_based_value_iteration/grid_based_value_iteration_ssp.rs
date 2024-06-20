use mdp::mdp_traits::*;
use num_traits::Float;

use std::hash::Hash;
use std::{collections::HashMap, fmt::Debug};

use crate::algorithms::grid_value_function_ssp::GridValueFunctionSSP;
use crate::algorithms::update_at_s::update_at_s;
use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;

use crate::algorithms::assoc_belief_point::AssocBeliefPointN;
use crate::algorithms::regular_grid_belief_points::RegularGridBeliefPoints;

pub fn grid_based_value_iteration_ssp<
    OM,
    M,
    A: Eq + PartialEq + Hash + Debug + Clone + Copy,
    const N: usize,
>(
    oamdp: &mut OAMDP<OM, M, A, N>,
    n_bin_per_dim: usize,
) -> GridValueFunctionSSP<M::State, AssocBeliefPointN<A, N>, N>
where
    M: StateEnumerable + IsTerminal,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>, Action = A>
        + PMassMut<f32>
        + Cost
        + ActionEnumerable,
{
    let mut table = HashMap::new();
    for s in oamdp.mdp.enumerate_states() {
        let grbp = RegularGridBeliefPoints::<AssocBeliefPointN<A, N>, N>::generate_uniform_grid(
            n_bin_per_dim,
        );
        table.insert(*s, grbp);
    }
    let mut vf = GridValueFunctionSSP::new(table);
    for t in 0..100000 {
        println!("starting {}th", t);
        let residual = one_iteration(oamdp, &mut vf);
        println!("iteration {} residual {}", t, residual);
        if residual < 0.001 {
            break;
        }
    }
    vf
}

fn one_iteration<OM, M, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>(
    oamdp: &mut OAMDP<OM, M, A, N>,
    vf: &mut GridValueFunctionSSP<M::State, AssocBeliefPointN<A, N>, N>,
) -> f32
where
    M: StateEnumerable + IsTerminal,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>, Action = A>
        + PMassMut<f32>
        + Cost
        + ActionEnumerable,
{
    let mut delta = 0.0;
    for s_id in 0..oamdp.mdp.num_states() {
        let s = *oamdp.mdp.id_to_state(s_id);
        delta = delta.max(update_at_s(oamdp, &s, vf));
    }
    delta
}
