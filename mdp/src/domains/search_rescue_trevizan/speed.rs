use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Deserialize, Serialize)]
pub enum Speed {
    Low,
    Medium,
    High,
}