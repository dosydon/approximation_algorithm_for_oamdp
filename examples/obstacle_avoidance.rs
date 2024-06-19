use mdp::{
    episode_runner::CostEpisodeIterator,
    mdp_traits::{BuildFrom, InitialState},
    policy::tabular_policy::TabularPolicy,
    simple_av::VehicleConfiguration,
    simple_av_obstacle_avoidance::VehicleConfigurationLane,
    simple_av_obstacle_avoidance::{Lane, ObstacleAvoidanceParameter::*},
    simple_av_obstacle_avoidance::{ObstacleAvoidanceBuilder, ObstacleAvoidanceState},
    value_iteration::value_iteration_ssp,
};
use rand::thread_rng;

fn main() {
    let start = ObstacleAvoidanceState::new(
        VehicleConfigurationLane::new(0, 3, Lane::Center),
        VehicleConfiguration::new(0, 2),
    );
    let builder = ObstacleAvoidanceBuilder::new(30, 4)
        .set_collision_zone(12, 18)
        .set_start_state(start);
    let mdp = builder.build_from(&AwareNotYielding);
    let value_table = value_iteration_ssp(&mdp);
    let policy = TabularPolicy::from_value_table_ssp(&mdp, &value_table);

    println!("{:?}", value_table.get_value(&mdp.initial_state()));
    let mut rng = thread_rng();
    for (s, _, _, c) in CostEpisodeIterator::from_initial_state(&mdp, &policy, &mut rng) {
        println!("{:?}", s);
        println!("{:?}", c);
    }
}
