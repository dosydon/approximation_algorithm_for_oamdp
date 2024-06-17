use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BakerCommunicationAction {
    Blue,
    Circle,
    Green,
    None,
    Square,
}
