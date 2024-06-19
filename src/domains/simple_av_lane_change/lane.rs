use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Deserialize, Serialize, EnumIter)]
pub enum Lane {
    LeftLeft,
    LeftCenter,
    LeftRight,
    RightLeft,
    RightCenter,
    RightRight,
}
