// use aostar::aostar::AOStar;
// use assert_approx_eq::assert_approx_eq;
// use mdp::heuristic::HminPreferredActionAvailableHeuristic;
// use mdp::heuristic::ZeroHeuristic;
// use mdp::race_track::*;
// 
// fn init() {
//     let _ = env_logger::builder().is_test(true).try_init();
// }
// 
// #[test]
// fn test_race_track_aostar() {
//     init();
//     let mdp = RaceTrackMDP::from_file("data/tracks/small.track").set_p_slip(0.1);
//     let zero_heuristic = ZeroHeuristic {};
//     let err = 1e-4;
//     let mut aostar = AOStar::new(mdp, zero_heuristic);
//     aostar.ilaostar(err);
//     //     aostar.dump();
//     println!("{:?}", aostar.num_generated());
//     assert_approx_eq!(7.4801, aostar.root_f(), err);
// }
// 
// #[test]
// fn test_race_track_aostar_hmin() {
//     init();
//     let mdp = RaceTrackMDP::from_file("data/tracks/small.track").set_p_slip(0.1);
//     let heuristic = HminPreferredActionAvailableHeuristic::new();
//     let err = 1e-4;
//     let mut aostar = AOStar::new(mdp, heuristic);
//     aostar.ilaostar(err);
//     println!("{:?}", aostar.num_generated());
//     //     aostar.dump();
//     //     println!("{:?}", aostar.root_f());
//     assert_approx_eq!(7.4801, aostar.root_f(), err);
// }
// 