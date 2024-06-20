// use aostar::aostar::AOStar;
// use legible_mdp::distance_measure::DistanceMeasure::*;
// use legible_mdp::legible_mdp_finite_horizon::LegibleMDPFiniteHorizonReward3;
// use legible_mdp::observability_assumption::ObservabilityAssumption::*;
// use mdp::baker_grid::baker_factory;
// use mdp::baker_grid::BakerGridMDP;
// use mdp::heuristic::ZeroHeuristic;
// use mdp::policy::softmax_policy::{SoftmaxRewardPolicy, SoftmaxPolicyBuilder};
//
// #[test]
// fn test_legible_baker_a8_v_infinity_based_large1_reward() {
//     let pair = baker_factory("Large1");
//     let goals = pair.1;
//     let partial_mdp = pair.0;
//
//     let softmax_policy = SoftmaxPolicyBuilder::new(3.0);
//     let legible_mdp = LegibleMDPFiniteHorizonReward3::<SoftmaxRewardPolicy<BakerGridMDP>, BakerGridMDP>::new(
//         &partial_mdp,
//         &softmax_policy,
//         goals,
//         0,
//         8,
//         Euclidean,
//         ActionObservable,
//     );
//     let zero_heuristic = ZeroHeuristic {};
//     let mut aostar = AOStar::new(&legible_mdp, zero_heuristic, None);
//     aostar.aostar();
//
//     assert_eq!(4.918374874879206, aostar.root_f());
// }
