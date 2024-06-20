// use aostar::aostar::AOStar;
// // use legible_mdp::legible_heuristic::LegibleHeuristic;
// use legible_mdp::distance_measure::DistanceMeasure::Euclidean;
// use legible_mdp::legible_mdp::LegibleMDP2;
// use mdp::episode_runner::run_episode;
// use mdp::finite_horizon_wrapper::{FiniteHorizonWrapper, FiniteHorizonWrapperBuilder};
// use mdp::grid_world::{GridWorldMDP, GridWorldPartialMDP, GridWorldState};
// use mdp::heuristic::ZeroHeuristic;
// use mdp::policy::softmax_policy::{SoftmaxPolicy, SoftmaxPolicyBuilder};
// use rand::prelude::*;
//
// #[test]
// #[ignore]
// fn test_aostar_legible() {
//     let partial_mdp = GridWorldPartialMDP::new(5, 5, GridWorldState::new(2, 4), vec![], vec![]);
//     let possible_goals = vec![
//         (&partial_mdp, GridWorldState::new(0, 2)),
//         (&partial_mdp, GridWorldState::new(2, 0)),
//     ];
//     let softmax_policy = SoftmaxPolicyBuilder::new(3.0);
//     let wrapper_factory = FiniteHorizonWrapperBuilder::new(9);
//     let legible_mdp = LegibleMDP2::<
//         SoftmaxPolicy<FiniteHorizonWrapper<GridWorldMDP>>,
//         FiniteHorizonWrapper<GridWorldMDP>,
//     >::new(
//         &wrapper_factory,
//         &softmax_policy,
//         possible_goals,
//         1,
//         Euclidean,
//     );
//
//     let zero_heuristic = ZeroHeuristic {};
//     let mut aostar = AOStar::new(legible_mdp, zero_heuristic);
//     aostar.aostar();
//     //     assert_approx_eq!(1.3337670577468095, aostar.root_f(), err);
//     let tabular_policy = aostar.to_policy();
//     //     let expected = vec![
//     //         BakerGridState { i: 8, j: 0 },
//     //         BakerGridState { i: 7, j: 1 },
//     //         BakerGridState { i: 8, j: 2 },
//     //         BakerGridState { i: 7, j: 3 },
//     //         BakerGridState { i: 6, j: 4 },
//     //         BakerGridState { i: 5, j: 5 },
//     //         BakerGridState { i: 4, j: 6 },
//     //         BakerGridState { i: 3, j: 7 },
//     //         BakerGridState { i: 3, j: 8 },
//     //         BakerGridState { i: 2, j: 9 },
//     //         BakerGridState { i: 1, j: 10 },
//     //     ];
//
//     let mut rng = thread_rng();
//     let result = run_episode(&aostar.mdp, &tabular_policy, &mut rng);
//     println!("{:?}", result);
//     //     assert_eq!(projected, expected);
// }
//
// #[test]
// #[ignore]
// fn test_aostar_legible_heuristic() {
//     let partial_mdp = GridWorldPartialMDP::new(5, 5, GridWorldState::new(2, 4), vec![], vec![]);
//     let possible_goals = vec![
//         (&partial_mdp, GridWorldState::new(0, 2), 9),
//         (&partial_mdp, GridWorldState::new(2, 0), 9),
//     ];
//     let softmax_policy = SoftmaxPolicy::new(3.0);
//     let legible_mdp = LegibleMDP2::<
//         (&GridWorldPartialMDP, GridWorldState, usize),
//         SoftmaxPolicy,
//         FiniteHorizonWrapper<GridWorldMDP>,
//     >::new(possible_goals, softmax_policy, 1);
//
//     let legible_heuristic = LegibleHeuristic::new(&legible_mdp);
//     let err = 1e-1;
//     let mut aostar = AOStar::new(&legible_mdp, legible_heuristic);
//     aostar.aostar();
//     //     assert_eq!(44281, aostar.count_on_solution());
//     //     assert_eq!(119565, aostar.num_generated());
//     assert_approx_eq!(1.3337670577468095, aostar.root_f(), err);
//     //     aostar.dump_on_solution()
// }

// #[test]
// #[ignore]
// fn test_ilaostar_indefinite_horizon() {
//     let partial_mdp = GridWorldPartialMDP::new(5, 5, GridWorldState::new(2, 4), vec![], vec![]);
//     let possible_goals = vec![GridWorldState::new(0, 2), GridWorldState::new(2, 0)];
//     let softmax_policy = SoftmaxPolicyBuilder::new(3.0);
//     let legible_mdp = LegibleMDP2::<SoftmaxPolicy<GridWorldMDP>, GridWorldMDP>::new(
//         &partial_mdp,
//         &softmax_policy,
//         possible_goals,
//         1,
//     );
//
//     //     let legible_heuristic = LegibleHeuristic::new(&legible_mdp);
//     let zero_heuristic = ZeroHeuristic {};
//     let err = 1e-1;
//     let mut aostar = AOStar::new(&legible_mdp, zero_heuristic);
//     aostar.ilaostar(err);
//     assert_approx_eq!(1.2584900361097509, aostar.root_f(), err);
//     //     aostar.dump_on_solution()
// }
