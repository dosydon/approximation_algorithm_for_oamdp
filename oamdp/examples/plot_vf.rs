use std::collections::HashMap;

use mdp::baker_grid::baker_factory;
use mdp::baker_grid::*;
use mdp::episode_runner::EpisodeRunner;
use mdp::into_inner::Inner;
use mdp::mdp_traits::DisplayState;
use mdp::mdp_traits::InitialState;
use oamdp::algorithms::grid_based_value_iteration::grid_based_value_iteration_ssp;
use oamdp::algorithms::regular_grid_belief_points::RegularGridBeliefPoints;
use oamdp::algorithms::AssocBeliefPointN;
use oamdp::belief_cost_function::{self, Objective};
use oamdp::belief_update_type::ObserveabilityAssumption::*;
use oamdp::oamdp::oamdp::OAMDP2;

use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;
use oamdp::observer_model::SoftmaxModel;
use rand::thread_rng;

fn main() {
    let color_map = HashMap::from([
        (Some(BakerGridAction::NorthWest), "blue".to_string()),
        (Some(BakerGridAction::SouthWest), "black".to_string()),
        (Some(BakerGridAction::SouthEast), "cyan".to_string()),
        (Some(BakerGridAction::NorthEast), "purple".to_string()),
        (Some(BakerGridAction::North), "orange".to_string()),
        (Some(BakerGridAction::West), "red".to_string()),
        (Some(BakerGridAction::South), "grey".to_string()),
        (Some(BakerGridAction::East), "olive".to_string()),
        (Some(BakerGridAction::Stay), "pink".to_string()),
        (None, "green".to_string()),
    ]);
    let pair = baker_factory("Tiny2");
    let partial_mdp = pair.0;
    let goals = [pair.1[0], pair.1[1]];
    println!("{:?}", goals);

    let softmax_policy = SoftmaxPolicyBuilder::new(0.3);
    let mut oamdp = OAMDP2::<SoftmaxModel<BakerGridMDP, 2>, BakerGridMDP>::new_implicit_model(
        &partial_mdp,
        &softmax_policy,
        goals,
        1,
        belief_cost_function::BeliefCostType::Euclidean,
        Objective::LinearCombination(1.0, 1.0),
        OnlyActionsAreConsidered,
    );
    let n_bin_per_dim = 32;

    let v = grid_based_value_iteration_ssp(&mut oamdp, n_bin_per_dim);
    for i in 0..oamdp.mdp.height() {
        for j in 0..oamdp.mdp.width() {
            let s = BakerGridState::new(i as i32, j as i32);
            if let Some(grbp) = v.table.get(&s) {
                grbp.plot(
                    &format!("test_{}_{}.png", i, j),
                    &format!("{} {}", i, j),
                    0.0,
                    20.0,
                    &color_map,
                );
            } else {
                RegularGridBeliefPoints::<AssocBeliefPointN<BakerGridAction, 2>, 2>::new(
                    n_bin_per_dim,
                )
                .plot(
                    &format!("test_{}_{}.png", i, j),
                    &format!("{} {}", i, j),
                    0.0,
                    20.0,
                    &color_map,
                );
            }
        }
    }

    let grid = oamdp.mdp.grid2d.clone();
    let grid_and_goals = GridAndGoals::new(
        grid,
        vec![
            (goals[0].i as usize, goals[0].j as usize),
            (goals[1].i as usize, goals[1].j as usize),
        ],
        vec!["A".to_string(), "B".to_string()],
    );

    let mut rng = thread_rng();
    let mut runner = EpisodeRunner::new(&oamdp, &v, oamdp.initial_state());
    for (s, _a, _, _) in runner.into_iter_with(&mut rng) {
        grid_and_goals.display(&s.inner());
        println!("");
    }
}
