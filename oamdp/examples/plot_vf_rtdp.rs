use std::collections::HashMap;

use mdp::baker_grid::baker_factory;
use mdp::baker_grid::*;
use mdp::heuristic::ZeroHeuristic;
use oamdp::algorithms::regular_grid_belief_points::RegularGridBeliefPoints;
use oamdp::algorithms::rtdp::RTDPTrait;
use oamdp::algorithms::rtdp::RTDP_OAMDP;
use oamdp::algorithms::AssocBeliefPointN;
use oamdp::belief_cost_function::{self, Objective};
use oamdp::belief_update_type::ObserveabilityAssumption::*;
use oamdp::oamdp::oamdp::OAMDP2;

use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;
use oamdp::observer_model::SoftmaxModel;
use rand::thread_rng;

fn main() {
    env_logger::init();
    let mut rng = thread_rng();
    let pair = baker_factory("Tiny2");
    let partial_mdp = pair.0;
    let goals = [pair.1[0], pair.1[1]];
    println!("{:?}", goals);

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

    let softmax_policy = SoftmaxPolicyBuilder::new(0.3);
    let oamdp = OAMDP2::<SoftmaxModel<BakerGridMDP, 2>, BakerGridMDP>::new_implicit_model(
        &partial_mdp,
        &softmax_policy,
        goals,
        1,
        belief_cost_function::BeliefCostType::TVDistance,
        Objective::LinearCombination(1.0, 1.0),
        OnlyActionsAreConsidered,
    );
    let n_bin_per_dim = 16;
    let h = ZeroHeuristic {};
    //     let mut rtdp: RTDPLinearInterpolation<BakerGridState, BakerGridAction, ZeroHeuristic, 2> =
    //         RTDPLinearInterpolation::new(h, RTDPGridResolution::Variable(n_bin_per_dim));
    let mut rtdp = RTDP_OAMDP::new(oamdp, h, n_bin_per_dim);
    rtdp.rtdp(1000, &mut rng);
    let vf = rtdp.to_grid_vf();

    for i in 0..rtdp.oamdp_d.oamdp.mdp.height() {
        for j in 0..rtdp.oamdp_d.oamdp.mdp.width() {
            let s = BakerGridState::new(i as i32, j as i32);
            if let Some(grbp) = vf.table.get(&s) {
                println!("{:?}", grbp);
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
}
