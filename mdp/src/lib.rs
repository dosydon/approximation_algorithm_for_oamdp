#![feature(associated_type_defaults)]
extern crate assert_approx_eq;
extern crate bimap;
extern crate itertools;
extern crate petgraph;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate strum;
extern crate strum_macros;

pub mod algorithms;
pub mod common;
mod domains;
pub mod episode_runner;
pub mod heuristic;
pub mod into_inner;
pub mod mdp_traits;
pub mod policy;
pub mod value_estimator;
mod wrapper;

pub use algorithms::policy_evaluation;
pub use algorithms::value_iteration;
pub use common::arena;
pub use common::state_queue;
pub use domains::*;
pub use wrapper::cache_wrapper;
pub use wrapper::finite_horizon_wrapper;
pub use wrapper::state_enumerable_wrapper;
