use assert_approx_eq::assert_approx_eq;
use mdp::episode_runner::{monte_carlo_evaluation, EpisodeRunner};
use mdp::grid_world::{GridWorldMDP, GridWorldState};
use mdp::mdp_traits::StateEnumerable;
use mdp::policy::tabular_policy::TabularPolicy;
// use mdp::policy::ziebart_entropy_policy::ZiebartEntropyPolicy;
use mdp::value_iteration::{value_iteration, value_iteration_ssp};
use rand::prelude::*;

#[test]
fn test_grid_world_value_iteration() {
    let mdp = GridWorldMDP::default();
    let value_table = value_iteration(&mdp);
    let expected = [
        4.01869, 4.37161, 3.86717, 3.41827, 2.99774, 4.55478, 5.03236, 4.38997, 3.83191, 2.93095,
        5.15754, 5.8013, 6.0733, 5.83364, 6.64727, 7.5769, 8.57383, 9.69459, 6.45529, 7.39071,
        8.46366, 9.69459, 0.0,
    ];
    let err = 1e-3;

    for (id, s) in mdp.enumerate_states().enumerate() {
        assert_approx_eq!(value_table.get_value(s), expected[id], err);
    }
}

#[test]
fn test_grid_world_soft_value_iteration() {
    let mdp = GridWorldMDP::default();
    let value_table = value_iteration(&mdp);
    //     let expected = [
    //         4.01869, 4.37161, 3.86717, 3.41827, 2.99774, 4.55478, 5.03236, 4.38997, 3.83191, 2.93095,
    //         5.15754, 5.8013, 6.0733, 5.83364, 6.64727, 7.5769, 8.57383, 9.69459, 6.45529, 7.39071,
    //         8.46366, 9.69459, 0.0,
    //     ];
    //     let err = 1e-3;

    for (_id, s) in mdp.enumerate_states().enumerate() {
        println!("{:?}", value_table.get_value(s));
        //         assert_approx_eq!(value_table.get_value(s), expected[id], err);
    }
}

#[test]
fn test_grid_world_value_iteration_ssp() {
    let mdp = GridWorldMDP::new(
        4,
        4,
        GridWorldState::new(0, 0),
        GridWorldState::new(3, 3),
        vec![GridWorldState::new(2, 3)],
        vec![],
    );
    let value_table = value_iteration_ssp(&mdp);
    let err = 1e-1;
    let expected = vec![
        7.558234211455596,
        6.531666163736264,
        5.964032295772926,
        7.158221299903407,
        6.372394696899038,
        5.252982774217446,
        4.603918364972983,
        6.265245347407095,
        5.192357938588053,
        3.973586054099136,
        3.2095189028435804,
        2.953501111932506,
        4.018531180671074,
        2.6951670081944945,
        1.365265817814859,
        0.0,
    ];

    for (id, s) in mdp.enumerate_states().enumerate() {
        assert_approx_eq!(value_table.get_value(s), expected[id], err);
    }

    let tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);
    let mut rng = thread_rng();
    let mut runner = EpisodeRunner::from_initial_state(&mdp, &tabular_policy);
    let result = monte_carlo_evaluation(&mut runner, &mut rng, 10000);
    assert_approx_eq!(result, 7.558234211455596, err);
}

// #[test]
// fn test_grid_world_soft_value_iteration_ssp() {
//     let mdp = GridWorldMDP::new(
//         4,
//         4,
//         GridWorldState::new(0, 0),
//         GridWorldState::new(3, 3),
//         vec![GridWorldState::new(2, 3)],
//         vec![],
//     );
//     let value_table = soft_value_iteration_ssp(&mdp);
//     let err = 1e-3;
//     let expected = vec![
//         5.7508303853328915,
//         5.458562918575882,
//         5.380820342441303,
//         6.146342277370471,
//         4.976519237434984,
//         4.508445964935423,
//         4.3026394806621955,
//         5.838028552205948,
//         4.1466619253084795,
//         3.450563294800355,
//         3.0318195911974932,
//         2.9316489269474264,
//         3.478244036205737,
//         2.4352292534588456,
//         1.2514205462796486,
//         0.0,
//     ];
//
//     for (id, s) in mdp.enumerate_states().enumerate() {
//         assert_approx_eq!(value_table.get_value(s), expected[id], err);
//     }
// }
