use mdp::{
    blocks_world::{Block, BlocksWorldPartialMDPN, Location::*},
    policy::softmax_policy::SoftmaxPolicyBuilder,
};
use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicyBuilder;

use crate::{
    belief_cost_function::{BeliefCostType, Objective},
    belief_update_type::ObserveabilityAssumption,
};

use super::BlocksOAMDPBuilder;

impl BlocksOAMDPBuilder<RTDPSoftmaxPolicyBuilder, 6, 2> {
    pub fn new6_2(id: usize) -> BlocksOAMDPBuilder<RTDPSoftmaxPolicyBuilder, 6, 2> {
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

impl BlocksOAMDPBuilder<SoftmaxPolicyBuilder, 6, 2> {
    pub fn new6_2_enumerable(id: usize) -> BlocksOAMDPBuilder<SoftmaxPolicyBuilder, 6, 2> {
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

fn get_partial_mdp(id: usize) -> BlocksWorldPartialMDPN<6> {
    let b0 = Block::new(0);
    let b1 = Block::new(1);
    let b2 = Block::new(2);
    let b3 = Block::new(3);
    let b4 = Block::new(4);
    match id {
        61 => BlocksWorldPartialMDPN::new(
            [OnTable, OnTable, OnTable, OnTable, OnTable, OnTable],
            0.3,
            ['A', 'B', 'C', 'D', 'E', 'F'],
        ),
        62 => BlocksWorldPartialMDPN::new(
            [OnTable, On(b0), On(b1), On(b2), On(b3), On(b4)],
            0.3,
            ['A', 'B', 'C', 'D', 'E', 'F'],
        ),
        _ => panic!("not matching instance id"),
    }
}

fn get_possible_goals(id: usize) -> [[char; 6]; 2] {
    match id {
        61 => [
            ['A', 'B', 'C', 'D', 'E', 'F'],
            ['C', 'B', 'C', 'D', 'E', 'F'],
        ],
        62 => [
            ['A', 'B', 'C', 'F', 'E', 'D'],
            ['C', 'B', 'C', 'D', 'E', 'F'],
        ],
        _ => panic!("not matching instance id"),
    }
}

fn get_true_goal(_id: usize) -> usize {
    0
}
