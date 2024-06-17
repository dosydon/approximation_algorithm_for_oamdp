#[derive(Clone, Copy)]
pub struct AV1dRange {
    pub(crate) miny: usize,
    pub(crate) maxy: usize,
    pub(crate) mindy: i32,
    pub(crate) maxdy: i32,
}

impl AV1dRange {
    pub fn new(miny: usize, maxy: usize, mindy: i32, maxdy: i32) -> Self {
        AV1dRange {
            miny: miny,
            maxy: maxy,
            mindy: mindy,
            maxdy: maxdy,
        }
    }
}
