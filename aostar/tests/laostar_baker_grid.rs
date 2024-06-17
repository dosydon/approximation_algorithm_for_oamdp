// use aostar::aostar::AOStar;
// use assert_approx_eq::assert_approx_eq;
// use mdp::baker_grid::{BakerGridMDP, BakerGridState};
// use mdp::grid_world::GridWorldMDP;
// use mdp::heuristic::ZeroHeuristic;
//
// #[test]
// fn test_baker_grid_aostar() {
//     let mdp = BakerGridMDP::new(3, 3, vec![], BakerGridState::new(0, 2));
//     let zero_heuristic = ZeroHeuristic {};
//     let err = 1e-1;
//     let mut aostar = AOStar::new(mdp, zero_heuristic);
//     aostar.ilaostar(err);
//     assert_approx_eq!(2.0 * (2.0 as f32).sqrt(), aostar.root_f(), err);
// }
//
// // #[test]
// // fn test_baker_grid_aostar_stochastic() {
// //     let width = 3;
// //     let height = 3;
// //     let obstacles = vec![];
// //
// //     let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles).set_prob_veering(0.5);
// //     let mdp = partial_mdp.build(BakerGridState::new(0, 2));
// //     let heuristic = HminPreferredHeuristic::new(&mdp);
// //     let err = 1e-1;
// //     let mut aostar = AOStar::new(&mdp, heuristic, None);
// //     aostar.ilaostar(err);
// //     assert_approx_eq!(4.258234673092283, aostar.root_f(), err);
// // }
//
// #[test]
// fn test_grid_laostar() {
//     let mdp = GridWorldMDP::default();
//     let zero_heuristic = ZeroHeuristic {};
//     let err = 1e-1;
//     let mut aostar = AOStar::new(mdp, zero_heuristic);
//     aostar.ilaostar(err);
//     assert_approx_eq!(aostar.root_f(), 9.787827666005699, err);
// }
