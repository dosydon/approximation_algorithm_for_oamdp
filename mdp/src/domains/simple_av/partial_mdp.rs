use crate::simple_av::SimpleAVState;

pub struct SimpleAVPartialMDP {
    pub(in crate::domains::simple_av) miny: usize,
    pub(in crate::domains::simple_av) maxy: usize,
    pub(in crate::domains::simple_av) mindy: i32,
    pub(in crate::domains::simple_av) maxdy: i32,
    pub(in crate::domains::simple_av) start: SimpleAVState,
}

impl SimpleAVPartialMDP {
    pub fn new(
        miny: usize,
        maxy: usize,
        mindy: i32,
        maxdy: i32,
        start: SimpleAVState,
    ) -> SimpleAVPartialMDP {
        SimpleAVPartialMDP {
            miny,
            maxy,
            mindy,
            maxdy,
            start: start,
        }
    }
}