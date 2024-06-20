use crate::rtdp::RTDP;
use mdp::heuristic::{HeuristicWithMDPMut, HminHeuristic, ZeroHeuristic};
use mdp::mdp_traits::*;
use mdp::policy::policy_traits::GetActionProbabilityMut;
use mdp::value_estimator::CostEstimatorMut;
use std::fmt::Debug;
use std::hash::Hash;

pub struct RTDPSoftmaxPolicyBuilder {
    pub beta: f32,
}

impl RTDPSoftmaxPolicyBuilder {
    pub fn new(beta: f32) -> RTDPSoftmaxPolicyBuilder {
        RTDPSoftmaxPolicyBuilder { beta }
    }
}

impl<'a, M: StatesActions> BuildFrom<&'a M, RTDPSoftmaxPolicy<M::State, ZeroHeuristic>>
    for RTDPSoftmaxPolicyBuilder
{
    fn build_from(&self, _: &'a M) -> RTDPSoftmaxPolicy<M::State, ZeroHeuristic> {
        RTDPSoftmaxPolicy::new(self.beta, RTDP::new(ZeroHeuristic {}))
    }
}

impl<'a, M: StatesActions> BuildFrom<&'a M, RTDPSoftmaxPolicy<M::State, HminHeuristic<M::State>>>
    for RTDPSoftmaxPolicyBuilder
{
    fn build_from(&self, _: &'a M) -> RTDPSoftmaxPolicy<M::State, HminHeuristic<M::State>> {
        RTDPSoftmaxPolicy::new(self.beta, RTDP::new(HminHeuristic::new()))
    }
}

pub struct RTDPSoftmaxPolicy<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> {
    pub rtdp: RTDP<S, H>,
    beta: f32,
    rng: rand::rngs::ThreadRng,
}

impl<S: PartialEq + Eq + Copy + Clone + Debug + Hash, H> RTDPSoftmaxPolicy<S, H> {
    pub fn new(beta: f32, rtdp: RTDP<S, H>) -> RTDPSoftmaxPolicy<S, H> {
        RTDPSoftmaxPolicy {
            rtdp,
            beta,
            rng: rand::thread_rng(),
        }
    }
}

impl<
        M: ActionAvailability + ActionEnumerable + PMassMut<f32> + Cost + IsTerminal + GetNextStateMut,
        H: HeuristicWithMDPMut<M>,
    > GetActionProbabilityMut<M::Action, M> for RTDPSoftmaxPolicy<M::State, H>
{
    fn get_action_probability_mut(&mut self, s: &M::State, a: &M::Action, mdp: &mut M) -> f32 {
        for aa_id in 0..mdp.num_actions() {
            let aa = *mdp.id_to_action(aa_id);
            for (ss, _p) in mdp.p_mass_mut(s, &aa) {
                if mdp.is_terminal(&ss) {
                    continue;
                }
                self.rtdp.lrtdp_inner(ss, mdp, 0, &mut self.rng, 1e-3);
            }
        }

        let min_qsa = self.rtdp.get_value_ssp_mut(s, mdp);
        let numerator =
            (self.beta * (-1.0) * (self.rtdp.get_qsa_ssp_mut(s, a, mdp) - min_qsa)).exp();
        let mut denominator = 0.0;
        for a_id in 0..mdp.num_actions() {
            let at = *mdp.id_to_action(a_id);
            denominator +=
                (self.beta * (-1.0) * (self.rtdp.get_qsa_ssp_mut(s, &at, mdp) - min_qsa)).exp();
        }
        let result = numerator / denominator;

        if result == 0.0 {
            1e-6
        } else {
            result
        }
    }
}

mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    
    use mdp::grid_world::{GridWorldAction::*, GridWorldMDP, GridWorldState};
    

    #[test]
    fn test_grid_world_softmax() {
        let mut mdp = GridWorldMDP::default();
        let err = 1e-3;
        let lrtdp = RTDP::new(ZeroHeuristic {});

        let mut softmax_policy = RTDPSoftmaxPolicy::new(1.0, lrtdp);

        assert_approx_eq!(
            0.40183,
            softmax_policy.get_action_probability_mut(
                &GridWorldState::new(0, 0),
                &AttemptRight,
                &mut mdp,
            ),
            err
        );
        assert_approx_eq!(
            0.287654,
            softmax_policy.get_action_probability_mut(
                &GridWorldState::new(0, 0),
                &AttemptDown,
                &mut mdp,
            ),
            err
        );
        assert_approx_eq!(
            0.15698,
            softmax_policy.get_action_probability_mut(
                &GridWorldState::new(0, 0),
                &AttemptUp,
                &mut mdp,
            ),
            err
        );
        assert_approx_eq!(
            0.1535276,
            softmax_policy.get_action_probability_mut(
                &GridWorldState::new(0, 0),
                &AttemptLeft,
                &mut mdp,
            ),
            err
        );
    }
}
