use crate::belief_cost_function::{BeliefCostType, Objective};
use crate::belief_update_type::ObserveabilityAssumption;

use mdp::blocks_world::*;
use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;
use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicyBuilder;

use super::BlocksOAMDPBuilder;

impl BlocksOAMDPBuilder<RTDPSoftmaxPolicyBuilder, 4, 2> {
    pub fn new4_2(id: usize) -> Self {
        BlocksOAMDPBuilder {
            policy_builder: RTDPSoftmaxPolicyBuilder::new(1.0),
            observability_assumption: ObserveabilityAssumption::ActionNotObservable,
            belief_cost_type: BeliefCostType::TVDistance,
            horizon: 13,
            partial_mdp: get_partial_mdp(id),
            possible_goals: get_possible_goals(id),
            true_goal: get_true_goal(id),
            objective: Objective::LinearCombination(1.0, 0.1),
        }
    }
}

impl BlocksOAMDPBuilder<SoftmaxPolicyBuilder, 4, 2> {
    pub fn new4_2_enumerable(id: usize) -> Self {
        BlocksOAMDPBuilder {
            policy_builder: SoftmaxPolicyBuilder::new(1.0),
            observability_assumption: ObserveabilityAssumption::ActionNotObservable,
            belief_cost_type: BeliefCostType::TVDistance,
            horizon: 13,
            partial_mdp: get_partial_mdp(id),
            possible_goals: get_possible_goals(id),
            true_goal: get_true_goal(id),
            objective: Objective::LinearCombination(1.0, 0.1),
        }
    }
}

fn get_partial_mdp(id: usize) -> BlocksWorldPartialMDP {
    let lm = LetterManager::new(['A', 'M', 'S', 'R']);
    match id {
        1 => BlocksWorldPartialMDP::new(lm.str_to_locations("A SM R"), 0.1, lm.letters),
        2 => BlocksWorldPartialMDP::new(lm.str_to_locations("ARSM"), 0.1, lm.letters),
        3 => BlocksWorldPartialMDP::new(lm.str_to_locations("MA SR"), 0.1, lm.letters),
        _ => panic!("not matching instance id"),
    }
}

fn get_possible_goals(id: usize) -> [[char; 4]; 2] {
    match id {
        1 => [['A', 'R', 'M', 'S'], ['R', 'A', 'M', 'S']],
        2 => [['R', 'A', 'M', 'S'], ['A', 'R', 'M', 'S']],
        3 => [['A', 'R', 'M', 'S'], ['S', 'R', 'A', 'M']],
        _ => panic!("not matching instance id"),
    }
}

fn get_true_goal(_id: usize) -> usize {
    0
}
