use assert_approx_eq::assert_approx_eq;
use mdp::blocks_world::Block;
use mdp::blocks_world::BlocksWorldMDPN;
use mdp::blocks_world::Location::*;
use mdp::episode_runner::EpisodeRunner;
use mdp::mdp_traits::Eval;
use mdp::policy::tabular_policy::TabularPolicy;
use mdp::state_enumerable_wrapper::StateEnumerableWrapper;
use mdp::value_iteration::value_iteration_ssp;
use rand::prelude::*;

#[test]
fn test_blocks_world_value_iteration() {
    let mdp = StateEnumerableWrapper::new(BlocksWorldMDPN::new(
        [OnTable, OnTable, On(Block::new(1)), OnTable],
        [
            On(Block::new(3)),
            On(Block::new(2)),
            OnTable,
            On(Block::new(1)),
        ],
        0.0,
        ['A', 'M', 'S', 'R'],
    ));
    let value_table = value_iteration_ssp(&mdp);

    let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
    let mut rng = thread_rng();
    let mut runner = EpisodeRunner::from_initial_state(&mdp, &tabular_policy);
    let result = runner.eval(&mut rng);
    assert_approx_eq!(result, 7.0);
}

#[test]
fn test_blocks_world_value_iteration6() {
    let b0 = Block::new(0);
    let b1 = Block::new(1);
    let b2 = Block::new(2);
    let b3 = Block::new(3);
    let b5 = Block::new(5);
    let mdp = StateEnumerableWrapper::new(BlocksWorldMDPN::new(
        [OnTable, OnTable, OnTable, OnTable, OnTable, OnTable],
        [OnTable, On(b0), On(b1), On(b2), On(b5), On(b3)],
        0.0,
        ['A', 'B', 'C', 'M', 'S', 'R'],
    ));
    let value_table = value_iteration_ssp(&mdp);

    let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
    let mut rng = thread_rng();
    let mut runner = EpisodeRunner::from_initial_state(&mdp, &tabular_policy);
    let result = runner.eval(&mut rng);
    assert_approx_eq!(result, 9.0);
}
