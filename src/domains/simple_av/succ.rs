pub fn usize_succ_bound(x: usize, dx: i32, ub: usize) -> usize {
    if (x as i32) + dx <= 0 {
        0
    } else {
        if ((x as i32) + dx) as usize > ub {
            ub
        } else {
            ((x as i32) + dx) as usize
        }
    }
}

pub fn i32_succ_bound(dx: i32, ddx: i32, lb: i32, ub: i32) -> i32 {
    if dx + ddx <= lb {
        lb
    } else {
        if dx + ddx >= ub {
            ub
        } else {
            dx + ddx
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_usize_succ_bound() {
        assert_eq!(2, usize_succ_bound(4, -1, 2));
        assert_eq!(0, usize_succ_bound(0, -1, 3));
        assert_eq!(0, usize_succ_bound(2, -4, 3));
    }
}
