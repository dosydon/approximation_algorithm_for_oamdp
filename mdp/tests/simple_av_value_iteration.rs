








// #[test]
// fn test_simple_av_value_iteration_keep_speed() {
//     let mdp = SimpleAVMDP::new(0, 20, -4, 8, SimpleAVState::new(0, 2), NonYield(18, 2, 3));
//     let value_table = value_iteration_ssp(&mdp);
//
//     let tabular_policy =
//         TabularPolicy::<SimpleAVState, SimpleAVAction>::from_value_table_ssp(&mdp, &value_table);
//     let mut rng = thread_rng();
//     let runner = EpisodeRunner::new();
//     let result = runner.run_episode(&mdp, &tabular_policy, &mut rng);
//     let _expected = vec![
//         (SimpleAVState::new(0, 2), Some(Accelerate)),
//         (SimpleAVState::new(2, 3), Some(Decelerate)),
//         (SimpleAVState::new(5, 2), Some(Keep)),
//         (SimpleAVState::new(7, 2), None),
//     ];
//     for s in result.0.iter() {
//         println!("{:?}", s);
//     }
//     //     assert_eq!(result.0, expected);
// }

//
// #[test]
// fn test_simple_av_value_iteration_stop() {
//     let mdp = SimpleAVMDP::new(0, 20, -4, 8, SimpleAVState::new(0, 2), Stopping(12, 14));
//     let value_table = value_iteration_ssp(&mdp);
//
//     let tabular_policy =
//         TabularPolicy::<SimpleAVState, SimpleAVAction>::from_value_table_ssp(&mdp, &value_table);
//     let mut rng = thread_rng();
//     let runner = EpisodeRunner::new();
//     let result = runner.run_episode(&mdp, &tabular_policy, &mut rng);
//     let _expected = vec![
//         (SimpleAVState::new(0, 2), Some(Accelerate)),
//         (SimpleAVState::new(2, 3), Some(Decelerate)),
//         (SimpleAVState::new(5, 2), Some(Keep)),
//         (SimpleAVState::new(7, 2), None),
//     ];
// }
