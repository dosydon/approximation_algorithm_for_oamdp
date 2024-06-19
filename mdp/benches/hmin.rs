use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use mdp::cache_wrapper::CacheWrapper;
use mdp::race_track::*;
use mdp::heuristic::HminHeuristic;
use mdp::heuristic::*;
use mdp::mdp_traits::*;

fn hmin_benchmark(c: &mut Criterion) {

    let filename = "data/tracks/medium.track".to_string();
    c.bench_with_input(BenchmarkId::new("hmin", &filename), &filename, |b, filename| { 
        b.iter(|| {
            let mut h = HminHeuristic::new();
            let mut mdp = CacheWrapper::new(RaceTrackMDP::from_file(filename).set_p_slip(0.1));
            for _ in 0..100 {
                h.h_with_mut(&mdp.initial_state(), &mut mdp);
            }
        });
    });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!(benches, hmin_benchmark);
criterion_main!(benches);