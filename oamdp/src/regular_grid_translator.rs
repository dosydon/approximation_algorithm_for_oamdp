use core::fmt::Debug;
use ordered_float::*;
use std::cmp::Ordering::*;

use crate::num_traits::FromPrimitive;
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct RegularGridTranslator<const N: usize> {
    pub(crate) num_bin_per_dim: usize,
}

impl<const N: usize> RegularGridTranslator<N> {
    pub fn new(num_bin_per_dim: usize) -> Self {
        RegularGridTranslator { num_bin_per_dim }
    }

    pub(crate) fn find_corners_of_subsimplex(&self, b: &[NotNan<f32>]) -> [[usize; N]; N] {
        let mut result = [[0; N]; N];

        let mut v = self.b_to_v(&b);
        let p = self.b_to_p(&b);
        /* println!("p: {:?}", p);  */
        /* let d = self.b_to_d(&b); */
        /* println!("d: {:?}", d);  */
        /* let x = self.b_to_x(&b); */
        /* println!("x: {:?}", x);  */

        result[0] = v;

        for i in 0..(N - 1) {
            if v[p[i] as usize] + 1 <= self.num_bin_per_dim {
                v[p[i] as usize] += 1;
            }
            result[i + 1] = v;
        }

        result
    }

    pub(crate) fn find_barycentric_coordinates(&self, b: &[NotNan<f32>]) -> [f32; N] {
        let mut result = [0.0; N];
        let mut sum = 0.0;
        let p = self.b_to_p(&b);
        let d = self.b_to_d(b);

        for i in 1..N {
            result[i] = d[p[i - 1]] - d[p[i]];
            sum += result[i];
        }
        result[0] = 1.0 - sum;

        self.normalize(result)
    }

    pub(crate) fn get_corner_and_lambdas(&self, b: &[NotNan<f32>]) -> Vec<([usize; N], f32)> {
        let corners = self.find_corners_of_subsimplex(&b);
        let baycentric_coordinates = self.find_barycentric_coordinates(&b);
        let pairs: Vec<_> = corners
            .iter()
            .cloned()
            .zip(baycentric_coordinates)
            .collect();

        pairs
    }

    fn normalize(&self, mut coord: [f32; N]) -> [f32; N] {
        let mut sum = 0.0;
        for i in 0..N {
            if coord[i] < 0.0 {
                coord[i] = 0.0;
            }
            sum += coord[i]
        }
        for i in 0..N {
            coord[i] = coord[i] / sum;
        }
        coord
    }

    fn b_to_p(&self, b: &[NotNan<f32>]) -> [usize; N] {
        let mut enu = [(0.0, 0); N];
        let mut result = [0; N];
        let d = self.b_to_d(b);
        for i in 0..N {
            enu[i] = (d[i], i);
        }
        enu.sort_by(|a, b| {
            if a.1 == 0 {
                Greater
            } else if b.1 == 0 {
                Less
            } else {
                b.0.partial_cmp(&a.0).unwrap()
            }
        });
        /* println!("{:?}", enu); */

        for i in 0..N {
            result[i] = enu[i].1;
        }
        result
    }

    fn b_to_d(&self, b: &[NotNan<f32>]) -> [f32; N] {
        let mut result = [0.0; N];
        let x = self.b_to_x(b);
        let v = self.x_to_v(x);

        for i in 0..N {
            result[i] = x[i] - (v[i] as f32);
        }
        result
    }

    fn b_to_x(&self, b: &[NotNan<f32>]) -> [f32; N] {
        let mut result = [0.0; N];
        for l in (0..N).rev() {
            result[l] = b[l].into_inner() * (self.num_bin_per_dim as f32);
            if l + 1 < N {
                result[l] += result[l + 1];
            }
        }

        result
    }

    fn x_to_v(&self, x: [f32; N]) -> [usize; N] {
        let mut result = [0; N];
        for i in 0..N {
            result[i] = x[i].floor() as usize;
        }
        result[0] = self.num_bin_per_dim;
        result
    }

    pub(crate) fn b_to_v(&self, b: &[NotNan<f32>]) -> [usize; N] {
        let x = self.b_to_x(b);
        //         println!("x: {:?}", x);
        self.x_to_v(x)
    }

    pub fn v_to_b(&self, v: &[usize]) -> [NotNan<f32>; N] {
        let mut result = [NotNan::from_f32(0.0).unwrap(); N];
        for l in (0..N).rev() {
            if l + 1 < N {
                result[l] =
                    NotNan::from_f32((v[l] - v[l + 1]) as f32 / (self.num_bin_per_dim as f32))
                        .unwrap();
            } else {
                result[l] = NotNan::from_f32(v[l] as f32 / (self.num_bin_per_dim as f32)).unwrap();
            }
        }
        result
    }

