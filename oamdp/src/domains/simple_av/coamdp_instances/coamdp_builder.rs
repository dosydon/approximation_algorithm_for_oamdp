use itertools::iproduct;
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
use mdp::mdp_traits::ActionEnumerable;
use mdp::mdp_traits::BuildFrom;
use mdp::simple_av::SimpleAVParameter;
use mdp::simple_av::SimpleAVPartialMDP;
use mdp::simple_av::SimpleAVState;
use mdp::simple_av::SimpleAVVehicleInFrontMDP;

use num_traits::FromPrimitive;
use ordered_float::NotNan;

use crate::belief_cost_function::BeliefCostFunction;
use crate::belief_cost_function::Objective;
use crate::domains::simple_av::communication_model::AVCommunicationModel;
use crate::domains::simple_av::joint_action::AVJointAction;
use crate::domains::simple_av::AVCommunicationAction;
use crate::oamdp::oamdp::OAMDP;

use crate::oamdp::OAMDPFiniteHorizon;

pub struct SimpleAVCOAMDPBuilder<const N: usize> {
    partial_mdp: SimpleAVPartialMDP,
    possible_goals: [SimpleAVParameter; N],
    communication_actions: Vec<AVCommunicationAction>,
    max_t: usize,
    mdp: SimpleAVVehicleInFrontMDP,
    belief_cost_function: BeliefCostFunction<N>,
    cost_type: Objective,
    communication_cost: f32,
}

fn get_partial_mdp(id: usize) -> SimpleAVPartialMDP {
    match id {
        1 => SimpleAVPartialMDP::new(0, 40, -2, 5, SimpleAVState::new(0, 2)),
        2 => SimpleAVPartialMDP::new(0, 40, -2, 5, SimpleAVState::new(0, 2)),
        //         3 => SimpleAVPartialMDP::new(0, 40, -2, 5, SimpleAVState::new(0, 2)),
        _ => panic!("Invalid instance id"),
    }
}

fn get_possible_goals(id: usize) -> [SimpleAVParameter; 3] {
    match id {
        1 => [
            SimpleAVParameter::NonYield(35, 2, 3),
            SimpleAVParameter::Stopping(6, 8),
            SimpleAVParameter::Stopping(15, 18),
        ],
        2 => [
            SimpleAVParameter::NonYield(35, 2, 3),
            SimpleAVParameter::Stopping(6, 8),
            SimpleAVParameter::Stopping(15, 18),
        ],
        //         3 => [
        //             SimpleAVParameter::NonYield(35, 2, 3),
        //             SimpleAVParameter::Stopping(6, 8),
        //             SimpleAVParameter::Stopping(15, 18),
        //         ],
        _ => panic!("Invalid instance id"),
    }
}

fn get_true_goal(id: usize) -> usize {
    match id {
        1 => 1,
        2 => 2,
        _ => panic!("not matching id"),
    }
}

fn get_belief_cost_function(id: usize) -> BeliefCostFunction<3> {
    match id {
        1 => BeliefCostFunction::get_legible_cost_function(get_true_goal(id)),
        2 => BeliefCostFunction::get_legible_cost_function(get_true_goal(id)),
        _ => panic!("not matching id"),
    }
}

fn get_cost_type(id: usize) -> Objective {
    match id {
        1 => Objective::LinearCombination(1.0, 1.0),
        2 => Objective::LinearCombination(1.0, 1.0),
        _ => panic!("not matching id"),
    }
}

impl SimpleAVCOAMDPBuilder<3> {
    pub fn new(instance_id: usize) -> Self {
        let true_goal = get_true_goal(instance_id);
        let partial_mdp = get_partial_mdp(instance_id);
        let possible_goals = get_possible_goals(instance_id);
        let mdp = partial_mdp.build_from(possible_goals[true_goal]);

        SimpleAVCOAMDPBuilder {
            partial_mdp: partial_mdp,
            possible_goals: possible_goals,
            communication_actions: vec![AVCommunicationAction::Flash, AVCommunicationAction::None],
            max_t: 15,
            mdp: mdp,
            belief_cost_function: get_belief_cost_function(instance_id),
            cost_type: get_cost_type(instance_id),
            communication_cost: 1.0,
        }
    }
}

impl SimpleAVCOAMDPBuilder<3> {
    pub fn build(
        self,
    ) -> OAMDPFiniteHorizon<AVCommunicationModel<3>, SimpleAVVehicleInFrontMDP, AVJointAction, 3>
    {
        let physical_actions: Vec<_> = self.mdp.enumerate_actions().cloned().collect();

        let joint_actions = iproduct!(physical_actions.iter(), self.communication_actions.iter())
            .map(|(a, b)| AVJointAction::new(*a, *b))
            .collect::<Vec<_>>();

        let communication_model = AVCommunicationModel::from_targets(
            &self.partial_mdp,
            self.possible_goals,
            self.communication_cost,
        );

        let oamdp = FiniteHorizonWrapper::new(
            OAMDP::new(
                communication_model,
                self.mdp,
                self.belief_cost_function,
                [NotNan::from_f32(1.0 / 3.0).unwrap(); 3],
                0.9,
                joint_actions,
                self.cost_type,
            ),
            self.max_t,
        );

        oamdp
    }
}
