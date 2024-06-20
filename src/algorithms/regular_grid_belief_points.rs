use crate::belief_cost_function::l1_distance;
use crate::num_traits::FromPrimitive;
use core::fmt::Debug;
use std::f32::MAX;

use log::debug;
use ordered_float::*;
use std::collections::HashMap;

use crate::algorithms::belief_point::BeliefPoint;
// use crate::algorithms::belief_points::GenerateUniformGrid;
use crate::algorithms::enumerate_grid_points::enumerate_grid_points;
use crate::algorithms::AssocBeliefPointN;
use crate::regular_grid_translator::RegularGridTranslator;

#[derive(Clone, Debug)]
pub struct RegularGridBeliefPoints<B: BeliefPoint<N> + Clone + Copy + Debug, const N: usize> {
    pub grid: HashMap<[usize; N], B>,
    pub translator: RegularGridTranslator<N>,
}

impl<B: Copy + Debug + Clone + BeliefPoint<N>, const N: usize> RegularGridBeliefPoints<B, N> {
    pub fn new(n: usize) -> RegularGridBeliefPoints<B, N> {
        RegularGridBeliefPoints {
            grid: HashMap::new(),
            translator: RegularGridTranslator { num_bin_per_dim: n },
        }
    }

    pub(crate) fn push(&mut self, bp: B) {
        let b = bp.inner();
        let v = self.translator.b_to_v(&b);
        self.grid.insert(v, bp);
    }
}

impl<B: Copy + Debug + Clone + BeliefPoint<N>, const N: usize> RegularGridBeliefPoints<B, N> {
    pub fn num_bin_per_dim(&self) -> usize {
        self.translator.num_bin_per_dim
    }
}

impl<A: Debug + Copy + Clone, const N: usize> RegularGridBeliefPoints<AssocBeliefPointN<A, N>, N> {
    #[allow(dead_code)]
    pub(crate) fn find_closest_belief_point(
        &self,
        b: &[NotNan<f32>],
    ) -> Option<AssocBeliefPointN<A, N>> {
        let mut result = None;
        let mut best = MAX;

        for v in self.translator.find_corners_of_subsimplex(b) {
            let bb = self.translator.v_to_b(&v);
            let d = l1_distance(&b, &bb);
            if d < best {
                result = self.grid.get(&v).copied();
                best = d;
            }
        }
        result
    }

    pub(crate) fn get_value_convex_interpolation(&self, b: &[NotNan<f32>]) -> f32 {
        let corners = self.translator.find_corners_of_subsimplex(b);
        let baycentric_coordinates = self.translator.find_barycentric_coordinates(b);
        let mut value = 0.0;

        for (v, lambda) in corners.iter().zip(baycentric_coordinates) {
            if lambda <= 1e-5 {
                continue;
            }
            debug!(
                "{:?} {:?} {} {}",
                v,
                self.translator.v_to_b(v),
                lambda,
                self.grid.get(v).unwrap().assoc_value()
            );
            value += lambda
                * self
                    .grid
                    .get(v)
                    .expect(&format!(
                        "no grid point found {:?} {:?} {:?} {:?} {}",
                        b, v, self.translator.num_bin_per_dim, corners, lambda
                    ))
                    .assoc_value()
                    .into_inner();
        }
        debug!("{:?} {:?}", b, value);

        value
    }

    pub(crate) fn update_value(
        &mut self,
        b: &[NotNan<f32>; N],
        value: f32,
        max_a: Option<A>,
    ) -> f32 {
        let v = self.translator.b_to_v(b);
        //         debug!("b: {:?}", b);
        //         debug!("value: {:?}", value);
        if let Some(assoc_point) = self.grid.get_mut(&v) {
            let residual = (assoc_point.assoc_value().into_inner() - value).abs();
            assoc_point.assoc = max_a;
            assoc_point.v = NotNan::from_f32(value).unwrap();
            residual
        } else {
            let assoc_point =
                AssocBeliefPointN::new_not_nan(max_a, NotNan::from_f32(value).unwrap(), *b);
            self.grid.insert(v, assoc_point);
            value
        }
    }
}

