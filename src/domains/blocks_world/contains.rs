use super::Location;

pub(in crate::domains::blocks_world) fn contains<const N: usize>(locations: &[Location; N], l: Location) -> bool {
    let mut flag = false;
    for i in 0..N {
        if locations[i] == l {
            flag = true;
        }
    }

    flag
}
