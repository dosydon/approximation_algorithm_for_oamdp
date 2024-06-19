use crate::domains::search_rescue_trevizan::CellStatus::*;
use crate::domains::search_rescue_trevizan::SRState;
use crate::mdp_traits::ToVarName;

impl<const N: usize> ToVarName for SRState<N> {
    fn to_var_name(&self) -> String {
        let mut result = format!(
            "i{}j{}{}{:?}",
            self.configuration.agent_pos.0,
            self.configuration.agent_pos.1,
            self.boarded,
            self.speed
        );
        for i in 0..N {
            for j in 0..N {
                result += match self.configuration.cells[i][j] {
                    Survivor => "s",
                    NoSurvivor => "n",
                    ProbLow => "l",
                    ProbMedium => "m",
                    ProbHigh => "h",
                };
            }
        }
        result
    }
}
