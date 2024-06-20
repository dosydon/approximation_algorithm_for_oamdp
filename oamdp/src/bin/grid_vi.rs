use std::time::Instant;

use clap::Parser;
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
use mdp::mdp_traits::*;
use mdp::{
    episode_runner::{monte_carlo_evaluation, EpisodeRunner},
    mdp_traits::{Build, InitialState},
};
use oamdp::domains::baker_grid::BakerCOAMDPBuilder;
use oamdp::domains::baker_grid_reset::BakerResetOAMDPBuilder;
use oamdp::domains::blocks_world::BlocksOAMDPBuilder;
use oamdp::domains::recycle::RecycleCOAMDPBuilder;
use oamdp::domains::spelling::{SpellingCOAMDPBuilder, SpellingOAMDPBuilder};
use oamdp::oamdp::oamdp::OAMDP;
use oamdp::oamdp::BeliefState;
use oamdp::{
    algorithms::grid_based_value_iteration::grid_based_value_iteration_ssp,
    domains::baker_grid::BakerOAMDPBuilder,
};
use rand::thread_rng;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    domain: String,

    id: usize,

    n_bin_per_dim: usize,

    #[arg(short, long, default_value_t = false)]
    display: bool,

    #[arg(short, long, default_value_t = 10)]
    n: usize,

    #[arg(short, long, default_value_t = 13)]
    horizon: usize,
}

fn grid_vi<B, OM, M, A: Eq + Hash + Debug + Copy + Clone, const N: usize>(args: &Args, builder: B)
where
    B: Build<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>>,
    M: 'static + IsTerminal + ActionEnumerable + StateEnumerable + StatesActions + Cost,
    OM: 'static,
    A: 'static,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>, Action = A>
        + PMassMut<f32>
        + PMass<f32>
        + Cost
        + DCost
        + InitialState
        + GetNextState
        + DisplayState<BeliefState<M::State, N>>
        + ActionEnumerable,
{
    let mut rng = thread_rng();
    let start = Instant::now();

    let mut oamdp = builder.build().mdp;
    let v = grid_based_value_iteration_ssp(&mut oamdp, args.n_bin_per_dim);

    let mut runner =
        EpisodeRunner::new(&oamdp, &v, oamdp.initial_state()).set_max_horizon(Some(args.horizon));

    let end = Instant::now();
    let result = monte_carlo_evaluation(&mut runner, &mut rng, args.n);

    println!("Legibility Cost: {:.2?}", result);
    println!("Elapsed time: {:.2?}s", (end - start).as_secs_f32());
    println!("Num States: {}", v.num_states());
    println!("Num Domain States: {}", v.num_domain_states());
    println!("Root Value: {:.2?}", v.get_value(&oamdp.initial_state()));

    let mut runner =
        EpisodeRunner::new(&oamdp, &v, oamdp.initial_state()).set_max_horizon(Some(args.horizon));
    for (s, _a, _, _c) in runner.into_iter_with(&mut rng) {
        oamdp.display(&s);
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    println!("{:?}", args);

    match args.domain.as_str() {
        "baker" => grid_vi(
            &args,
            BakerOAMDPBuilder::<3>::new(args.id).set_horizon(args.horizon),
        ),
        "baker_com" => grid_vi(
            &args,
            BakerCOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        ),
        "blocks" => grid_vi(
            &args,
            BlocksOAMDPBuilder::new4_2_enumerable(args.id).set_horizon(args.horizon),
        ),
        "blocks4_3" => grid_vi(
            &args,
            BlocksOAMDPBuilder::new4_3_enumerable(args.id).set_horizon(args.horizon),
        ),
        "blocks6" => grid_vi(
            &args,
            BlocksOAMDPBuilder::new6_2_enumerable(args.id).set_horizon(args.horizon),
        ),
        "baker5" => grid_vi(
            &args,
            BakerOAMDPBuilder::<5>::new(args.id).set_horizon(args.horizon),
        ),
        "baker5_com" => grid_vi(
            &args,
            BakerCOAMDPBuilder::new5(args.id).set_horizon(args.horizon),
        ),
        "spelling" => grid_vi(
            &args,
            SpellingOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        ),
        "spelling_com" => grid_vi(
            &args,
            SpellingCOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        ),
        "reset" => grid_vi(
            &args,
            BakerResetOAMDPBuilder::<3>::new(args.id).set_horizon(args.horizon),
        ),
        "reset5" => grid_vi(
            &args,
            BakerResetOAMDPBuilder::<5>::new(args.id).set_horizon(args.horizon),
        ),
        "recycle" => grid_vi(&args, RecycleCOAMDPBuilder::new(args.id)),
        _ => panic!("{} not implemented", args.domain.as_str()),
    };
}
