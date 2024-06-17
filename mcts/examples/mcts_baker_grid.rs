use mcts::{Budget, MCTS};
use mdp::{
    baker_grid::{BakerGridPartialMDP, BakerGridState},
    finite_horizon_wrapper::{FiniteHorizonPolicyWrapper, FiniteHorizonWrapper},
    into_inner::Inner,
    mdp_traits::{BuildFrom, DisplayState},
    policy::tabular_policy::TabularPolicy,
    value_iteration::value_iteration_ssp,
};
use rand::thread_rng;

fn main() {
    env_logger::init();
    let width = 9;
    let height = 7;
    let obstacles = vec![];

    let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles)
        .set_prob_veering(0.1)
        .set_initial_state(BakerGridState::new(3, 0));

    let mdp = partial_mdp.build_from(&BakerGridState::new(6, 8));
    let vt = value_iteration_ssp(&mdp);
    let policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
    let policy = FiniteHorizonPolicyWrapper::new(policy);

    //     let policy = RandomPolicy {};
    let grid = mdp.grid2d.clone();
    let mdp = FiniteHorizonWrapper::new(mdp, 20);
    let mut mcts = MCTS::new(mdp, policy)
        .set_c(5.0)
        .set_num_rollouts(10)
        .set_budget(Budget::TimeBudget(1.0));
    let mut rng = thread_rng();

    for (_s, _a, ss, _c) in mcts.into_iter_with(&mut rng) {
        println!("{:?}", ss);
        grid.display(&ss.inner());
    }
}
