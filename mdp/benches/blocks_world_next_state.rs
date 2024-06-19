use criterion::{criterion_group, criterion_main, Criterion};
use mdp::blocks_world::BlocksWorldAction::*;
use mdp::blocks_world::Location::*;
use mdp::blocks_world::{Block, BlocksWorldMDPN};
use mdp::mdp_traits::*;
use rand::thread_rng;

fn blocks_world_next_state_benachmark(c: &mut Criterion) {
    c.bench_function("blocks_world_next_state", |b| {
        let mdp = BlocksWorldMDPN::<6>::new(
            [OnTable, OnTable, OnTable, OnTable, OnTable, OnTable],
            [
                OnTable,
                On(Block::new(0)),
                On(Block::new(1)),
                On(Block::new(2)),
                On(Block::new(5)),
                On(Block::new(3)),
            ],
            0.0,
            ['A', 'B', 'C', 'M', 'S', 'R'],
        );
        let mut rng = thread_rng();
        b.iter(|| {
            for _ in 0..100 {
                mdp.get_next_state(&mdp.initial_state(), &PickUp(Block::new(0)), &mut rng);
            }
        });
    });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!(benches, blocks_world_next_state_benachmark);
criterion_main!(benches);
