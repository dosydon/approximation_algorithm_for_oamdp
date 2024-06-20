#![feature(trait_upcasting)]
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Instant;

use clap::Parser;
use mcts::{Budget, MCTSTrait, MCTS};
use mdp::{
    episode_runner::monte_carlo_evaluation,
    finite_horizon_wrapper::FiniteHorizonWrapper,
    heuristic::ZeroHeuristic,
    into_inner::IntoInnerMost,
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Build, Cost, GetNextStateMut, InitialState,
        IsTerminal, PMass, PMassMut, StateEnumerable, StatesActions,
    },
    policy::{random_policy::RandomPolicy, tabular_policy::TabularPolicy},
    value_iteration::value_iteration_ssp,
};
use oamdp::{
    algorithms::mcts_split::{MCTSAM, MCTSMA},
    domain_evaluator::DomainEvaluator,
    domains::{
        baker_grid::{BakerCOAMDPBuilder, BakerOAMDPBuilder},
        baker_grid_reset::BakerResetOAMDPBuilder,
        blocks_world::BlocksOAMDPBuilder,
        recycle::RecycleCOAMDPBuilder,
        spelling::{SpellingCOAMDPBuilder, SpellingOAMDPBuilder},
    },
    oamdp::oamdp::OAMDP,
    policy::{RTDPOAMDPPolicy, RandomOAMDPPolicy, TabularOAMDPPolicy},
    traits::{DomainAction, Message},
};
use rand::thread_rng;
use rtdp::{rtdp::RTDP, rtdp_ensure_convergence_wrapper::RTDPEnsureConvergenceWrapper};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    domain: String,

    id: usize,

    budget: usize,

    #[arg(short, long, default_value_t = false)]
    display: bool,

    #[arg(short, long, default_value_t = false)]
    am_split: bool,

    #[arg(short, long, default_value_t = false)]
    ma_split: bool,

    #[arg(short, long, default_value_t = false)]
    use_random_policy: bool,

    #[arg(short, long, default_value_t = false)]
    full_rollouts: bool,

    #[arg(short, long, default_value_t = 10)]
    n: usize,

    #[arg(short, long, default_value_t = 1.0)]
    c: f32,

    #[arg(short, long, default_value_t = 13)]
    horizon: usize,

    #[arg(short, long, default_value_t = 10)]
    num_rollouts: usize,

    #[arg(short, long)]
    lookahead: Option<usize>,
}

fn build_mcts<B, OM, M, A: Eq + Hash + Debug + Copy + Clone, const N: usize>(
    args: &Args,
    builder: B,
) -> Box<dyn MCTSTrait>
where
    B: Build<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>>,
    M: 'static
        + InitialState
        + IsTerminal
        + ActionEnumerable
        + StateEnumerable
        + StatesActions
        + Cost
        + PMass<f32>
        + ActionAvailability,
    OM: 'static,
    A: 'static,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, RandomOAMDPPolicy>: MCTSTrait,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, DomainEvaluator<RandomPolicy>>: MCTSTrait,
    MCTS<
        FiniteHorizonWrapper<OAMDP<OM, M, A, N>>,
        DomainEvaluator<TabularPolicy<M::State, M::Action>>,
    >: MCTSTrait,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, TabularOAMDPPolicy<M>>: MCTSTrait,
{
    let oamdp = builder.build();

    if args.use_random_policy {
        if args.full_rollouts {
            let policy = RandomOAMDPPolicy::new();
            let mcts = MCTS::new(oamdp, policy);

            Box::new(mcts)
        } else {
            let policy = DomainEvaluator::new(RandomPolicy {});

            let mcts = MCTS::new(oamdp, policy);

            Box::new(mcts)
        }
    } else {
        let mdp = oamdp.into_inner_most();
        //         let mdp = &oamdp.mdp.mdp;
        let value_table = value_iteration_ssp(mdp);
        let tabular_policy = TabularPolicy::from_value_table_ssp(mdp, &value_table);

        if args.full_rollouts {
            let policy = TabularOAMDPPolicy::new(tabular_policy);

            let mcts = MCTS::new(oamdp, policy);

            Box::new(mcts)
        } else {
            let policy = DomainEvaluator::new(tabular_policy);

            let mcts = MCTS::new(oamdp, policy);

            Box::new(mcts)
        }
    }
}

