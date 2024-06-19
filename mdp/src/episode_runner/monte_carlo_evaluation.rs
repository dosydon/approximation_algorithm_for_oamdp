use rand::rngs::ThreadRng;

use crate::mdp_traits::*;

pub fn monte_carlo_evaluation(evaluator: &mut dyn Eval, rng: &mut ThreadRng, n: usize) -> f32 {
    assert!(n > 0);
    let mut cumulative_cost = 0.0;
    for _ in 0..n {
        let cost = evaluator.eval(rng);
        cumulative_cost += cost;
    }

    cumulative_cost / (n as f32)
}
