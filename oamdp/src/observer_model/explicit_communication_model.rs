use std::convert::TryInto;

use mdp::into_inner::Inner;
use mdp::mdp_traits::StatesActions;
use mdp::mdp_traits::*;
use mdp::policy::policy_traits::{GetActionProbability, GetActionProbabilityMut};

use crate::belief_update_type::ObserveabilityAssumption;

use crate::traits::{CommunicationProbability, Message, ProbSassGivenTheta};

pub struct ExplicitCommunicationModel<P, M, C, const N: usize> {
    pub mdp_for_each_goal: [M; N],
    pub assumed_policy: [P; N],
    pub observability_assumption: ObserveabilityAssumption,
    pub communication_model: C,
}

impl<P, M: StatesActions, C, const N: usize> ExplicitCommunicationModel<P, M, C, N> {
    pub fn new(
        mdp_for_each_goal: [M; N],
        assumed_policy: [P; N],
        observability_assumption: ObserveabilityAssumption,
        communication_model: C,
    ) -> Self {
        ExplicitCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            observability_assumption,
            communication_model,
        }
    }

    pub fn get_mdp_for_goal(&self, id: usize) -> &M {
        &self.mdp_for_each_goal[id]
    }
}

impl<
        P,
        M: ActionEnumerable + ActionAvailability + ExplicitTransition + PMass<f32> + IsTerminal + Cost,
        C,
        const N: usize,
    > ExplicitCommunicationModel<P, M, C, N>
{
    pub fn new_from_possible_goals<MF, MP, PF>(
        mdp_factory: &MF,
        policy_builder: &PF,
        possible_goals: [MP; N],
        observability_assumption: ObserveabilityAssumption,
        communication_model: C,
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

        ExplicitCommunicationModel {
            mdp_for_each_goal,
            assumed_policy,
            observability_assumption,
            communication_model,
        }
    }
}

impl<
        'a,
        P: GetActionProbability<M::Action, M>,
        M: StatesActions + ExplicitTransition + ActionEnumerable,
        A: Inner<Result = M::Action> + Copy + Clone + Message,
        C: CommunicationProbability<A::Message>,
        const N: usize,
    > ProbSassGivenTheta<M::State, A> for &'a ExplicitCommunicationModel<P, M, C, N>
where
    M::Action: Inner<Result = M::Action>,
    A::Message: From<A>,
{
    fn prob_sass_given_theta(self, id: usize, s: &M::State, a: &A, _ss: &M::State) -> f32 {
        match self.observability_assumption {
            ObserveabilityAssumption::OnlyActionsAreConsidered => {
                self.assumed_policy[id].get_action_probability(
                    s,
                    &a.inner(),
                    &self.mdp_for_each_goal[id],
                ) * self
                    .communication_model
                    .communication_probability(id, &(*a).into())
            }
            _ => panic!("Not implemented"),
        }
    }
}

impl<
        'a,
        P: GetActionProbabilityMut<M::Action, M>,
        M: StatesActions + ExplicitTransition + ActionEnumerable,
        A: Inner<Result = M::Action> + Message,
        C: CommunicationProbability<A::Message>,
        const N: usize,
    > ProbSassGivenTheta<M::State, A> for &'a mut ExplicitCommunicationModel<P, M, C, N>
where
    M::Action: Inner<Result = M::Action>,
    A::Message: From<A>,
{
    fn prob_sass_given_theta(self, id: usize, s: &M::State, a: &A, _ss: &M::State) -> f32 {
        match self.observability_assumption {
            ObserveabilityAssumption::OnlyActionsAreConsidered => self.assumed_policy[id]
                .get_action_probability_mut(s, &a.inner(), &mut self.mdp_for_each_goal[id]),
            _ => panic!("Not implemented"),
        }
    }
}
