use crate::common::value_table::ValueTable;
use crate::mdp_traits::*;
use crate::value_estimator::CostEstimator;

pub fn value_iteration_ssp_n_step<
    M: ActionAvailability + StateEnumerable + ActionEnumerable + PMass<f32> + Cost,
>(
    mdp: &M,
    horizon: usize,
) -> ValueTable<M::State> {
    let mut even_value_table = ValueTable::<M::State>::new(0.0);
    let mut odd_value_table = ValueTable::<M::State>::new(0.0);
    for h in 1..horizon {
        for s in mdp.enumerate_states() {
            if h % 2 == 0 {
                let min_qsa = get_min_qsa(s, &mut odd_value_table, mdp);
                even_value_table.set_value(s, min_qsa);
            } else {
                let min_qsa = get_min_qsa(s, &mut even_value_table, mdp);
                odd_value_table.set_value(s, min_qsa);
            }
        }
    }

    if horizon % 2 == 0 {
        even_value_table
    } else {
        odd_value_table
    }
}

fn get_min_qsa<M: ActionAvailability + ActionEnumerable + PMass<f32> + Cost>(
    s: &M::State,
    vt: &mut ValueTable<M::State>,
    mdp: &M,
) -> f32 {
    mdp.enumerate_actions()
        .filter(|a| mdp.action_available(s, a))
        .map(|a| vt.get_qsa_ssp(s, a, mdp))
        .fold(1. / 0., f32::min)
}
