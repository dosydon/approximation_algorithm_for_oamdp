use mdp::heuristic::HminHeuristic;
use rtdp::rtdp_n::RTDPN;
use rtdp::rtdp::RTDP;
use std::env;
use std::time::Instant;
use multi_objective_mdp::domains::search_rescue_trevizan::{SRMDPD, SRFactory};
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];

    let mdp_d = SRFactory::from_name(name);
    let start = Instant::now();
    let mut rng = thread_rng();
    match mdp_d {
        SRMDPD::SRMDP2(mut mdp) => {
            let mut rtdp_n = RTDPN::new([RTDP::new(HminHeuristic::new()), RTDP::new(HminHeuristic::new())]);
            rtdp_n.lrtdp(&mut mdp, 1e-2, &mut rng);
        },
        SRMDPD::SRMDP3(mut mdp) => {
            let mut rtdp_n = RTDPN::new([RTDP::new(HminHeuristic::new()), RTDP::new(HminHeuristic::new())]);
            rtdp_n.lrtdp(&mut mdp, 1e-2, &mut rng);
        },
        SRMDPD::SRMDP4(mut mdp) => {
            let mut rtdp_n = RTDPN::new([RTDP::new(HminHeuristic::new()), RTDP::new(HminHeuristic::new())]);
            rtdp_n.lrtdp(&mut mdp, 1e-2, &mut rng);
        },
        SRMDPD::SRMDP5(mut mdp) => {
            let mut rtdp_n = RTDPN::new([RTDP::new(HminHeuristic::new()), RTDP::new(HminHeuristic::new())]);
            rtdp_n.lrtdp(&mut mdp, 1e-2, &mut rng);
        },
    };


    println!("elapsed {:?}", start.elapsed().as_secs_f32());
}