impl<A: Debug + Copy + Clone, const N: usize> RegularGridBeliefPoints<AssocBeliefPointN<A, N>, N> {
    pub fn generate_uniform_grid(n_bin_per_dim: usize) -> Self {
        let mut grid = RegularGridBeliefPoints::new(n_bin_per_dim);
        for v in enumerate_grid_points::<N>(n_bin_per_dim) {
            let b = grid.translator.v_to_b(&v);
            grid.grid.insert(
                v,
                AssocBeliefPointN::new_not_nan(None, NotNan::from_f32(0.0).unwrap(), b),
            );
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::AssocBeliefPoint3;
    use crate::num_traits::FromPrimitive;
    use assert_approx_eq::assert_approx_eq;
    use rand::prelude::ThreadRng;
    use rand::*;

    #[derive(Clone, Debug)]
    struct VecBeliefPoints<B: Clone + Copy + Debug> {
        pub(crate) vec: Vec<B>,
    }

    impl<B: Clone + Copy + Debug> VecBeliefPoints<B> {
        fn new(_n: usize) -> VecBeliefPoints<B> {
            VecBeliefPoints { vec: vec![] }
        }

        fn push(&mut self, bp: B) {
            self.vec.push(bp);
        }
    }

    impl<A: Debug + Copy + Clone, const N: usize> VecBeliefPoints<AssocBeliefPointN<A, N>> {
        #[allow(dead_code)]
        pub(crate) fn find_closest_belief_point(
            &self,
            b: &[NotNan<f32>],
        ) -> Option<AssocBeliefPointN<A, N>> {
            let mut result = None;
            let mut current_best = MAX;
            for bp in self.vec.iter() {
                if l1_distance(b, &bp.inner()) <= current_best {
                    current_best = l1_distance(b, &bp.inner());
                    result = Some(*bp);
                }
            }

            result
        }
    }

    impl<A: Debug + Copy + Clone, const N: usize> VecBeliefPoints<AssocBeliefPointN<A, N>> {
        pub(crate) fn generate_uniform_grid(n_bin_per_dim: usize) -> Self {
            let mut vec = Self::new(n_bin_per_dim);
            let translator = RegularGridTranslator::new(n_bin_per_dim);
            for v in enumerate_grid_points::<N>(n_bin_per_dim) {
                let b = translator.v_to_b(&v);
                vec.push(AssocBeliefPointN::new_not_nan(
                    None,
                    NotNan::<f32>::from_f32(0.0).unwrap(),
                    b,
                ));
            }

            vec
        }
    }

    fn sample_point(rng: &mut ThreadRng) -> [NotNan<f32>; 3] {
        let a: f32 = rng.gen();
        let b: f32 = rng.gen();
        let xs = if a < b {
            [a, b - a, 1.0 - b]
        } else {
            [b, a - b, 1.0 - a]
        };
        [
            NotNan::<f32>::from_f32(xs[0]).unwrap(),
            NotNan::<f32>::from_f32(xs[1]).unwrap(),
            NotNan::<f32>::from_f32(xs[2]).unwrap(),
        ]
    }

    #[test]
    fn test_enumerate_grid_points() {
        let v = enumerate_grid_points::<3>(2);
        let expected = [
            [2, 0, 0],
            [2, 1, 0],
            [2, 2, 0],
            [2, 1, 1],
            [2, 2, 1],
            [2, 2, 2],
        ];
        for i in 0..v.len() {
            assert_eq!(v[i], expected[i]);
        }

        let v = enumerate_grid_points::<3>(3);
        assert_eq!(v.len(), 10);

        let v = enumerate_grid_points::<4>(3);
        for i in 0..v.len() {
            println!("{:?}", v[i]);
        }
    }

    #[test]
    fn test_grid_belief_points_find() {
        let gbps: RegularGridBeliefPoints<AssocBeliefPoint3<usize>, 3> =
            RegularGridBeliefPoints::generate_uniform_grid(20);
        let vbps: VecBeliefPoints<AssocBeliefPoint3<usize>> =
            VecBeliefPoints::generate_uniform_grid(20);
        let mut rng = thread_rng();
        for _i in 0..100 {
            let b = sample_point(&mut rng);
            if let Some(gbp) = gbps.find_closest_belief_point(&b) {
                if let Some(vbp) = vbps.find_closest_belief_point(&b) {
                    for j in 0..3 {
                        assert_approx_eq!(gbp.inner()[j], vbp.inner()[j]);
                    }
                }
            }
        }
    }

    #[test]
    fn test_get_corner_and_lambdas() {
        let mut rng = thread_rng();
        let gbps: RegularGridBeliefPoints<AssocBeliefPoint3<usize>, 3> =
            RegularGridBeliefPoints::generate_uniform_grid(4);
        for _ in 0..10 {
            let mut r = [0.0; 3];
            let b = sample_point(&mut rng);
            let p = gbps.translator.get_corner_and_lambdas(&b);
            for (v, l) in p {
                let bb = gbps.translator.v_to_b(&v);
                for i in 0..3 {
                    r[i] += l * bb[i].into_inner();
                }
            }
            for i in 0..3 {
                assert_approx_eq!(b[i].into_inner(), r[i])
            }
        }
    }

    #[test]
    fn test_get_corner_and_lambdas2() {
        let gbps: RegularGridBeliefPoints<AssocBeliefPointN<usize, 5>, 5> =
            RegularGridBeliefPoints::generate_uniform_grid(64);
        let b = [
            NotNan::from_f32(0.28453165).unwrap(),
            NotNan::from_f32(0.17970422).unwrap(),
            NotNan::from_f32(0.34496406).unwrap(),
            NotNan::from_f32(0.1908002).unwrap(),
            NotNan::from_f32(0.0).unwrap(),
        ];
        let p = gbps.translator.get_corner_and_lambdas(&b);
        println!("{:?}", p);
        let v = gbps.get_value_convex_interpolation(&b);
        println!("{:?}", v);
        let corners = gbps.translator.find_corners_of_subsimplex(&b);
        let expected = [
            [64, 45, 34, 12, 0],
            [64, 46, 34, 12, 0],
            [64, 46, 35, 12, 0],
            [64, 46, 35, 13, 0],
            [64, 46, 35, 13, 1],
        ];
        assert_eq!(corners, expected)
    }
}
