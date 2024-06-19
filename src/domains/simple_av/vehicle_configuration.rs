use crate::into_inner::InnerMost;
use crate::simple_av::action::action_to_ddy;
use crate::simple_av::succ::*;
use crate::simple_av::SimpleAVAction;
use mdp_derive::InnerMost;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, InnerMost)]
pub struct VehicleConfiguration {
    pub y: usize,
    pub dy: i32,
}

impl VehicleConfiguration {
    pub fn new(y: usize, dy: i32) -> VehicleConfiguration {
        VehicleConfiguration { y, dy }
    }

    pub fn next(
        self,
        a: &SimpleAVAction,
        maxy: usize,
        mindy: i32,
        maxdy: i32,
    ) -> VehicleConfiguration {
        let ddy = action_to_ddy(a);
        VehicleConfiguration::new(
            usize_succ_bound(self.y, self.dy, maxy),
            i32_succ_bound(self.dy, ddy, mindy, maxdy),
        )
    }
}
