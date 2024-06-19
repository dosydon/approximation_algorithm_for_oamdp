use crate::domains::search_rescue_trevizan::SRMDP;
use serde::{Deserialize, Serialize};
use std::fs;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub enum SRMDPD {
    SRMDP2(SRMDP<2>),
    SRMDP3(SRMDP<3>),
    SRMDP4(SRMDP<4>),
    SRMDP5(SRMDP<5>),
}

impl SRMDPD {
    pub fn write(&self, filename: &str) {
        let data = serde_json::to_string(self).unwrap();
        fs::write(filename, data).expect("Unable to write file");
    }
    pub fn from_file(name: &str) -> SRMDPD {
        let data = fs::read_to_string(name).expect("Unable to read file");
        serde_json::from_str(&data).unwrap()
    }
}