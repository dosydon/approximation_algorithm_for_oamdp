use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::grid_turning::{GridTurningAction, GridTurningState};

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct GridTurningPair {
    pub s: GridTurningState,
    a: Option<GridTurningAction>,
}

impl GridTurningPair {
    pub fn new(s: GridTurningState, a: Option<GridTurningAction>) -> GridTurningPair {
        GridTurningPair { s: s, a: a }
    }
}

#[wasm_bindgen]
impl GridTurningPair {
    pub fn has_action(&self) -> bool {
        if let Some(_a) = self.a {
            true
        } else {
            false
        }
    }
    pub fn get_action(&self) -> GridTurningAction {
        self.a.unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct GridTurningPairs {
    vec: Vec<GridTurningPair>,
}

impl GridTurningPairs {
    pub fn new(vec: Vec<GridTurningPair>) -> GridTurningPairs {
        GridTurningPairs { vec: vec }
    }
}

#[wasm_bindgen]
impl GridTurningPairs {
    pub fn size(&self) -> usize {
        self.vec.len()
    }

    pub fn at(&self, index: usize) -> GridTurningPair {
        self.vec[index as usize].clone()
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    pub fn from_yaml(yaml: &str) -> GridTurningPair {
        serde_yaml::from_str(yaml).unwrap()
    }
}
