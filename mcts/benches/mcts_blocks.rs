use criterion::{criterion_group, criterion_main, Criterion};
use mcts::MCTS;

use mdp::blocks_world::Location::*;
use mdp::blocks_world::{Block, BlocksWorldMDPN};
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;

use mdp::policy::random_policy::RandomPolicy;
use rand::thread_rng;

fn mcts_blocks6(c: &mut Criterion) {
    c.bench_function("mcts_blocks6", |b| {
        let b0 = Block::new(0);
        let b1 = Block::new(1);
        let b2 = Block::new(2);
        let b3 = Block::new(3);
        let b4 = Block::new(4);
        let mdp = BlocksWorldMDPN::<6>::new(
            [OnTable, OnTable, OnTable, OnTable, OnTable, OnTable],
            [OnTable, On(b0), On(b1), On(b2), On(b3), On(b4)],
            0.1,
            ['A', 'B', 'C', 'D', 'E', 'F'],
        );
        let wrapper = FiniteHorizonWrapper::new(mdp, 15);
        let mut rng = thread_rng();

        let random_policy = RandomPolicy {};
        let mut mcts = MCTS::new(wrapper, random_policy).set_c(-1.0);

        b.iter(|| {
            mcts.solve(10, &mut rng);
            mcts.clear();
        });
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = mcts_blocks6
}
criterion_main!(benches);
