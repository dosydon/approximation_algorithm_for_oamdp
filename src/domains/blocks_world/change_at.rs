use super::Location;


pub(in crate::domains::blocks_world) fn change_at<const N: usize>(mut locations: [Location; N], i: usize, l: Location) -> [Location; N] {
    locations[i] = l;

    locations
}
