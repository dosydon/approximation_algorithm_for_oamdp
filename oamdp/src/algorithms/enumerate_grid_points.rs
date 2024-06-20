pub(crate) fn enumerate_grid_points<const N: usize>(k: usize) -> Vec<[usize; N]> {
    let mut result = Vec::new();
    for i in 0..=k {
        let mut v = [k; N];
        v[N - 1] = i;
        result.push(v);
    }
    for i in (1..(N - 1)).rev() {
        let mut new_result = Vec::new();
        for v in result.iter() {
            for j in v[i + 1]..=k {
                let mut new_v = *v;
                new_v[i] = j;
                new_result.push(new_v);
            }
        }
        result = new_result;
    }
    result
}