fn build_mcts_com<B, OM, M, A: Eq + Hash + Debug + Copy + Clone, const N: usize>(
    args: &Args,
    builder: B,
) -> Box<dyn MCTSTrait>
where
    B: Build<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>>,
    M: 'static
        + InitialState
        + IsTerminal
        + ActionEnumerable
        + StateEnumerable
        + StatesActions
        + Cost
        + PMass<f32>
        + ActionAvailability,
    OM: 'static,
    A: 'static,
    OAMDP<OM, M, A, N>: Message + DomainAction,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, RandomOAMDPPolicy>: MCTSTrait,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, DomainEvaluator<RandomPolicy>>: MCTSTrait,
    MCTS<
        FiniteHorizonWrapper<OAMDP<OM, M, A, N>>,
        DomainEvaluator<TabularPolicy<M::State, M::Action>>,
    >: MCTSTrait,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, TabularOAMDPPolicy<M>>: MCTSTrait,
    MCTSMA<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, RandomOAMDPPolicy>: MCTSTrait,
    MCTSMA<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, DomainEvaluator<RandomPolicy>>: MCTSTrait,
    MCTSMA<
        FiniteHorizonWrapper<OAMDP<OM, M, A, N>>,
        DomainEvaluator<TabularPolicy<M::State, M::Action>>,
    >: MCTSTrait,
    MCTSMA<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, TabularOAMDPPolicy<M>>: MCTSTrait,
    MCTSAM<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, TabularOAMDPPolicy<M>>: MCTSTrait,
    MCTSAM<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, RandomOAMDPPolicy>: MCTSTrait,
    MCTSAM<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, DomainEvaluator<RandomPolicy>>: MCTSTrait,
    MCTSAM<
        FiniteHorizonWrapper<OAMDP<OM, M, A, N>>,
        DomainEvaluator<TabularPolicy<M::State, M::Action>>,
    >: MCTSTrait,
{
    let oamdp = builder.build();

    if args.use_random_policy {
        if args.full_rollouts {
            let policy = RandomOAMDPPolicy::new();
            if args.am_split {
                Box::new(MCTSAM::new(oamdp, policy))
            } else if args.ma_split {
                Box::new(MCTSMA::new(oamdp, policy))
            } else {
                Box::new(MCTS::new(oamdp, policy))
            }
        } else {
            let policy = DomainEvaluator::new(RandomPolicy {});

            if args.am_split {
                Box::new(MCTSAM::new(oamdp, policy))
            } else if args.ma_split {
                Box::new(MCTSMA::new(oamdp, policy))
            } else {
                Box::new(MCTS::new(oamdp, policy))
            }
        }
    } else {
        let mdp = oamdp.into_inner_most();
        //         let mdp = &oamdp.mdp.mdp;
        let value_table = value_iteration_ssp(mdp);
        let tabular_policy = TabularPolicy::from_value_table_ssp(mdp, &value_table);
        if args.full_rollouts {
            let policy = TabularOAMDPPolicy::new(tabular_policy);
            if args.am_split {
                Box::new(MCTSAM::new(oamdp, policy))
            } else if args.ma_split {
                Box::new(MCTSMA::new(oamdp, policy))
            } else {
                Box::new(MCTS::new(oamdp, policy))
            }
        } else {
            let policy = DomainEvaluator::new(tabular_policy);
            if args.am_split {
                Box::new(MCTSAM::new(oamdp, policy))
            } else if args.ma_split {
                Box::new(MCTSMA::new(oamdp, policy))
            } else {
                Box::new(MCTS::new(oamdp, policy))
            }
        }
    }
}

