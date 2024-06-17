// use assert_approx_eq::assert_approx_eq;
// use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
// use mdp::grid_world::{GridWorldMDP, GridWorldState};
// use mdp::mdp_traits::InitialState;
// use mdp::state_explicating_wrapper::*;
// use mdp::value_iteration::{value_iteration, value_iteration_ssp};
//
// #[test]
// fn test_finite_horizon_value_iteration_ssp() {
//     let mdp = GridWorldMDP::new(
//         4,
//         4,
//         GridWorldState::new(0, 0),
//         GridWorldState::new(3, 3),
//         vec![GridWorldState::new(2, 3)],
//         vec![],
//     );
//     let finite_horizon_mdp = StateExplicatingWrapper::new(FiniteHorizonWrapper::new(mdp, 9));
//     let value_table = value_iteration_ssp(&finite_horizon_mdp);
//     let err = 1e-3;
//     assert_approx_eq!(
//         value_table.get_value(&finite_horizon_mdp.initial_state()),
//         7.31875,
//         err
//     );
// }
