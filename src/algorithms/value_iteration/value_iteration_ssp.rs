use crate::common::value_table::ValueTable;
use crate::mdp_traits::*;
use crate::value_estimator::CostEstimator;

pub fn value_iteration_ssp<
    M: ActionAvailability + ActionEnumerable + StateEnumerable + PMass<f32> + Cost + IsTerminal,
>(
    mdp: &M,
) -> ValueTable<M::State> {
    let err = 1e-9;
    let mut value_table = ValueTable::<M::State>::new(1e+8);
    loop {
        let residual = update_ssp(&mut value_table, mdp, &set_min_qsa);
        //         println!("{:?}", residual);
        if residual < err {
            break;
        }
    }
    value_table
}

pub fn soft_value_iteration_ssp<
    M: ActionAvailability + ActionEnumerable + StateEnumerable + PMass<f32> + Cost + IsTerminal,
>(
    mdp: &M,
) -> ValueTable<M::State> {
    let err = 1e-7;
    let mut value_table = ValueTable::<M::State>::new(1000.0);
    loop {
        let residual = update_ssp(&mut value_table, mdp, &set_soft_min_qsa);
        if residual < err {
            break;
        }
    }
    value_table
}

fn update_ssp<M: ActionEnumerable + StateEnumerable + PMass<f32> + Cost + IsTerminal>(
    vt: &mut ValueTable<M::State>,
    mdp: &M,
    f: &dyn Fn(&M::State, &mut ValueTable<M::State>, &M) -> f32,
) -> f32 {
    let mut max_residual = 0.0;
    for s in mdp.enumerate_states() {
        if mdp.is_terminal(s) {
            let residual = (vt.get_value(s) - 0.0).abs();
            if residual > max_residual {
                max_residual = residual;
            }
            vt.set_value(s, 0.0);
            continue;
        }

        let residual = f(s, vt, mdp);

        if residual > max_residual {
            max_residual = residual;
        }
    }

    max_residual
}

fn set_min_qsa<M: ActionAvailability + ActionEnumerable + PMass<f32> + Cost>(
    s: &M::State,
    vt: &mut ValueTable<M::State>,
    mdp: &M,
) -> f32 {
    let min_qsa = vt.get_value_ssp(s, mdp);

    if vt.get_value(s) > min_qsa {
        let residual = (min_qsa - vt.get_value(s)).abs();
        vt.set_value(s, min_qsa);
        residual
    } else {
        0.0
    }
}

fn set_soft_min_qsa<
    M: ActionAvailability + ActionEnumerable + StateEnumerable + PMass<f32> + Cost,
>(
    s: &M::State,
    vt: &mut ValueTable<M::State>,
    mdp: &M,
) -> f32 {
    let k = -1.0;
    let min_qsa = vt.get_value_ssp(s, mdp);
    let logsumexp = mdp
        .enumerate_actions()
        .map(|a| (k * (vt.get_qsa_ssp(s, a, mdp) - min_qsa)).exp())
        .sum::<f32>()
        .ln();

    let soft_min_qsa = k * logsumexp + min_qsa;
    let residual = (soft_min_qsa - vt.get_value(s)).abs();
    vt.set_value(s, soft_min_qsa);
    println!("{:?} {:?} {:?} {:?}", s, logsumexp, soft_min_qsa, residual);

    residual
}
