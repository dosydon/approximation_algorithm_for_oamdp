use mdp::{
    baker_grid::{BakerGridMDP, BakerGridState},
    policy::softmax_policy::SoftmaxPolicy,
    value_iteration::ValueTable,
};
use serde::{Deserialize, Serialize};

use crate::{observer_model::ExplicitCommunicationModel, traits::CommunicationProbability};

use super::{BakerCommunicationAction, Shape};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CommunicationType {
    RSA,
    GenerativeNoise(f32),
    SoftGenerativeNoise(f32, f32),
}

pub type BakerCommunicationModel<const N: usize> = ExplicitCommunicationModel<
    SoftmaxPolicy<ValueTable<BakerGridState>>,
    BakerGridMDP,
    BakerCommunicationProb<N>,
    N,
>;

pub struct BakerCommunicationProb<const N: usize> {
    pub(crate) shapes: [Shape; N],
    pub(crate) possible_goals: [BakerGridState; N],
    communication_type: CommunicationType,
    pub(crate) communication_actions: Vec<BakerCommunicationAction>,
    pub(crate) communication_cost: f32,
}

impl<const N: usize> BakerCommunicationProb<N> {
    pub fn new(
        shapes: [Shape; N],
        possible_goals: [BakerGridState; N],
        communication_type: CommunicationType,
        communication_actions: Vec<BakerCommunicationAction>,
        communication_cost: f32,
    ) -> Self {
        BakerCommunicationProb {
            shapes,
            possible_goals,
            communication_type,
            communication_actions,
            communication_cost,
        }
    }
}

impl<const N: usize> CommunicationProbability<BakerCommunicationAction>
    for BakerCommunicationProb<N>
{
    fn communication_probability(&self, id: usize, a: &BakerCommunicationAction) -> f32 {
        match self.communication_type {
            CommunicationType::SoftGenerativeNoise(alpha, eta) => match self.shapes[id] {
                Shape::BlueSquare => match a {
                    BakerCommunicationAction::Blue => 0.5 * (alpha - eta),
                    BakerCommunicationAction::Square => 0.5 * (alpha - eta),
                    BakerCommunicationAction::Circle => 0.5 * eta,
                    BakerCommunicationAction::Green => 0.5 * eta,
                    BakerCommunicationAction::None => 1.0 - alpha - eta,
                },
                Shape::BlueCircle => match a {
                    BakerCommunicationAction::Blue => 0.5 * (alpha - eta),
                    BakerCommunicationAction::Circle => 0.5 * (alpha - eta),
                    BakerCommunicationAction::Green => 0.5 * eta,
                    BakerCommunicationAction::Square => 0.5 * eta,
                    BakerCommunicationAction::None => 1.0 - alpha - eta,
                },
                Shape::GreenSquare => match a {
                    BakerCommunicationAction::Blue => 0.5 * eta,
                    BakerCommunicationAction::Circle => 0.5 * eta,
                    BakerCommunicationAction::Green => 0.5 * (alpha - eta),
                    BakerCommunicationAction::Square => 0.5 * (alpha - eta),
                    BakerCommunicationAction::None => 1.0 - alpha - eta,
                },
                Shape::GreenCircle => match a {
                    BakerCommunicationAction::Blue => 0.5 * eta,
                    BakerCommunicationAction::Circle => 0.5 * (alpha - eta),
                    BakerCommunicationAction::Green => 0.5 * (alpha - eta),
                    BakerCommunicationAction::Square => 0.5 * eta,
                    BakerCommunicationAction::None => 1.0 - alpha - eta,
                },
            },
            _ => panic!("Invalid communication type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use mdp::baker_grid::*;

    use mdp::mdp_traits::Build;
    use mdp::mdp_traits::GetNextState;
    use mdp::mdp_traits::InitialState;
    use rand::thread_rng;

    use crate::domains::baker_grid::BakerCOAMDPBuilder;
    use crate::domains::baker_grid::BakerCommunicationAction;
    use crate::domains::baker_grid::BakerJointAction;
    use crate::traits::BeliefOverGoal;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_belief_changes() {
        let mut rng = thread_rng();
        let builder = BakerCOAMDPBuilder::new(1);

        let oamdp = builder.build();
        let initial_state = oamdp.initial_state();
        let a = BakerJointAction::new(BakerGridAction::NorthEast, BakerCommunicationAction::Blue);
        let ss = oamdp.get_next_state(&initial_state, &a, &mut rng);

        let b = ss.get_belief_over_goal();
        assert_approx_eq!(0.45727921, b[0].into_inner(), 1e-3);
        assert_approx_eq!(0.108521506, b[1].into_inner(), 1e-3);
        assert_approx_eq!(0.43419936, b[2].into_inner(), 1e-3);
        println!("{:?}", ss);
    }
}
