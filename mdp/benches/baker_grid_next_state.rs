use criterion::{criterion_group, criterion_main, Criterion};
use mdp::{
    baker_grid::{BakerGridAction::*, BakerGridMDP, BakerGridPartialMDP, BakerGridState},
    mdp_traits::*,
};
use rand::thread_rng;

fn baker_grid_next_state(c: &mut Criterion) {
    c.bench_function("baker_grid_next_state", |b| {
        let width = 17;
        let height = 9;
        let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

        let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles).set_prob_veering(0.5);
        let mdp: BakerGridMDP = partial_mdp.build_from(&BakerGridState::new(0, 16));

        let _rng = thread_rng();
        b.iter(|| {
            for _ in 0..100 {
                let _s =
                    mdp.get_next_state(&BakerGridState::new(5, 6), &East, &mut rand::thread_rng());
            }
        });
    });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!(benches, baker_grid_next_state);
criterion_main!(benches);
