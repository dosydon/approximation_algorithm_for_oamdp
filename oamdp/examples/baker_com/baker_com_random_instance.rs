use oamdp::{belief_cost_function::BeliefCostType, domains::baker_grid::BakerCOAMDPBuilder};
use rand::prelude::*;
use std::fs;

fn main() {
    let mut rng = thread_rng();

    //     for i in 200..230 {
    for i in 300..330 {
        //     for i in 10..20 {
        let builder = BakerCOAMDPBuilder::random_instance5(
            13,
            13,
            vec![
                (0, 6),
                (1, 6),
                (2, 6),
                (4, 6),
                (5, 6),
                (6, 6),
                (7, 6),
                (8, 6),
                (10, 6),
                (11, 6),
                (12, 6),
                (6, 0),
                (6, 1),
                (6, 2),
                (6, 4),
                (6, 5),
                (6, 7),
                (6, 8),
                (6, 10),
                (6, 11),
                (6, 12),
            ],
            10,
            BeliefCostType::Disimulation,
            //             BeliefCostType::TVDistance,
            &mut rng,
        );
        //         let builder = BakerCOAMDPBuilder::new(1);
        let yaml = serde_yaml::to_string(&builder).unwrap();
        println!("{}", yaml);
        fs::write(format!("baker_{}.yaml", i), yaml).expect("Unable to read file");
    }
}
