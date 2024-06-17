use oamdp::domains::recycle::RecycleCOAMDPBuilder;
use rand::prelude::*;
use std::fs;

fn main() {
    let mut rng = thread_rng();

    for i in 50..80 {
        let builder = RecycleCOAMDPBuilder::<5, 4>::random_instance(&mut rng);
        let yaml = serde_yaml::to_string(&builder).unwrap();
        println!("{}", yaml);
        fs::write(format!("recycle_{}.yaml", i), yaml).expect("Unable to read file");
    }
}
