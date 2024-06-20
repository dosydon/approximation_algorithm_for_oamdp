#![feature(trait_upcasting)]
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Instant;

use clap::Parser;
use mdp::{
    episode_runner::monte_carlo_evaluation,
    finite_horizon_wrapper::FiniteHorizonWrapper,
    heuristic::ZeroHeuristic,
    into_inner::IntoInner,
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Build, Cost, GetNextStateMut, InitialState,
        IsTerminal, PMass, PMassMut, StateEnumerable, StatesActions,
    },
    value_iteration::value_iteration_ssp,
};
use oamdp::{
    algorithms::rtdp::{RTDPTraitAll, RTDP_OAMDP},
    belief_cost_function::Objective,
    domains::{
        baker_grid::{BakerCOAMDPBuilder, BakerOAMDPBuilder},
        baker_grid_reset::BakerResetOAMDPBuilder,
        blocks_world::BlocksOAMDPBuilder,
        recycle::RecycleCOAMDPBuilder,
        spelling::{SpellingCOAMDPBuilder, SpellingOAMDPBuilder},
    },
    oamdp::oamdp::OAMDP,
    scaled_rtdp::ScaledRTDP,
    scaled_value_table::ScaledValueTable,
};
use rand::thread_rng;
use rtdp::rtdp::RTDP;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    domain: String,

    id: usize,

    n_bin_per_dim: usize,

    num_trials: usize,

    #[arg(short, long, default_value_t = 10)]
    n: usize,

    #[arg(short, long, default_value_t = 13)]
    horizon: usize,

    #[arg(short, long, default_value_t = false)]
    lrtdp: bool,

    #[arg(short, long, default_value_t = false)]
    domain_heuristic: bool,
}

fn build_rtdp<B, OM, M, A: Eq + Hash + Debug + Copy + Clone, const N: usize>(
    args: &Args,
    builder: B,
) -> Box<dyn RTDPTraitAll>
where
    B: Build<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>>,
    M: 'static
        + IsTerminal
        + ActionEnumerable
        + StateEnumerable
        + StatesActions
        + Cost
        + PMass<f32>
        + ActionAvailability,
    OM: 'static,
    A: 'static,
    RTDP_OAMDP<OM, M, A, ZeroHeuristic, N>: RTDPTraitAll,
    RTDP_OAMDP<OM, M, A, ScaledValueTable<M::State>, N>: RTDPTraitAll,
{
    let oamdp = builder.build().mdp;

    if args.domain_heuristic {
        let vt = value_iteration_ssp(oamdp.into_inner());
        let alpha = match oamdp.objective {
            Objective::BeliefCostOnly => 0.0,
            Objective::LinearCombination(_c, d) => d,
        };
        let vt = ScaledValueTable::new(alpha, vt);
        let rtdp = RTDP_OAMDP::new(oamdp, vt, args.n_bin_per_dim).set_max_horizon(args.horizon);
        Box::new(rtdp)
    } else {
        let h = ZeroHeuristic {};
        let rtdp = RTDP_OAMDP::new(oamdp, h, args.n_bin_per_dim).set_max_horizon(args.horizon);
        Box::new(rtdp)
    }
}

fn build_rtdp_rtdp<B, OM, M, A: Eq + Hash + Debug + Copy + Clone, const N: usize>(
    args: &Args,
    builder: B,
) -> Box<dyn RTDPTraitAll>
where
    B: Build<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>>,
    M: 'static
        + IsTerminal
        + ActionEnumerable
        + StatesActions
        + InitialState
        + GetNextStateMut
        + Cost
        + PMassMut<f32>
        + ActionAvailability,
    OM: 'static,
    A: 'static,
    RTDP_OAMDP<OM, M, A, ZeroHeuristic, N>: RTDPTraitAll,
    RTDP_OAMDP<OM, M, A, ScaledRTDP<M::State, ZeroHeuristic>, N>: RTDPTraitAll,
{
    let mut oamdp = builder.build().mdp;

    if args.domain_heuristic {
        let mut rtdp = RTDP::new(ZeroHeuristic {});
        rtdp.lrtdp(&mut oamdp.mdp, 0, &mut thread_rng(), 1e-3);

        //         let vt = value_iteration_ssp(oamdp.into_inner());
        let alpha = match oamdp.objective {
            Objective::BeliefCostOnly => 0.0,
            Objective::LinearCombination(_c, d) => d,
        };
        let h = ScaledRTDP::new(alpha, rtdp);
        let rtdp = RTDP_OAMDP::new(oamdp, h, args.n_bin_per_dim).set_max_horizon(args.horizon);
        Box::new(rtdp)
    } else {
        let h = ZeroHeuristic {};
        let rtdp = RTDP_OAMDP::new(oamdp, h, args.n_bin_per_dim).set_max_horizon(args.horizon);
        Box::new(rtdp)
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    println!("{:?}", args);
    let mut rng = thread_rng();

    let start = Instant::now();
    let mut rtdp = match args.domain.as_str() {
        "baker" => build_rtdp(
            &args,
            BakerOAMDPBuilder::<3>::new(args.id).set_horizon(args.horizon),
        ),
        "baker_com" => build_rtdp(
            &args,
            BakerCOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        ),
        "blocks" => build_rtdp_rtdp(
            &args,
            BlocksOAMDPBuilder::new4_2(args.id).set_horizon(args.horizon),
        ),
        "blocks4_3" => build_rtdp_rtdp(
            &args,
            BlocksOAMDPBuilder::new4_3(args.id).set_horizon(args.horizon),
        ),
        "blocks6" => build_rtdp_rtdp(
            &args,
            BlocksOAMDPBuilder::new6_2(args.id).set_horizon(args.horizon),
        ),
        "baker5" => build_rtdp(
            &args,
            BakerOAMDPBuilder::<5>::new(args.id).set_horizon(args.horizon),
        ),
        "baker5_com" => build_rtdp(
            &args,
            BakerCOAMDPBuilder::new5(args.id).set_horizon(args.horizon),
        ),
        "spelling" => build_rtdp_rtdp(
            &args,
            SpellingOAMDPBuilder::new_rtdp(args.id).set_horizon(args.horizon),
        ),
        //         "spelling" => build_rtdp(
        //             &args,
        //             SpellingOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        //         ),
        "spelling_com" => build_rtdp(
            &args,
            SpellingCOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        ),
        "reset" => build_rtdp(
            &args,
            BakerResetOAMDPBuilder::<3>::new(args.id).set_horizon(args.horizon),
        ),
        "reset5" => build_rtdp(
            &args,
            BakerResetOAMDPBuilder::<5>::new(args.id).set_horizon(args.horizon),
        ),
        "recycle" => build_rtdp(&args, RecycleCOAMDPBuilder::new(args.id)),
        _ => panic!("{} not implemented", args.domain.as_str()),
    };

    if args.lrtdp {
        rtdp.lrtdp(args.num_trials, &mut rng);
    } else {
        rtdp.rtdp(args.num_trials, &mut rng);
    }
    let end = Instant::now();

    let result = monte_carlo_evaluation(&mut *rtdp, &mut rng, args.n);

    rtdp.run_episode(&mut rng);

    println!("Legibility Cost: {:.2?}", result);
    println!("Elapsed time: {:.2?}s", (end - start).as_secs_f32());
    println!("Num States: {}", rtdp.num_states());
    println!("Num Domain States: {}", rtdp.num_domain_states());
    println!("Root Value: {:.2?}", rtdp.root_value());
}
