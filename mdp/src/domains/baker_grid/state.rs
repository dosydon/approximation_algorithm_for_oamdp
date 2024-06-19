use crate::common::coordinate2::Coordinate2;

pub type BakerGridState = Coordinate2;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serde_grid_state() {
        let state = BakerGridState::new(3, 3);
        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: BakerGridState = serde_json::from_str(&serialized).unwrap();
        assert_eq!(state, deserialized);
    }
}
