use crate::belief_cost_function::BeliefCostType;
use crate::belief_update_type::ObserveabilityAssumption;
use crate::belief_update_type::ObserveabilityAssumption::*;
use crate::oamdp::oamdp::OAMDP2;
use crate::oamdp::OAMDPFiniteHorizon2;
use crate::observer_model::SoftmaxModel;
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
use mdp::mdp_traits::BuildFrom;
use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;
use mdp::simple_av::SimpleAVPedestrianParameter::*;
use mdp::simple_av::*;
use num_traits::cast::FromPrimitive;
use ordered_float::*;

type LegibleAV = OAMDP2<SoftmaxModel<SimpleAVPedestrianMDP, 2>, SimpleAVPedestrianMDP>;
type LegibleAVFiniteHorizon =
    OAMDPFiniteHorizon2<SoftmaxModel<SimpleAVPedestrianMDP, 2>, SimpleAVPedestrianMDP>;

pub struct LegibleAVBuilder {
    beta: f32,
    observability_assumption: ObserveabilityAssumption,
    distance_measure: BeliefCostType,
    horizon: usize,
}

impl LegibleAVBuilder {
    pub fn new(beta: f32) -> LegibleAVBuilder {
        LegibleAVBuilder {
            beta: beta,
            observability_assumption: ActionObservable,
            distance_measure: BeliefCostType::Euclidean,
            horizon: 13,
        }
    }

    pub fn set_horizon(mut self, horizon: usize) -> LegibleAVBuilder {
        self.horizon = horizon;

        self
    }

    pub fn set_distance_measure(mut self, distance_measure: BeliefCostType) -> LegibleAVBuilder {
        self.distance_measure = distance_measure;

        self
    }
}

fn get_partial_mdp() -> SimpleAVPedestrianPartialMDP {
    SimpleAVPedestrianPartialMDP::new(30, 10, 4)
}

fn get_possible_goals(id: usize) -> [SimpleAVPedestrianParameter; 2] {
    match id {
        1 => [NonYield, Yield],
        2 => [Far, FastPedestrian],
        _ => panic!("not matching instance id"),
    }
}

fn get_true_goal(id: usize) -> usize {
    match id {
        1 => 1,
        2 => 0,
        _ => panic!("not matching instance id"),
    }
}

fn get_initial_belief(id: usize) -> [NotNan<f32>; 2] {
    match id {
        _ => [NotNan::<f32>::from_f32(1.0 / 2.0).unwrap(); 2],
    }
}

impl BuildFrom<usize, LegibleAVFiniteHorizon> for LegibleAVBuilder {
    fn build_from(&self, id: usize) -> LegibleAVFiniteHorizon {
        let partial_mdp = get_partial_mdp();
        let softmax_policy = SoftmaxPolicyBuilder::new(self.beta);
        let possible_goals = get_possible_goals(id);

        let legible_mdp = LegibleAV::new_with_initial_belief(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            get_true_goal(id),
            self.distance_measure.clone(),
            crate::belief_cost_function::Objective::BeliefCostOnly,
            self.observability_assumption,
            get_initial_belief(id),
        );

        FiniteHorizonWrapper::new(legible_mdp, self.horizon)
    }
}
