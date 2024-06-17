use mcts::{Budget, MCTS};
use mdp::finite_horizon_wrapper::FiniteHorizonWrapper;
use mdp::heuristic::ZeroHeuristic;
use mdp::into_inner::Inner;
use mdp::mdp_traits::DisplayState;
use mdp::mdp_traits::*;
use mdp::search_rescue::{
    Coordinate, ObstacleCompatibility, SearchRescueParameter, SearchRescuePartialMDP,
};
use mdp::search_rescue::{SearchRescueAction, SearchRescueMDP};
use oamdp::oamdp::oamdp::{OAMDP, OAMDP2};
use oamdp::observer_model::ImplicitCommunicationModel;
use oamdp::policy::RandomOAMDPPolicy;
use rand::thread_rng;
use rtdp::rtdp_softmax_policy::{RTDPSoftmaxPolicy, RTDPSoftmaxPolicyBuilder};

fn main() {
    env_logger::init();
    let partial_mdp = SearchRescuePartialMDP::new(5, 5, vec![(4, 1)], Coordinate::new(0, 4));
    let softmax_policy = RTDPSoftmaxPolicyBuilder::new(0.3);
    let possble_types = [
        SearchRescueParameter::new(Coordinate::new(4, 2), ObstacleCompatibility::High),
        SearchRescueParameter::new(Coordinate::new(4, 2), ObstacleCompatibility::Low),
    ];
    let oamdp: OAMDP<
        ImplicitCommunicationModel<RTDPSoftmaxPolicy<_, ZeroHeuristic>, _, 2>,
        _,
        _,
        2,
    > = OAMDP2::new_implicit_model(
        &partial_mdp,
        &softmax_policy,
        possble_types,
        0,
        oamdp::belief_cost_function::BeliefCostType::TVDistance,
        oamdp::belief_cost_function::Objective::LinearCombination(1.0, 0.1),
        oamdp::belief_update_type::ObserveabilityAssumption::ActionObservable,
    );
    let mut oamdp = FiniteHorizonWrapper::new(oamdp, 15);

    let mut rng = thread_rng();
    oamdp.get_next_state_mut(&oamdp.initial_state(), &SearchRescueAction::North, &mut rng);

    let policy = RandomOAMDPPolicy::new();
    let mut mcts = MCTS::new(oamdp, policy)
        .set_budget(Budget::NumIterations(1000))
        .set_c(1.0)
        .set_num_rollouts(10);

    unsafe {
        let mdp_p = &mcts.mdp.mdp.mdp as *const SearchRescueMDP;
        for (s, _a, _, c) in mcts.into_iter_with(&mut rng) {
            println!("{:?} {}", s, c);
            (*mdp_p).display(&s.inner().inner());
        }
    }
}
