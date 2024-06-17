use mdp::cache_wrapper::CacheWrapper;
use mdp::race_track::*;
use mdp::heuristic::HminHeuristic;
use std::env;
use std::time::Instant;
use rtdp::rtdp::RTDP;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut rng = thread_rng();
    let mut mdp = CacheWrapper::new(
        RaceTrackMDP::from_file(filename)
            .set_p_slip(0.1)
    );

    let start = Instant::now();
    let mut rtdp = RTDP::new(HminHeuristic::new());
    rtdp.lrtdp(&mut mdp,  &mut rng, 1e-2);
    println!("elapsed {:?}", start.elapsed());
}
