use criterion::{criterion_group, criterion_main, Criterion};

use mdp::{
    baker_grid::{baker_factory, BakerGridMDP},
    heuristic::ZeroHeuristic,
    policy::softmax_policy::SoftmaxPolicyBuilder,
};
use oamdp::{
    algorithms::rtdp::{RTDPTrait, RTDP_OAMDP},
    belief_update_type::ObserveabilityAssumption::OnlyActionsAreConsidered,
    observer_model::SoftmaxModel,
};
use oamdp::{
    belief_cost_function::{self, Objective},
    oamdp::oamdp::OAMDP2,
};
use rand::thread_rng;

// fn rtdp_baker_linear_iterpolation(c: &mut Criterion) {
//     c.bench_function("rtdp_baker_linear_interpolation", |b| {
//         let pair = baker_factory("Tiny2");
//         let partial_mdp = pair.0;
//         let goals = [pair.1[0], pair.1[1]];
//         println!("{:?}", goals);
//
//         let softmax_policy = SoftmaxPolicyBuilder::new(0.3);
//         let oamdp = OAMDP2::<SoftmaxModel<BakerGridMDP, 2>, BakerGridMDP>::new_softmax(
//             &partial_mdp,
//             &softmax_policy,
//             goals,
//             1,
//             belief_cost_function::BeliefCostType::TVDistance,
//             Objective::LinearCombination(1.0, 1.0),
//             OnlyActionsAreConsidered,
//         );
//         let n_bin_per_dim = 20;
//
//         let mut rng = thread_rng();
//         let h = ZeroHeuristic {};
//         let mut rtdp: RTDPLinearInterpolation<BakerGridState, BakerGridAction, ZeroHeuristic, 2> =
//             RTDPLinearInterpolation::new(h, RTDPGridResolution::Fixed(n_bin_per_dim));
//
//         b.iter(|| rtdp.rtdp(&oamdp, 1000, &mut rng));
//     });
// }

fn rtdp_baker(c: &mut Criterion) {
    c.bench_function("rtdp_baker", |b| {
        let pair = baker_factory("Tiny2");
        let partial_mdp = pair.0;
        let goals = [pair.1[0], pair.1[1]];
        println!("{:?}", goals);

        let softmax_policy = SoftmaxPolicyBuilder::new(0.3);
        let oamdp = OAMDP2::<SoftmaxModel<BakerGridMDP, 2>, BakerGridMDP>::new_implicit_model(
            &partial_mdp,
            &softmax_policy,
            goals,
            1,
            belief_cost_function::BeliefCostType::TVDistance,
            Objective::LinearCombination(1.0, 1.0),
            OnlyActionsAreConsidered,
        );
        let n_bin_per_dim = 20;

        let mut rng = thread_rng();
        let h = ZeroHeuristic {};
        let mut rtdp = RTDP_OAMDP::new(oamdp, h, n_bin_per_dim);
        //         let mut rtdp: RTDPLinearInterpolation<BakerGridState, BakerGridAction, ZeroHeuristic, 2> =
        //             RTDPLinearInterpolation::new(h, RTDPGridResolution::Fixed(n_bin_per_dim));

        b.iter(|| rtdp.rtdp(1000, &mut rng));
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = rtdp_baker
}
criterion_main!(benches);
