use num_traits::FromPrimitive;
use ordered_float::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BeliefCostType {
    Euclidean,
    KLDivergence,
    Disimulation,
    TVDistance,
    Deceptive(usize),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Objective {
    BeliefCostOnly,
    LinearCombination(f32, f32),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum BeliefCostFunction<const N: usize> {
    #[serde(with = "serde_arrays")]
    Euclidean([NotNan<f32>; N]),
    #[serde(with = "serde_arrays")]
    TVDistance([NotNan<f32>; N]),
    KLDivergence(usize),
    Disimulation,
    Threashold(f32, f32, usize),
    //     Add(Box<BeliefCostFunction<N>>, Box<BeliefCostFunction<N>>),
}

impl<const N: usize> BeliefCostFunction<N> {
    pub fn get_legible_cost_function(id: usize) -> Self {
        let mut target_belief = [NotNan::from_f32(0.0).unwrap(); N];
        let true_goal = id;
        target_belief[true_goal] = NotNan::from_f32(1.0).unwrap();

        Self::TVDistance(target_belief)
    }
}

impl<const N: usize> BeliefCostFunction<N> {
    pub fn b_cost(&self, b: &[NotNan<f32>; N]) -> f32 {
        match self {
            Self::Euclidean(target_belief) => euclidean_distance(target_belief, b),
            Self::KLDivergence(true_goal) => kl_divergence_for_one_hot(b, *true_goal),
            Self::TVDistance(target_belief) => 0.5 * l1_distance(target_belief, b),
            Self::Disimulation => (N as f32).log2() - entropy(b),
            Self::Threashold(th, cost, i) => {
                if b[*i].into_inner() > *th {
                    *cost
                } else {
                    0.0
                }
            } //             Self::Add(c1, c2) => c1.b_cost(b) + c2.b_cost(b),
        }
    }
}

pub fn squared_euclidean_distance(a: &[NotNan<f32>], b: &[NotNan<f32>]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(aa, bb)| (aa.into_inner() - bb.into_inner()) * (aa.into_inner() - bb.into_inner()))
        .sum::<f32>()
}

pub fn euclidean_distance(a: &[NotNan<f32>], b: &[NotNan<f32>]) -> f32 {
    squared_euclidean_distance(a, b).sqrt()
}

pub fn kl_divergence_for_one_hot(b: &[NotNan<f32>], true_goal: usize) -> f32 {
    -1.0 * (b[true_goal].into_inner()).ln()
}

pub fn l1_distance(a: &[NotNan<f32>], b: &[NotNan<f32>]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(aa, bb)| (aa.into_inner() - bb.into_inner()).abs())
        .sum::<f32>()
}

pub fn entropy(a: &[NotNan<f32>]) -> f32 {
    (-1.0)
        * a.iter()
            .filter(|aa| aa.into_inner() > 0.0)
            .map(|aa| (aa.into_inner() * aa.into_inner().log2()))
            .sum::<f32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use num_traits::cast::FromPrimitive;

    #[test]
    fn test_euclidian_distance() {
        let err = 1e-1;
        assert_approx_eq!(
            (2.0_f32).sqrt(),
            euclidean_distance(
                &[
                    NotNan::<f32>::from_f32(1.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap()
                ],
                &[
                    NotNan::<f32>::from_f32(0.0).unwrap(),
                    NotNan::<f32>::from_f32(1.0).unwrap()
                ]
            ),
            err
        );

        println!(
            "{}",
            euclidean_distance(
                &[
                    NotNan::<f32>::from_f32(1.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap()
                ],
                &[
                    NotNan::<f32>::from_f32(1.0 / 3.0).unwrap(),
                    NotNan::<f32>::from_f32(1.0 / 3.0).unwrap(),
                    NotNan::<f32>::from_f32(1.0 / 3.0).unwrap()
                ]
            ),
        );

        println!(
            "{}",
            euclidean_distance(
                &[
                    NotNan::<f32>::from_f32(1.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap()
                ],
                &[
                    NotNan::<f32>::from_f32(1.0 / 2.0).unwrap(),
                    NotNan::<f32>::from_f32(1.0 / 2.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap()
                ]
            ),
        );
    }

    #[test]
    fn test_l1_distance() {
        //         let err = 1e-1;
        println!(
            "{}",
            l1_distance(
                &[
                    NotNan::<f32>::from_f32(1.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap()
                ],
                &[
                    NotNan::<f32>::from_f32(1.0 / 3.0).unwrap(),
                    NotNan::<f32>::from_f32(1.0 / 3.0).unwrap(),
                    NotNan::<f32>::from_f32(1.0 / 3.0).unwrap()
                ]
            ),
        );

        println!(
            "{}",
            l1_distance(
                &[
                    NotNan::<f32>::from_f32(1.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap()
                ],
                &[
                    NotNan::<f32>::from_f32(1.0 / 2.0).unwrap(),
                    NotNan::<f32>::from_f32(1.0 / 2.0).unwrap(),
                    NotNan::<f32>::from_f32(0.0).unwrap()
                ]
            ),
        );
    }

    #[test]
    fn test_kl_divergence() {
        let err = 1e-1;
        assert_approx_eq!(
            0.693147180560,
            kl_divergence_for_one_hot(
                &[
                    NotNan::<f32>::from_f32(0.5).unwrap(),
                    NotNan::<f32>::from_f32(0.5).unwrap()
                ],
                0
            ),
            err
        );
    }

    #[test]
    fn test_deceptive() {
        let b = BeliefCostType::Deceptive(0);
        let s = serde_yaml::to_string(&b).unwrap();
        println!("{:?}", s);
    }

    #[test]
    fn test_entropy() {
        let err = 1e-1;
        assert_approx_eq!(
            1.0,
            entropy(&[
                NotNan::<f32>::from_f32(0.5).unwrap(),
                NotNan::<f32>::from_f32(0.5).unwrap()
            ],),
            err
        );

        assert_approx_eq!(
            0.0,
            entropy(&[
                NotNan::<f32>::from_f32(1.0).unwrap(),
                NotNan::<f32>::from_f32(0.0).unwrap()
            ],),
            err
        );

        let f = BeliefCostFunction::Disimulation;
        let b0 = [NotNan::<f32>::from_f32(0.5).unwrap(); 2];
        let b1 = [
            NotNan::<f32>::from_f32(0.0).unwrap(),
            NotNan::<f32>::from_f32(1.0).unwrap(),
        ];
        println!("{}", f.b_cost(&b0));
        println!("{}", f.b_cost(&b1));
    }
}
