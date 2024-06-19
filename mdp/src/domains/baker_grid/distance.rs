use super::BakerGridState;

pub(in crate::domains::baker_grid) fn distance(s: &BakerGridState, ss: &BakerGridState) -> f32 {
    (((s.i - ss.i) * (s.i - ss.i) + (s.j - ss.j) * (s.j - ss.j)) as f32).sqrt()
}
