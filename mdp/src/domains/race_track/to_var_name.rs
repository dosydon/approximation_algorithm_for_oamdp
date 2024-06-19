use crate::mdp_traits::ToVarName;
use crate::race_track::RaceTrackState;

fn format_int(i: i32) -> String {
    if i >= 0 {
        format!("{}", i)
    } else {
        format!("m{}", -i)
    }
}

impl ToVarName for RaceTrackState {
    fn to_var_name(&self) -> String {
        match self {
            RaceTrackState::Dummy => "dummy".to_string(),
            RaceTrackState::Wrapper(inner) => {
                format!(
                    "x{}y{}dx{}dy{}",
                    inner.x,
                    inner.y,
                    format_int(inner.dx),
                    format_int(inner.dy)
                )
            }
        }
    }
}
