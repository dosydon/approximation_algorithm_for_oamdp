// use aostar::aostar::AOStar;
// use mdp::canadian_traveler::*;
// use mdp::heuristic::ZeroHeuristic;
//
// #[test]
// fn test_canadian_traveler_aostar() {
//     let mdp = CanadianTravelerMDP::default();
//     let zero_heuristic = ZeroHeuristic {};
//     let err = 1e-1;
//     let mut aostar = AOStar::new(mdp, zero_heuristic);
//     aostar.ilaostar(err);
//     aostar.dump();
//     println!("{:?}", aostar.root_f());
//     //     assert_approx_eq!(2.0 * (2.0 as f32).sqrt(), aostar.root_f(), err);
// }
