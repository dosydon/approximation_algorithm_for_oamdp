use std::convert::TryInto;

use mdp::mdp_traits::StatesActions;
use mdp::mdp_traits::*;
use mdp::policy::softmax_policy::SoftmaxPolicy;
use mdp::value_iteration::ValueTable;

use crate::belief_update_type::ObserveabilityAssumption;

use super::ImplicitCommunicationModel;

pub type SoftmaxModel<M, const N: usize> =
    ImplicitCommunicationModel<SoftmaxPolicy<ValueTable<<M as StatesActions>::State>>, M, N>;

impl<M: StatesActions, const N: usize> SoftmaxModel<M, N> {
    pub fn set_beta(&mut self, id: usize, beta: f32) {
        self.assumed_policy[id].beta = beta;
    }
}

impl<
        P,
        M: ActionEnumerable + ActionAvailability + ExplicitTransition + PMass<f32> + IsTerminal + Cost,
        const N: usize,
    > ImplicitCommunicationModel<P, M, N>
{
    pub fn new_from_possible_goals<MF, MP, PF>(
        mdp_factory: &MF,
        policy_builder: &PF,
        possible_goals: [MP; N],
        observability_assumption: ObserveabilityAssumption,
    ) -> Self
    where
        for<'a> MF: BuildFrom<&'a MP, M>,
        for<'a> PF: BuildFrom<&'a M, P>,
    {
        let mdp_for_each_goal: [M; N] = match (0..N)
            .into_iter()
            .map(|i| mdp_factory.build_from(&possible_goals[i]))
            .collect::<Vec<_>>()
            .try_into()
        {
            Ok(m) => m,
            Err(_) => panic!("Failed to convert Vec to array"),
        };
        let assumed_policy = match mdp_for_each_goal
            .iter()
            .map(|mdp| policy_builder.build_from(mdp))
            .collect::<Vec<_>>()
            .try_into()
        {
            Ok(m) => m,
            Err(_) => panic!("Failed to convert Vec to array"),
        };

        ImplicitCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            observability_assumption,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::belief_update_type::ObserveabilityAssumption::*;
    use crate::traits::ProbSassGivenTheta;

    use mdp::baker_grid::*;

    use assert_approx_eq::assert_approx_eq;
    use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;
    use mdp::value_estimator::CostEstimator;

    #[test]
    fn test_prob_sass_given_theta() {
        let width = 9;
        let height = 5;
        let obstacles = vec![];

        let softmax_policy = SoftmaxPolicyBuilder::new(1.0);
        let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles)
            .set_prob_veering(0.1)
            .set_initial_state(BakerGridState::new(2, 0));
        let possible_goals = [
            BakerGridState::new(2, 8),
            BakerGridState::new(0, 8),
            BakerGridState::new(4, 8),
        ];
        let om = ImplicitCommunicationModel::new_from_possible_goals(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            OnlyActionsAreConsidered,
        );

        assert_approx_eq!(
            7.4726315,
            om.assumed_policy[0].get_qsa_ssp(
                &BakerGridState::new(2, 0),
                &BakerGridAction::East,
                &om.mdp_for_each_goal[0]
            )
        );

        assert_approx_eq!(
            0.22491117,
            om.prob_sass_given_theta(
                0,
                &BakerGridState::new(2, 0),
                &BakerGridAction::East,
                &BakerGridState::new(2, 1)
            )
        );
        assert_approx_eq!(
            0.082740195,
            om.prob_sass_given_theta(
                0,
                &BakerGridState::new(2, 0),
                &BakerGridAction::West,
                &BakerGridState::new(2, 1)
            )
        );
    }
}
