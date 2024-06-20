use crate::belief_cost_function::*;
use crate::belief_update_type::ObserveabilityAssumption;
use crate::observer_model::ImplicitCommunicationModel;
use core::fmt::Debug;
use core::hash::Hash;
use mdp::mdp_traits::*;
use std::collections::HashMap;

use num_traits::cast::FromPrimitive;
use ordered_float::*;

use super::oamdp::OAMDP;

impl<OM, M: StatesActions, A: Eq + Debug + Hash + Copy, const N: usize> OAMDP<OM, M, A, N> {
    pub fn new(
        assumed_model: OM,
        mdp: M,
        distance_measure: BeliefCostFunction<N>,
        initial_belief: [NotNan<f32>; N],
        gamma: f32,
        all_actions: Vec<A>,
        objective: Objective,
    ) -> OAMDP<OM, M, A, N> {
        OAMDP {
            assumed_model,
            mdp,
            distance_measure,
            initial_belief,
            gamma,
            all_actions,
            objective,
            cache: HashMap::new(),
            cache_hit: 0,
            cache_miss: 0,
        }
    }

    pub fn print_cache_stats(&self) {
        println!("Cache hit: {}", self.cache_hit);
        println!("Cache miss: {}", self.cache_miss);
    }
}

impl<
        P,
        M: ActionEnumerable + ActionAvailability + ExplicitTransition + PMass<f32> + IsTerminal + Cost,
        A: Eq + Debug + Hash + Copy,
        const N: usize,
    > OAMDP<ImplicitCommunicationModel<P, M, N>, M, A, N>
where
    M: StatesActions<Action = A>,
{
    pub fn new_implicit_model<MP, MF, PF>(
        mdp_factory: &MF,
        policy_factory: &PF,
        possible_goals: [MP; N],
        true_goal: usize,
        distance_measure: BeliefCostType,
        objective: Objective,
        observability_assumption: ObserveabilityAssumption,
    ) -> OAMDP<ImplicitCommunicationModel<P, M, N>, M, A, N>
    where
        for<'a> MF: BuildFrom<&'a MP, M>,
        for<'a> PF: BuildFrom<&'a M, P>,
    {
        OAMDP::new_with_initial_belief(
            mdp_factory,
            policy_factory,
            possible_goals,
            true_goal,
            distance_measure,
            objective,
            observability_assumption,
            [NotNan::<f32>::from_f32(1.0 / N as f32).unwrap(); N],
        )
    }
}

impl<
        P,
        M: ActionEnumerable + ActionAvailability + ExplicitTransition + PMass<f32> + IsTerminal + Cost,
        A: Eq + Debug + Hash + Copy,
        const N: usize,
    > OAMDP<ImplicitCommunicationModel<P, M, N>, M, A, N>
where
    M: StatesActions<Action = A>,
{
    pub fn new_with_initial_belief<MP, MF, PF>(
        mdp_factory: &MF,
        policy_factory: &PF,
        possible_goals: [MP; N],
        true_goal: usize,
        distance_measure: BeliefCostType,
        objective: Objective,
        observability_assumption: ObserveabilityAssumption,
        initial_belief: [NotNan<f32>; N],
    ) -> OAMDP<ImplicitCommunicationModel<P, M, N>, M, A, N>
    where
        for<'a> MF: BuildFrom<&'a MP, M>,
        for<'a> PF: BuildFrom<&'a M, P>,
    {
        let mdp = mdp_factory.build_from(&possible_goals[true_goal]);
        let assumed_model = ImplicitCommunicationModel::new_from_possible_goals(
            mdp_factory,
            policy_factory,
            possible_goals,
            observability_assumption,
        );
        let all_actions = assumed_model.mdp_for_each_goal[true_goal]
            .enumerate_actions()
            .cloned()
            .collect();
        let mut target_belief = [NotNan::<f32>::from_f32(0.0).unwrap(); N];
        target_belief[true_goal] = NotNan::<f32>::from_f32(1.0).unwrap();

        let belief_cost_function = match distance_measure {
            BeliefCostType::Euclidean => BeliefCostFunction::Euclidean(target_belief),
            BeliefCostType::KLDivergence => BeliefCostFunction::KLDivergence(true_goal),
            BeliefCostType::Disimulation => BeliefCostFunction::Disimulation,
            BeliefCostType::Deceptive(i) => {
                let mut target_belief = [NotNan::<f32>::from_f32(0.0).unwrap(); N];
                target_belief[i] = NotNan::<f32>::from_f32(1.0).unwrap();
                BeliefCostFunction::TVDistance(target_belief)
            }
            BeliefCostType::TVDistance => BeliefCostFunction::TVDistance(target_belief),
        };

        OAMDP {
            assumed_model: assumed_model,
            mdp: mdp,
            distance_measure: belief_cost_function,
            initial_belief: initial_belief,
            gamma: 0.9,
            all_actions: all_actions,
            objective: objective,
            cache: HashMap::new(),
            cache_hit: 0,
            cache_miss: 0,
        }
    }
}
