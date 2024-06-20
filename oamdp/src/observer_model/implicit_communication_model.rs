use mdp::into_inner::Inner;
use mdp::mdp_traits::StatesActions;
use mdp::mdp_traits::*;
use mdp::policy::policy_traits::{GetActionProbability, GetActionProbabilityMut};

use crate::belief_update_type::ObserveabilityAssumption;

use crate::traits::ProbSassGivenTheta;

pub struct ImplicitCommunicationModel<P, M, const N: usize> {
    pub mdp_for_each_goal: [M; N],
    pub assumed_policy: [P; N],
    pub observability_assumption: ObserveabilityAssumption,
}

impl<P, M: StatesActions, const N: usize> ImplicitCommunicationModel<P, M, N> {
    pub fn new(
        mdp_for_each_goal: [M; N],
        assumed_policy: [P; N],
        observability_assumption: ObserveabilityAssumption,
    ) -> Self {
        ImplicitCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            observability_assumption,
        }
    }

    pub fn get_mdp_for_goal(&self, id: usize) -> &M {
        &self.mdp_for_each_goal[id]
    }
}

impl<
        'a,
        P: GetActionProbability<M::Action, M>,
        M: StatesActions + ExplicitTransition + ActionEnumerable,
        A: Inner<Result = M::Action>,
        const N: usize,
    > ProbSassGivenTheta<M::State, A> for &'a ImplicitCommunicationModel<P, M, N>
where
    M::Action: Inner<Result = M::Action>,
{
    fn prob_sass_given_theta(self, id: usize, s: &M::State, a: &A, ss: &M::State) -> f32 {
        match self.observability_assumption {
            ObserveabilityAssumption::ActionNotObservable => self.mdp_for_each_goal[id]
                .enumerate_actions()
                .map(|a| {
                    self.assumed_policy[id].get_action_probability(
                        s,
                        &a.inner(),
                        &self.mdp_for_each_goal[id],
                    ) * self.mdp_for_each_goal[id].p(s, &a.inner(), ss)
                })
                .sum(),
            ObserveabilityAssumption::ActionObservable => {
                self.assumed_policy[id].get_action_probability(
                    s,
                    &a.inner(),
                    &self.mdp_for_each_goal[id],
                ) * self.mdp_for_each_goal[id].p(s, &a.inner(), ss)
            }
            ObserveabilityAssumption::OnlyActionsAreConsidered => self.assumed_policy[id]
                .get_action_probability(s, &a.inner(), &self.mdp_for_each_goal[id]),
        }
    }
}

impl<
        'a,
        P: GetActionProbabilityMut<M::Action, M>,
        M: StatesActions + ExplicitTransition + ActionEnumerable,
        A: Inner<Result = M::Action>,
        const N: usize,
    > ProbSassGivenTheta<M::State, A> for &'a mut ImplicitCommunicationModel<P, M, N>
where
    M::Action: Inner<Result = M::Action>,
{
    fn prob_sass_given_theta(self, id: usize, s: &M::State, a: &A, ss: &M::State) -> f32 {
        match self.observability_assumption {
            ObserveabilityAssumption::ActionNotObservable => (0..self.mdp_for_each_goal[id]
                .num_actions())
                .map(|aa_id| {
                    let aa = *self.mdp_for_each_goal[id].id_to_action(aa_id);
                    self.assumed_policy[id].get_action_probability_mut(
                        s,
                        &aa,
                        &mut self.mdp_for_each_goal[id],
                    ) * self.mdp_for_each_goal[id].p(s, &aa.inner(), ss)
                })
                .sum(),
            ObserveabilityAssumption::ActionObservable => {
                self.assumed_policy[id].get_action_probability_mut(
                    s,
                    &a.inner(),
                    &mut self.mdp_for_each_goal[id],
                ) * self.mdp_for_each_goal[id].p(s, &a.inner(), ss)
            }
            ObserveabilityAssumption::OnlyActionsAreConsidered => self.assumed_policy[id]
                .get_action_probability_mut(s, &a.inner(), &mut self.mdp_for_each_goal[id]),
        }
    }
}
