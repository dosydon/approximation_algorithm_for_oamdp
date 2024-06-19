use crate::domains::search_rescue_trevizan::cell_status::CellStatus;
use super::speed::Speed;
use serde::{Deserialize, Serialize};
use super::map_configuration::MapConfiguration;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct SRState<const N: usize> {
    pub configuration: MapConfiguration<N>,
    pub boarded: bool,
    pub speed: Speed,
}

impl<const N: usize> SRState<N> {
    pub fn new(cells: [[CellStatus; N]; N], agent_pos: (i32, i32), boarded: bool, speed: Speed) -> Self {
        Self {
            configuration: MapConfiguration { cells, agent_pos },
            boarded: boarded,
            speed: speed,
        }
    }
}
