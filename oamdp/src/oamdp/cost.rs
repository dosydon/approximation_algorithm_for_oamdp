use super::oamdp::OAMDP;

use crate::belief_cost_function::Objective;
use crate::observer_model::ImplicitCommunicationModel;
use crate::traits::{BeliefOverGoal, CommunicationCost};
use mdp::into_inner::Inner;
use mdp::mdp_traits::*;
use std::fmt::Debug;
use std::hash::Hash;

impl<P, M: StatesActions, A: Eq + Copy + Debug + Hash, const N: usize> CommunicationCost
    for OAMDP<ImplicitCommunicationModel<P, M, N>, M, A, N>
where
    Self: StatesActions<Action = A>,
{
    fn communication_cost(&self, _s: &Self::State, _a: &Self::Action) -> f32 {
        0.0
    }
}

impl<
        OM,
        M: StatesActions + Cost,
        A: Eq + Copy + Debug + Hash + Inner<Result = M::Action>,
        const N: usize,
    > Cost for OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A> + ActionEnumerable + IsTerminal,
    Self::State: Inner<Result = M::State> + BeliefOverGoal<N>,
    Self: CommunicationCost,
{
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        if self.is_terminal(s) {
            0.0
        } else {
            let b = s.get_belief_over_goal();
            match self.objective {
                Objective::BeliefCostOnly => self.distance_measure.b_cost(&b),
                Objective::LinearCombination(c, d) => {
                    let b_cost = self.distance_measure.b_cost(&b);
                    let s_cost =
                        self.mdp.cost(&s.inner(), &a.inner()) + self.communication_cost(s, a);
                    c * b_cost + d * s_cost
                }
            }
        }
    }
}

impl<
        OM,
        M: StatesActions + DCost,
        A: Eq + Copy + Debug + Hash + Inner<Result = M::Action>,
        const N: usize,
    > DCost for OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A> + IsTerminal,
    Self::State: Inner<Result = M::State> + BeliefOverGoal<N>,
    Self: CommunicationCost,
{
    fn d_cost(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        if self.is_terminal(st) {
            0.0
        } else {
            //         let b = stt.get_belief_over_goal();
            let b = st.get_belief_over_goal();
            match self.objective {
                Objective::BeliefCostOnly => self.distance_measure.b_cost(&b),
                Objective::LinearCombination(c, d) => {
                    let b_cost = self.distance_measure.b_cost(&b);
                    let s_cost = self.mdp.d_cost(&st.inner(), &a.inner(), &stt.inner())
                        + self.communication_cost(st, a);
                    //                 println!("b_cost: {}, s_cost: {}", b_cost, s_cost);
                    c * b_cost + d * s_cost
                }
            }
        }
    }
}
