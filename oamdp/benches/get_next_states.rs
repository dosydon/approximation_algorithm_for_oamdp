use criterion::{criterion_group, criterion_main, Criterion};

use mdp::mdp_traits::*;
use mdp::policy::policy_traits::GetAction;
use mdp::policy::random_policy::RandomPolicy;

use oamdp::domains::baker_grid::BakerCOAMDPBuilder;

use rand::thread_rng;

fn oamdp_get_next_state(c: &mut Criterion) {
    c.bench_function("oamdp_get_next_state", |b| {
        let mut rng = thread_rng();
        let builder = BakerCOAMDPBuilder::new(3);
        let oamdp = builder.build().mdp;
        let mut state = oamdp.initial_state();
        let random_policy = RandomPolicy {};

        b.iter(|| {
            for _ in 0..100 {
                let a = random_policy.get_action(&state, &oamdp, &mut rng).unwrap();
                state = oamdp.get_next_state(&state, &a, &mut rng);
            }
        });
    });
}

fn oamdp_get_next_state_mut(c: &mut Criterion) {
    c.bench_function("oamdp_get_next_state_mut", |b| {
        let mut rng = thread_rng();
        let builder = BakerCOAMDPBuilder::new(3);
        let mut oamdp = builder.build().mdp;
        let mut state = oamdp.initial_state();
        let random_policy = RandomPolicy {};

        b.iter(|| {
            for _ in 0..100 {
                let a = random_policy.get_action(&state, &oamdp, &mut rng).unwrap();
                state = oamdp.get_next_state_mut(&state, &a, &mut rng);
            }
        });
    });
}

fn mdp_get_next_state(c: &mut Criterion) {
    c.bench_function("mdp_get_next_state", |b| {
        let mut rng = thread_rng();
        let builder = BakerCOAMDPBuilder::new(3);
        let oamdp = builder.build();
        let mdp = oamdp.mdp.mdp;
        let mut state = mdp.initial_state();
        let random_policy = RandomPolicy {};

        b.iter(|| {
            for _ in 0..100 {
                let a = random_policy.get_action(&state, &mdp, &mut rng).unwrap();
                state = mdp.get_next_state(&state, &a, &mut rng);
            }
        });
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = oamdp_get_next_state_mut, oamdp_get_next_state, mdp_get_next_state
}
criterion_main!(benches);
