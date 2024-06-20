use crate::traits::BeliefOverGoal;
use core::fmt::Debug;
use core::hash::Hash;

use mdp::into_inner::Inner;
use mdp::mdp_traits::*;

use crate::traits::ProbSassGivenTheta;

use super::oamdp::OAMDP;
use super::BeliefState;

impl<
        OM,
        M: StatesActions,
        A: Eq + Hash + Copy + Debug + Inner<Result = M::Action>,
        const N: usize,
    > PMass<f32> for OAMDP<OM, M, A, N>
where
    Self: StatesActions<State = BeliefState<M::State, N>, Action = A> + ActionEnumerable,
    M: PMass<f32, Distribution = Vec<(<M as StatesActions>::State, f32)>>,
    for<'a> &'a OM: ProbSassGivenTheta<M::State, A>,
{
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &A) -> Vec<(Self::State, f32)> {
        self.mdp
            .p_mass(&s.inner(), &a.inner())
            .iter()
            .map(|(new_s, c)| {
                let new_belief =
                    self.get_new_belief(&s.get_belief_over_goal(), &s.inner(), a, &new_s);

                (Self::State::new(*new_s, new_belief), *c)
            })
            .collect::<Vec<_>>()
    }
}

impl<
        OM,
        M: PMass<f32, Distribution = Vec<(<M as StatesActions>::State, f32)>>,
        A: Eq + Hash + Copy + Debug + Inner<Result = M::Action>,
        const N: usize,
    > PMassMut<f32> for OAMDP<OM, M, A, N>
where
    Self: StatesActions<State = BeliefState<M::State, N>, Action = A> + ActionEnumerable,
    for<'a> &'a mut OM: ProbSassGivenTheta<M::State, A>,
{
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass_mut(&mut self, s: &Self::State, a: &A) -> Vec<(Self::State, f32)> {
        self.mdp
            .p_mass(&s.inner(), &a.inner())
            .iter()
            .map(|(new_s, c)| {
                let new_belief =
                    self.get_new_belief_mut(&s.get_belief_over_goal(), &s.inner(), a, &new_s);

                (Self::State::new(*new_s, new_belief), *c)
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::belief_cost_function::{BeliefCostType, Objective};
    use crate::belief_update_type::ObserveabilityAssumption::*;
    use crate::oamdp::oamdp::OAMDP3;

    use mdp::baker_grid::*;

    use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;

    #[test]
    fn test_oamdp_p_mass() {
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

        let mut oamdp: OAMDP3<_, _> = OAMDP::new_implicit_model(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            0,
            BeliefCostType::Euclidean,
            Objective::BeliefCostOnly,
            OnlyActionsAreConsidered,
        );
        let s = oamdp.initial_state();
        let a = BakerGridAction::East;
        println!("{:?}", oamdp.p_mass(&s, &a));
        println!("{:?}", oamdp.p_mass_mut(&s, &a));
        println!("{:?}", oamdp.p_mass_mut(&s, &a));
    }
}
