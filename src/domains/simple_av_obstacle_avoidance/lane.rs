use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize, EnumIter)]
pub enum Lane {
    Left,
    Center,
    Right,
}
