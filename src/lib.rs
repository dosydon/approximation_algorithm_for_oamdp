#![feature(trait_upcasting)]
extern crate arraymap;
extern crate assert_approx_eq;
extern crate bson;
extern crate itertools;
extern crate num_traits;
extern crate ordered_float;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

pub mod algorithms;
pub mod belief_cost_function;
//mod bin;
pub mod domains;
#[macro_use]
pub mod oamdp;
pub mod belief_update_type;
pub mod domain_evaluator;
pub mod oamdp_d;
pub mod observer_model;
pub mod plot_belief_changes;
// pub mod poamdp;
pub mod policy;
pub mod regular_grid_translator;
pub mod scaled_rtdp;
pub mod scaled_value_table;
pub mod traits;
