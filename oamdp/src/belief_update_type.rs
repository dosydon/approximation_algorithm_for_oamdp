use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ObserveabilityAssumption {
    ActionObservable,
    ActionNotObservable,
    OnlyActionsAreConsidered,
}
