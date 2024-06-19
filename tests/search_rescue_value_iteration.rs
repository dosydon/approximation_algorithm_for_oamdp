// use mdp::episode_runner::EpisodeRunner;
// use mdp::mdp_traits::{ActionEnumerable, StateEnumerable};
// use mdp::policy::tabular_policy::TabularPolicy;
// use mdp::search_rescue::*;
// use mdp::value_estimator::CostEstimator;
// use mdp::value_iteration::value_iteration_ssp;
// use rand::thread_rng;
//
// #[test]
// fn test_search_rescue_value_iteration() {
//     let mdp = SearchRescueNonRemovingMDP::new(
//         11,
//         11,
//         vec![(0, 10), (1, 9), (1, 10)],
//         [Coordinate::new(0, 0), Coordinate::new(0, 10)],
//         Coordinate::new(9, 5),
//         ObstacleCompatibility::Low,
//     );
//     let value_table = value_iteration_ssp(&mdp);
//     for s in mdp.enumerate_states() {
//         println!("{:?} {:?}", s, value_table.get_value(s));
//         for a in mdp.enumerate_actions() {
//             println!("{:?} {:?}", a, value_table.get_qsa_ssp(s, a, &mdp));
//         }
//     }
//
//     let tabular_policy =
//         TabularPolicy::<SearchRescueNonRemovingState, SearchRescueAction>::from_value_table_ssp(
//             &mdp,
//             &value_table,
//         );
//     let mut rng = thread_rng();
//     let runner = EpisodeRunner::new();
//     let result = runner.run_episode(&mdp, &tabular_policy, &mut rng);
//
//     for s in result.0.iter() {
//         println!("{:?}", s);
//     }
// }
//
