use mdp::cache_wrapper::CacheWrapper;
use mdp::grid_world::*;
use std::env;
use std::time::Instant;
use rtdp::rtdp_n::RTDPN;
use rtdp::rtdp::RTDP;
use rand::prelude::*;
use mdp::heuristic::HminHeuristic;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut mdp = CacheWrapper::new(GridWorldMDP::from_file(filename));

    let start = Instant::now();
    let mut rng = thread_rng();
    let mut rtdp_n = RTDPN::new([RTDP::new(HminHeuristic::new()), RTDP::new(HminHeuristic::new())], );
    rtdp_n.lrtdp(&mut mdp, 1e-2, &mut rng);
    println!("elapsed {:?}", start.elapsed().as_secs_f32());
}