fn build_mcts_state_not_enumerable<B, OM, M, A: Eq + Hash + Debug + Copy + Clone, const N: usize>(
    args: &Args,
    builder: B,
) -> Box<dyn MCTSTrait>
where
    B: Build<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>>,
    M: 'static
        + InitialState
        + IsTerminal
        + ActionEnumerable
        + StatesActions
        + GetNextStateMut
        + Cost
        + PMassMut<f32>
        + ActionAvailability,
    OM: 'static,
    A: 'static,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, RandomOAMDPPolicy>: MCTSTrait,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, DomainEvaluator<RandomPolicy>>: MCTSTrait,
    MCTS<
        FiniteHorizonWrapper<OAMDP<OM, M, A, N>>,
        DomainEvaluator<RTDPEnsureConvergenceWrapper<M::State, ZeroHeuristic>>,
    >: MCTSTrait,
    MCTS<FiniteHorizonWrapper<OAMDP<OM, M, A, N>>, RTDPOAMDPPolicy<M::State, ZeroHeuristic>>:
        MCTSTrait,
{
    let mut oamdp = builder.build();

    if args.use_random_policy {
        if args.full_rollouts {
            let policy = RandomOAMDPPolicy::new();
            let mcts = MCTS::new(oamdp, policy);

            Box::new(mcts)
        } else {
            let policy = DomainEvaluator::new(RandomPolicy {});

            let mcts = MCTS::new(oamdp, policy);

            Box::new(mcts)
        }
    } else {
        let mut lrtdp = RTDP::new(ZeroHeuristic {});
        lrtdp.lrtdp(&mut oamdp.mdp.mdp, 0, &mut thread_rng(), 1e-3);

        if args.full_rollouts {
            let policy = RTDPOAMDPPolicy::new(lrtdp);

            let mcts = MCTS::new(oamdp, policy);

            Box::new(mcts)
        } else {
            let lrtdp = RTDPEnsureConvergenceWrapper::new(lrtdp, 1e-3);
            let policy = DomainEvaluator::new(lrtdp);

            let mcts = MCTS::new(oamdp, policy);

            Box::new(mcts)
        }
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    println!("{:?}", args);

    let mut rng = thread_rng();

    let start = Instant::now();
    let mut mcts = match args.domain.as_str() {
        "baker" => build_mcts(
            &args,
            BakerOAMDPBuilder::<3>::new(args.id).set_horizon(args.horizon),
        ),
        "blocks" => build_mcts_state_not_enumerable(
            &args,
            BlocksOAMDPBuilder::new4_2(args.id).set_horizon(args.horizon),
        ),
        "blocks4_3" => build_mcts_state_not_enumerable(
            &args,
            BlocksOAMDPBuilder::new4_3(args.id).set_horizon(args.horizon),
        ),
        "blocks6" => build_mcts_state_not_enumerable(
            &args,
            BlocksOAMDPBuilder::new6_2(args.id).set_horizon(args.horizon),
        ),
        "reset" => build_mcts(
            &args,
            BakerResetOAMDPBuilder::<3>::new(args.id).set_horizon(args.horizon),
        ),
        "reset5" => build_mcts(
            &args,
            BakerResetOAMDPBuilder::<5>::new(args.id).set_horizon(args.horizon),
        ),
        "baker_com" => build_mcts_com(
            &args,
            BakerCOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        ),
        "baker5" => build_mcts(
            &args,
            BakerOAMDPBuilder::<5>::new(args.id).set_horizon(args.horizon),
        ),
        "baker5_com" => build_mcts_com(
            &args,
            BakerCOAMDPBuilder::new5(args.id).set_horizon(args.horizon),
        ),
        "spelling" => build_mcts_state_not_enumerable(
            &args,
            SpellingOAMDPBuilder::new_rtdp(args.id).set_horizon(args.horizon),
        ),
        "spelling_com" => build_mcts_com(
            &args,
            SpellingCOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        ),
        "recycle" => build_mcts_com(
            &args,
            RecycleCOAMDPBuilder::new(args.id).set_horizon(args.horizon),
        ),
        _ => panic!("{} not implemented", args.domain.as_str()),
    };
    mcts.set_budget(Budget::NumIterations(args.budget));
    mcts.set_c(args.c);
    mcts.set_num_rollouts(args.num_rollouts);
    mcts.set_lookahead(args.lookahead);

    let result = if args.n > 1 {
        monte_carlo_evaluation(&mut *mcts, &mut rng, args.n)
    } else {
        mcts.run_episode(&mut rng)
    };
    let end = Instant::now();

    println!("Legibility Cost: {:.2?}", result);
    println!(
        "Elapsed time: {:.2?}s",
        (end - start).as_secs_f32() / (args.n as f32)
    );
}
