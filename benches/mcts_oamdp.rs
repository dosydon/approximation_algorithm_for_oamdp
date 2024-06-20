use criterion::{criterion_group, criterion_main, Criterion};

use mcts::MCTS;

use mdp::{mdp_traits::Build, policy::tabular_policy::TabularPolicy};

use mdp::value_iteration::value_iteration_ssp;
use oamdp::{
    domain_evaluator::DomainEvaluator, domains::baker_grid::BakerCOAMDPBuilder,
    policy::TabularOAMDPPolicy,
};

use rand::thread_rng;

fn mcts_oamdp_baker(c: &mut Criterion) {
    c.bench_function("mcts_oamdp", |b| {
        let mut rng = thread_rng();
        let builder = BakerCOAMDPBuilder::new(3);
        let oamdp = builder.build();
        let vt = value_iteration_ssp(&oamdp.mdp.mdp);
        let policy =
            TabularOAMDPPolicy::new(TabularPolicy::from_value_table_ssp(&oamdp.mdp.mdp, &vt));
        let mut mcts = MCTS::new(oamdp, policy).set_c(5.0);

        b.iter(|| mcts.solve(1000, &mut rng));
    });
}

fn mcts_oamdp_baker_no_belief_update(c: &mut Criterion) {
    c.bench_function("mcts_oamdp_no_belief_update", |b| {
        let mut rng = thread_rng();
        let builder = BakerCOAMDPBuilder::new(3);
        let oamdp = builder.build();

        let vt = value_iteration_ssp(&oamdp.mdp.mdp);
        let policy = DomainEvaluator::new(TabularPolicy::from_value_table_ssp(&oamdp.mdp.mdp, &vt));
        let mut mcts = MCTS::new(oamdp, policy).set_c(5.0);

        b.iter(|| mcts.solve(1000, &mut rng));
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = mcts_oamdp_baker, mcts_oamdp_baker_no_belief_update
}
criterion_main!(benches);
