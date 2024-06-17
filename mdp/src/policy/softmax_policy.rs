use crate::mdp_traits::*;
use crate::policy::policy_traits::Policy;
use crate::value_estimator::{CostEstimator, ValueEstimator};

use crate::common::value_table::ValueTable;
use crate::value_iteration::value_iteration_ssp;

use super::policy_traits::PolicyMut;

pub struct SoftmaxPolicyBuilder {
    beta: f32,
}

impl SoftmaxPolicyBuilder {
    pub fn new(beta: f32) -> SoftmaxPolicyBuilder {
        SoftmaxPolicyBuilder { beta: beta }
    }
}

pub struct SoftmaxPolicy<V> {
    pub beta: f32,
    pub vt: V,
}

impl<V> SoftmaxPolicy<V> {
    pub fn new(beta: f32, vt: V) -> SoftmaxPolicy<V> {
        SoftmaxPolicy { beta, vt }
    }
}

impl<M: ActionAvailability + StateEnumerable + ActionEnumerable + PMass<f32> + Cost>
    Policy<M::Action, M> for SoftmaxPolicy<ValueTable<M::State>>
{
    fn get_probability(&self, s: &M::State, a: &M::Action, mdp: &M) -> f32 {
        let min_qsa = self.vt.get_value_ssp(s, mdp);
        let result = ((self.beta * (-1.0) * (self.vt.get_qsa_ssp(s, a, mdp) - min_qsa)).exp())
            / (mdp
                .enumerate_actions()
                .filter(|a| mdp.action_available(s, a))
                .map(|at| (self.beta * (-1.0) * (self.vt.get_qsa_ssp(s, at, mdp) - min_qsa)).exp())
                .sum::<f32>());
        if result == 0.0 {
            1e-6
        } else {
            result
        }
    }
}

impl<M: ActionAvailability + StateEnumerable + ActionEnumerable + PMass<f32> + Cost>
    PolicyMut<M::Action, M> for SoftmaxPolicy<ValueTable<M::State>>
{
    fn get_probability_mut(
        &mut self,
        s: &<M as StatesActions>::State,
        a: &M::Action,
        mdp: &mut M,
    ) -> f32 {
        self.get_probability(s, a, mdp)
    }
}

impl<
        'a,
        M: ActionAvailability + StateEnumerable + ActionEnumerable + PMass<f32> + Cost + IsTerminal,
    > BuildFrom<&'a M, SoftmaxPolicy<ValueTable<M::State>>> for SoftmaxPolicyBuilder
{
    fn build_from(&self, mdp: &'a M) -> SoftmaxPolicy<ValueTable<M::State>> {
        let vt = value_iteration_ssp(mdp);
        SoftmaxPolicy::new(self.beta, vt)
    }
}

impl<M: ActionAvailability + StateEnumerable + ActionEnumerable + PMass<f32> + Cost>
    CostEstimator<M> for SoftmaxPolicy<ValueTable<M::State>>
{
    fn get_qsa_ssp(&self, s: &M::State, a: &M::Action, mdp: &M) -> f32 {
        self.vt.get_qsa_ssp(s, a, mdp)
    }
    fn get_value_ssp(&self, s: &M::State, mdp: &M) -> f32 {
        self.vt.get_value_ssp(s, mdp)
    }
}

pub struct SoftmaxRewardPolicy<M: StatesActions> {
    pub beta: f32,
    pub vt: ValueTable<M::State>,
}

impl<M: StatesActions> SoftmaxRewardPolicy<M> {
    pub fn new(beta: f32, vt: ValueTable<M::State>) -> SoftmaxRewardPolicy<M> {
        SoftmaxRewardPolicy { beta, vt }
    }
}

impl<M: ActionEnumerable + StateEnumerable + PMass<f32> + Rsa + DiscountFactor> Policy<M::Action, M>
    for SoftmaxRewardPolicy<M>
{
    fn get_probability(&self, s: &M::State, a: &M::Action, mdp: &M) -> f32 {
        ((self.beta * self.vt.get_qsa(s, a, mdp)).exp())
            / (mdp
                .enumerate_actions()
                .into_iter()
                .map(|at| (self.beta * self.vt.get_qsa(s, at, mdp)).exp())
                .sum::<f32>())
    }
}

// impl<M: StatesActions> Builder<ValueTable<M::State>, SoftmaxRewardPolicy<M>>
//     for SoftmaxPolicyBuilder
// {
//     fn build(&self, goal: ValueTable<M::State>) -> SoftmaxRewardPolicy<M> {
//         SoftmaxRewardPolicy::new(self.beta, goal)
//     }
// }

// impl<M: StateEnumerable + ActionEnumerable + PMass<f32> + ReWardMDP + DiscountFactor>
//     Builder<&M, SoftmaxRewardPolicy<M>> for SoftmaxPolicyBuilder
// {
//     fn build(&self, mdp: &M) -> SoftmaxRewardPolicy<M> {
//         let vt = value_iteration(mdp);
//         SoftmaxRewardPolicy::new(self.beta, vt)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid_world::GridWorldAction::*;
    use crate::grid_world::{GridWorldMDP, GridWorldState};
    use crate::value_iteration::value_iteration_ssp;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_grid_world_softmax() {
        let mdp = GridWorldMDP::default();
        let value_table = value_iteration_ssp(&mdp);
        let softmax_policy = SoftmaxPolicy::new(1.0, value_table);
        let err = 1e-3;
        assert_approx_eq!(
            0.40183,
            softmax_policy.get_probability(&GridWorldState::new(0, 0), &AttemptRight, &mdp,),
            err
        );
        assert_approx_eq!(
            0.287654,
            softmax_policy.get_probability(&GridWorldState::new(0, 0), &AttemptDown, &mdp,),
            err
        );
        assert_approx_eq!(
            0.15698,
            softmax_policy.get_probability(&GridWorldState::new(0, 0), &AttemptUp, &mdp,),
            err
        );
        assert_approx_eq!(
            0.1535276,
            softmax_policy.get_probability(&GridWorldState::new(0, 0), &AttemptLeft, &mdp,),
            err
        );
    }
}
