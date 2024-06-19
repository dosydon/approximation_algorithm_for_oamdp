#[derive(PartialEq, Debug, Clone)]
pub struct Grid2D {
    pub height: usize,
    pub width: usize,
    pub(crate) is_obstacled: Vec<Vec<bool>>,
}

impl Grid2D {
    pub fn new(height: usize, width: usize, is_obstacled: Vec<Vec<bool>>) -> Self {
        Grid2D {
            height: height,
            width: width,
            is_obstacled: is_obstacled,
        }
    }

    pub(crate) fn within_bound(&self, i: i32, j: i32) -> bool {
        (0 <= i) && (i < self.height as i32) && (j >= 0) && (j < self.width as i32)
    }

    fn is_obstacled(&self, i: i32, j: i32) -> bool {
        self.is_obstacled[i as usize][j as usize]
    }

    pub(crate) fn is_valid_cordinate(&self, i: i32, j: i32) -> bool {
        if !self.within_bound(i, j) {
            false
        } else if self.is_obstacled(i, j) {
            false
        } else {
            true
        }
    }
}
