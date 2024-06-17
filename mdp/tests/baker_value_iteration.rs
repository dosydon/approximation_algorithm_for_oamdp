use mdp::baker_grid::{BakerGridMDP, BakerGridPartialMDP, BakerGridState};
use mdp::mdp_traits::{BuildFrom, StateEnumerable};
use mdp::value_iteration::{value_iteration, value_iteration_ssp};

#[test]
fn test_baker_value_iteration_a() {
    let width = 17;
    let height = 9;
    let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

    let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles);
    let mdp: BakerGridMDP = partial_mdp.build_from(&BakerGridState::new(0, 16));
    let value_table = value_iteration(&mdp);

    //     for s in mdp.enumerate_states() {
    //         for a in mdp.enumerate_actions() {
    //             println!("{:?} {:?} {:?} {:?}", s, a, softmax_policy.vt.get_qsa_ssp(s, a, &mdp), softmax_policy.get_probability(s, a, &mdp));
    //         }
    //     }
    for s in mdp.enumerate_states() {
        println!("{:?} {:?}", s, value_table.get_value(s));
    }
}

#[test]
fn test_baker_value_iteration_c() {
    let width = 17;
    let height = 9;
    let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

    let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles);
    let mdp: BakerGridMDP = partial_mdp.build_from(&BakerGridState::new(0, 8));
    let _value_table = value_iteration_ssp(&mdp);

    //     let softmax_policy = SoftmaxPolicy::new(3.0, value_table);
    //     for s in mdp.enumerate_states() {
    //         for a in mdp.enumerate_actions() {
    //             println!("{:?} {:?} {:?} {:?}", s, a, softmax_policy.vt.get_qsa_ssp(s, a, &mdp), softmax_policy.get_probability(s, a, &mdp));
    //         }
    //     }
    //     for s in mdp.enumerate_states() {
    //         println!("{:?} {:?}", s, value_table.get_value(s));
    //     }
}