    //     pub(crate) fn discretize(&self, b: &[NotNan<f32>]) -> [NotNan<f32>; N] {
    //         let x = self.b_to_x(b);
    //         let v = self.x_to_v(x);
    //         self.v_to_b(&v)
    //     }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num_traits::FromPrimitive;

    #[test]
    fn test_b_to_x() {
        let b = [
            NotNan::<f32>::from_f32(0.4).unwrap(),
            NotNan::<f32>::from_f32(0.4).unwrap(),
            NotNan::<f32>::from_f32(0.2).unwrap(),
        ];
        let t = RegularGridTranslator::<3> { num_bin_per_dim: 2 };

        let x = t.b_to_x(&b);
        println!("{:?}", x);
        let v = t.x_to_v(x);
        assert_eq!(v, [2, 1, 0]);
        //         println!("{:?}", v);
        //         println!("{:?}", t.v_to_b(&v));
        let d = t.b_to_d(&b);
        println!("{:?}", d);
        let p = t.b_to_p(&b);
        println!("{:?}", p);
        //
        let mut result = [0.0; 3];
        for (v, lambda) in t
            .find_corners_of_subsimplex(&b)
            .iter()
            .zip(t.find_barycentric_coordinates(&b))
        {
            let v_b = t.v_to_b(v);
            println!("{:?} {:?} {}", v, v_b, lambda);
            for i in 0..3 {
                result[i] += lambda * v_b[i].into_inner();
            }
        }
        println!("{:?}", result);
    }

    #[test]
    fn test_to_v() {
        let b = [
            NotNan::<f32>::from_f32(0.27).unwrap(),
            NotNan::<f32>::from_f32(0.26).unwrap(),
            NotNan::<f32>::from_f32(0.47).unwrap(),
        ];
        let t = RegularGridTranslator::<3> {
            num_bin_per_dim: 20,
        };

        let x = t.b_to_x(&b);
        println!("{:?}", x);
        let v = t.x_to_v(x);
        assert_eq!(v, [20, 14, 9]);
        println!("{:?}", v);
        println!("{:?}", t.v_to_b(&v));
        let d = t.b_to_d(&b);
        println!("{:?}", d);
        let p = t.b_to_p(&b);
        println!("{:?}", p);

        let mut result = [0.0; 3];
        for (v, lambda) in t
            .find_corners_of_subsimplex(&b)
            .iter()
            .zip(t.find_barycentric_coordinates(&b))
        {
            let v_b = t.v_to_b(v);
            println!("{:?} {:?}", v, v_b);
            for i in 0..3 {
                result[i] += lambda * v_b[i].into_inner();
            }
        }
        println!("{:?}", result);
    }

    #[test]
    fn test_b_to_p() {
        let t = RegularGridTranslator::<3> {
            num_bin_per_dim: 20,
        };
        let b = [
            NotNan::<f32>::from_f32(0.27).unwrap(),
            NotNan::<f32>::from_f32(0.26).unwrap(),
            NotNan::<f32>::from_f32(0.47).unwrap(),
        ];
        let p = t.b_to_p(&b);
        println!("{:?}", p);
        assert_eq!(p, [1, 2, 0]);

        let b = [
            NotNan::<f32>::from_f32(0.7).unwrap(),
            NotNan::<f32>::from_f32(0.3).unwrap(),
            NotNan::<f32>::from_f32(0.0).unwrap(),
        ];
        let p = t.b_to_p(&b);
        println!("{:?}", p);
        println!("{:?}", t.find_corners_of_subsimplex(&b));
        println!("{:?}", t.find_barycentric_coordinates(&b));
    }

    #[test]
    fn test_find_corners_of_subsimplex() {
        let b = [
            NotNan::from_f32(0.28453165).unwrap(),
            NotNan::from_f32(0.17970422).unwrap(),
            NotNan::from_f32(0.34496406).unwrap(),
            NotNan::from_f32(0.1908002).unwrap(),
            NotNan::from_f32(0.0).unwrap(),
        ];
        let t = RegularGridTranslator::<5> {
            num_bin_per_dim: 64,
        };
        println!("{:?}", t.b_to_v(&b));

        for (v, _lambda) in t
            .find_corners_of_subsimplex(&b)
            .iter()
            .zip(t.find_barycentric_coordinates(&b))
        {
            let v_b = t.v_to_b(v);
            println!("{:?} {:?}", v, v_b);
        }
    }
}
