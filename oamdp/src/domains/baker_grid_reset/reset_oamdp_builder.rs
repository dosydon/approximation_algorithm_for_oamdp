use mdp::{
    baker_grid::{
        baker_grid_reset::{BakerGridResetBuilder, BakerGridResetMDP},
        BakerGridAction, BakerGridState,
    },
    finite_horizon_wrapper::FiniteHorizonWrapper,
    mdp_traits::Build,
    policy::softmax_policy::SoftmaxPolicyBuilder,
};
use serde::{Deserialize, Serialize};

use crate::{
    belief_cost_function::Objective,
    domains::baker_grid::BakerOAMDPBuilder,
    oamdp::{oamdp::OAMDP, OAMDPFiniteHorizon},
    observer_model::SoftmaxModel,
};

#[derive(Serialize, Deserialize)]
pub struct BakerResetOAMDPBuilder<const N: usize> {
    builder: BakerOAMDPBuilder<N>,
    reset_prob: f32,
    reset_states: Vec<BakerGridState>,
    objective: Objective,
}

fn reset_states(instance_id: usize) -> Vec<BakerGridState> {
    match instance_id {
        //         302 => vec![
        //             BakerGridState::new(6, 3),
        //             BakerGridState::new(3, 6),
        //             BakerGridState::new(6, 9),
        //             BakerGridState::new(9, 6),
        //         ],
        //         303 => vec![
        //             BakerGridState::new(6, 3),
        //             BakerGridState::new(3, 6),
        //             BakerGridState::new(6, 9),
        //             BakerGridState::new(9, 6),
        //         ],
        _ => vec![],
    }
}

impl<const N: usize> BakerResetOAMDPBuilder<N> {
    pub fn new(instance_id: usize) -> Self {
        let builder = BakerOAMDPBuilder::<N>::new(instance_id);
        BakerResetOAMDPBuilder {
            builder,
            reset_prob: 0.1,
            reset_states: reset_states(instance_id),
            objective: Objective::LinearCombination(1.0, 0.1),
        }
    }

    pub fn set_horizon(mut self, horizon: usize) -> Self {
        self.builder.horizon = horizon;
        self
    }
}

impl<const N: usize>
    Build<
        OAMDPFiniteHorizon<
            SoftmaxModel<BakerGridResetMDP, N>,
            BakerGridResetMDP,
            BakerGridAction,
            N,
        >,
    > for BakerResetOAMDPBuilder<N>
{
    fn build(
        self,
    ) -> OAMDPFiniteHorizon<SoftmaxModel<BakerGridResetMDP, N>, BakerGridResetMDP, BakerGridAction, N>
    {
        FiniteHorizonWrapper::new(self.build_oamdp(), self.builder.horizon)
    }
}
impl<const N: usize> BakerResetOAMDPBuilder<N> {
    pub fn build_oamdp(
        &self,
    ) -> OAMDP<SoftmaxModel<BakerGridResetMDP, N>, BakerGridResetMDP, BakerGridAction, N> {
        let partial_mdp = self
            .builder
            .partial_mdp
            .clone()
            .set_prob_veering(self.builder.prob_veering);
        let partial_mdp =
            BakerGridResetBuilder::new(partial_mdp, self.reset_prob, self.reset_states.clone());
        let softmax_policy = SoftmaxPolicyBuilder::new(self.builder.beta);
        let possible_goals = self.builder.possible_goals;

        let mut oamdp = OAMDP::new_with_initial_belief(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            self.builder.true_goal,
            self.builder.belief_cost_type.clone(),
            self.objective,
            self.builder.observability_assumption,
            self.builder.initial_belief,
        );
        oamdp.gamma = self.builder.gamma;
        if let Some(i) = self.builder.random {
            oamdp.assumed_model.assumed_policy[i].beta = 0.0;
        }
        oamdp
    }
}
