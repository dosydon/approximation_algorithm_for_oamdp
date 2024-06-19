#[derive(PartialEq, Debug, Clone, Copy)]
pub(in crate) enum GridStatus {
    Blank,
    Start,
    Goal,
    Wall,
    Watery,
}
