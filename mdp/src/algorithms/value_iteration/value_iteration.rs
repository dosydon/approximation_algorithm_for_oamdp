pub use crate::common::value_table::ValueTable;
use crate::mdp_traits::*;
use crate::value_estimator::ValueEstimator;
use core::f32::MIN;

pub fn value_iteration<
    M: ActionEnumerable + StateEnumerable + PMass<f32> + Rsa + DiscountFactor + IsTerminal,
>(
    mdp: &M,
) -> ValueTable<M::State> {
    let err = 1e-7;
    let mut value_table = ValueTable::<M::State>::new(MIN);
    loop {
        let residual = update(&mut value_table, mdp, &set_max_qsa);
        //         println!("{}", residual);
        if residual < err {
            break;
        }
    }
    value_table
}

pub fn soft_value_iteration<
    M: ActionEnumerable + PMass<f32> + Rsa + DiscountFactor + StateEnumerable,
>(
    mdp: &M,
    beta: f32,
) -> ValueTable<M::State> {
    let err = 1e-7;
    let mut value_table = ValueTable::<M::State>::new(0.0);
    loop {
        let residual = update_soft(&mut value_table, mdp, beta);
        if residual < err {
            break;
        }
    }
    value_table
}

fn set_max_qsa<M: ActionEnumerable + StateEnumerable + PMass<f32> + Rsa + DiscountFactor>(
    s: &M::State,
    vt: &mut ValueTable<M::State>,
    mdp: &M,
) -> f32 {
    let max_qsa = vt.get_max_qsa(s, mdp);
    if vt.get_value(s) < max_qsa {
        let residual = (max_qsa - vt.get_value(s)).abs();
        vt.set_value(s, max_qsa);
        residual
    } else {
        0.0
    }
}

fn update<M: ActionEnumerable + StateEnumerable + PMass<f32> + Rsa + IsTerminal>(
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

fn update_soft<M: ActionEnumerable + StateEnumerable + PMass<f32> + Rsa + DiscountFactor>(
    vt: &mut ValueTable<M::State>,
    mdp: &M,
    alpha: f32,
) -> f32 {
    let mut max_residual = 0.0;
    for s in mdp.enumerate_states() {
        let residual = set_soft_max_qsa(s, vt, mdp, alpha);
        if residual > max_residual {
            max_residual = residual;
        }
    }

    max_residual
}

fn set_soft_max_qsa<M: ActionEnumerable + StateEnumerable + PMass<f32> + Rsa + DiscountFactor>(
    s: &M::State,
    vt: &mut ValueTable<M::State>,
    mdp: &M,
    alpha: f32,
) -> f32 {
    let max_qsa = vt.get_max_qsa(s, mdp);
    let logsumexp = mdp
        .enumerate_actions()
        .map(|a| ((vt.get_qsa(s, a, mdp) - max_qsa) / alpha).exp())
        .sum::<f32>()
        .ln();
    let soft_max_qsa = alpha * logsumexp + max_qsa;
    //     if mdp.is_terminal(s) {
    //         println!("{:?} {:?} {:?} {:?}", s, logsumexp, soft_max_qsa, max_qsa);
    //     }
    let residual = (soft_max_qsa - vt.get_value(s)).abs();
    vt.set_value(s, soft_max_qsa);

    residual
}
