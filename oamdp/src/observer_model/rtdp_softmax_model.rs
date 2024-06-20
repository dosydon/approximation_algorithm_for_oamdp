use mdp::mdp_traits::StatesActions;
use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicy;

use super::ImplicitCommunicationModel;

pub type RTDPSoftmaxModel<M, H, const N: usize> =
    ImplicitCommunicationModel<RTDPSoftmaxPolicy<<M as StatesActions>::State, H>, M, N>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::belief_update_type::ObserveabilityAssumption::*;
    use crate::traits::ProbSassGivenTheta;

    use mdp::baker_grid::*;

    use assert_approx_eq::assert_approx_eq;
    use mdp::blocks_world::BlocksWorldPartialMDP;
    use mdp::blocks_world::Location::*;
    use mdp::blocks_world::*;
    use mdp::heuristic::HminHeuristic;
    use mdp::heuristic::ZeroHeuristic;
    use mdp::mdp_traits::{BuildFrom, GetNextStateMut, InitialState};
    use mdp::search_rescue::{
        Coordinate, ObstacleCompatibility, SearchRescueAction, SearchRescueParameter,
        SearchRescuePartialMDP,
    };
    use mdp::value_estimator::CostEstimator;
    use rand::thread_rng;
    use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicyBuilder;

    #[test]
    fn test_rtdp_model_baker() {
        let width = 9;
        let height = 5;
        let obstacles = vec![];

        let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles)
            .set_prob_veering(0.1)
            .set_initial_state(BakerGridState::new(2, 0));
        let possible_goals = [
            BakerGridState::new(2, 8),
            BakerGridState::new(0, 8),
            BakerGridState::new(4, 8),
        ];
        let softmax_policy = RTDPSoftmaxPolicyBuilder::new(1.0);
        let mut om: ImplicitCommunicationModel<RTDPSoftmaxPolicy<_, HminHeuristic<_>>, _, 3> =
            ImplicitCommunicationModel::new_from_possible_goals(
                &partial_mdp,
                &softmax_policy,
                possible_goals,
                OnlyActionsAreConsidered,
            );

        assert_approx_eq!(
            0.22491117,
            om.prob_sass_given_theta(
                0,
                &BakerGridState::new(2, 0),
                &BakerGridAction::East,
                &BakerGridState::new(2, 1)
            ),
            1e-2
        );

        assert_approx_eq!(
            7.4726315,
            om.assumed_policy[0].rtdp.get_qsa_ssp(
                &BakerGridState::new(2, 0),
                &BakerGridAction::East,
                &om.mdp_for_each_goal[0]
            ),
            1e-2
        );

        assert_approx_eq!(
            0.082740195,
            om.prob_sass_given_theta(
                0,
                &BakerGridState::new(2, 0),
                &BakerGridAction::West,
                &BakerGridState::new(2, 1)
            ),
            1e-2
        );
    }

    #[test]
    fn test_rtdp_model_search_rescue() {
        let partial_mdp = SearchRescuePartialMDP::new(5, 5, vec![(4, 1)], Coordinate::new(0, 4));
        let softmax_policy = RTDPSoftmaxPolicyBuilder::new(0.3);
        let possible_types = [
            SearchRescueParameter::new(Coordinate::new(4, 2), ObstacleCompatibility::High),
            SearchRescueParameter::new(Coordinate::new(4, 2), ObstacleCompatibility::Low),
        ];
        let mut mdp = partial_mdp.build_from(&possible_types[0]);

        let mut om: ImplicitCommunicationModel<RTDPSoftmaxPolicy<_, ZeroHeuristic>, _, 2> =
            ImplicitCommunicationModel::new_from_possible_goals(
                &partial_mdp,
                &softmax_policy,
                possible_types,
                OnlyActionsAreConsidered,
            );

        let mut rng = thread_rng();
        let s = mdp.initial_state();
        let ss = mdp.get_next_state_mut(&s, &SearchRescueAction::East, &mut rng);
        println!("{:?}", ss);
        println!(
            "{}",
            om.prob_sass_given_theta(0, &s, &SearchRescueAction::East, &ss)
        );
    }

    #[test]
    fn test_rtdp_model_blocks() {
        let partial_mdp = BlocksWorldPartialMDP::new(
            [OnTable, OnTable, On(Block::new(1)), OnTable],
            0.1,
            ['A', 'M', 'S', 'R'],
        );
        let possible_goals = [
            [
                On(Block::new(3)),
                On(Block::new(2)),
                OnTable,
                On(Block::new(1)),
            ],
            [
                On(Block::new(1)),
                On(Block::new(2)),
                OnTable,
                On(Block::new(0)),
            ],
        ];
        let softmax_policy = RTDPSoftmaxPolicyBuilder::new(0.3);
        let mut mdp = partial_mdp.build_from(&possible_goals[0]);
        let mut om: ImplicitCommunicationModel<RTDPSoftmaxPolicy<_, ZeroHeuristic>, _, 2> =
            ImplicitCommunicationModel::new_from_possible_goals(
                &partial_mdp,
                &softmax_policy,
                possible_goals,
                OnlyActionsAreConsidered,
            );
        let mut rng = thread_rng();
        let s = mdp.initial_state();
        let ss = mdp.get_next_state_mut(&s, &BlocksWorldAction::PickUp(Block::new(0)), &mut rng);
        println!("{:?}", ss);
        println!(
            "{}",
            om.prob_sass_given_theta(0, &s, &BlocksWorldAction::PickUp(Block::new(0)), &ss)
        );
        println!(
            "{}",
            om.prob_sass_given_theta(1, &s, &BlocksWorldAction::PickUp(Block::new(0)), &ss)
        );
    }
}
