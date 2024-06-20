use crate::traits::ProbSassGivenTheta;

use mdp::into_inner::Inner;
use mdp::mdp_traits::*;
use std::fmt::Debug;
use std::hash::Hash;

use num_traits::cast::FromPrimitive;
use ordered_float::*;

use super::belief_tuple::BeliefTuple;
use super::oamdp::OAMDP;

impl<
        OM,
        M: StatesActions,
        A: Inner<Result = M::Action> + Debug + Copy + Hash + Eq,
        const N: usize,
    > OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A> + ActionEnumerable,
    for<'a> &'a OM: ProbSassGivenTheta<M::State, A>,
{
    pub fn get_new_belief(
        &self,
        previous_belief: &[NotNan<f32>; N],
        s: &M::State,
        a: &A,
        ss: &M::State,
    ) -> [NotNan<f32>; N] {
        let mut new_belief = [NotNan::<f32>::from_f32(0.0).unwrap(); N];
        let mut pags = [0.0; N];
        let mut pa = 0.0;
        for i in 0..N {
            pags[i] = self.assumed_model.prob_sass_given_theta(i, s, a, ss)
                * previous_belief[i].into_inner();

            pa += pags[i];
        }
        if pa <= 0.0 {
            return *previous_belief;
        }
        for i in 0..N {
            new_belief[i] = NotNan::<f32>::from_f32(pags[i] / pa).unwrap();
        }

        new_belief
    }
}

impl<
        OM,
        M: StatesActions,
        A: Inner<Result = M::Action> + Debug + Copy + Hash + Eq,
        const N: usize,
    > OAMDP<OM, M, A, N>
where
    Self: StatesActions<Action = A> + ActionEnumerable,
    for<'a> &'a mut OM: ProbSassGivenTheta<M::State, A>,
{
    pub fn get_new_belief_mut(
        &mut self,
        previous_belief: &[NotNan<f32>; N],
        s: &M::State,
        a: &A,
        ss: &M::State,
    ) -> [NotNan<f32>; N] {
        let tuple = BeliefTuple::new(*s, *a, *ss, *previous_belief);
        if let Some(belief) = self.cache.get(&tuple) {
            self.cache_hit += 1;
            *belief
        } else {
            self.cache_miss += 1;
            let mut new_belief = [NotNan::<f32>::from_f32(0.0).unwrap(); N];
            let mut pags = [0.0; N];
            let mut pa = 0.0;
            for i in 0..N {
                pags[i] = self.assumed_model.prob_sass_given_theta(i, s, a, ss)
                    * previous_belief[i].into_inner();

                pa += pags[i];
            }
            if pa <= 0.0 {
                return *previous_belief;
            }
            for i in 0..N {
                new_belief[i] = NotNan::<f32>::from_f32(pags[i] / pa).unwrap();
            }

            self.cache.insert(tuple, new_belief);
            new_belief
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::belief_cost_function::BeliefCostType;

    use crate::belief_update_type::ObserveabilityAssumption::*;
    use crate::oamdp::oamdp::OAMDP;
    use crate::oamdp::oamdp::OAMDP2;
    use crate::observer_model::ImplicitCommunicationModel;
    use assert_approx_eq::assert_approx_eq;

    use mdp::blocks_world::BlocksWorldAction::*;
    use mdp::blocks_world::Location::*;
    use mdp::blocks_world::*;

    use mdp::heuristic::ZeroHeuristic;
    use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicy;
    use rtdp::rtdp_softmax_policy::RTDPSoftmaxPolicyBuilder;
    #[test]
    #[allow(non_snake_case)]
    fn test_belief_changes() {
        let A = Block::new(0);
        let M = Block::new(1);
        let S = Block::new(2);
        let R = Block::new(3);
        let OnA = Location::On(A);
        let OnS = Location::On(S);
        let OnM = Location::On(M);
        let OnR = Location::On(R);

        let partial_mdp =
            BlocksWorldPartialMDP::new([OnTable, OnTable, OnM, OnTable], 0.1, ['A', 'M', 'S', 'R']);
        let possible_goals = [[OnR, OnS, OnTable, OnM], [OnM, OnS, OnTable, OnA]];
        let softmax_policy = RTDPSoftmaxPolicyBuilder::new(1.0);
        let mut oamdp: OAMDP<
            ImplicitCommunicationModel<
                RTDPSoftmaxPolicy<BlocksWorldStateN<4>, ZeroHeuristic>,
                BlocksWorldMDPN<4>,
                2,
            >,
            _,
            _,
            2,
        > = OAMDP2::new_implicit_model(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            0,
            BeliefCostType::Euclidean,
            crate::belief_cost_function::Objective::BeliefCostOnly,
            ActionObservable,
        );

        let trace = vec![
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnM, OnTable],
                },
                Some(PickUp(R)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnM, OnHold],
                },
                Some(PutDown(R, OnA)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnM, OnA],
                },
                Some(PickUp(S)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnHold, OnA],
                },
                Some(PutDown(S, OnTable)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnTable, OnA],
                },
                Some(PickUp(M)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnHold, OnTable, OnA],
                },
                Some(PutDown(M, OnS)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnS, OnTable, OnA],
                },
                Some(PickUp(R)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnS, OnTable, OnHold],
                },
                Some(PutDown(R, OnM)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnS, OnTable, OnM],
                },
                Some(PickUp(A)),
            ),
            (
                BlocksWorldState {
                    locations: [OnHold, OnS, OnTable, OnM],
                },
                Some(PutDown(A, OnR)),
            ),
            (
                BlocksWorldState {
                    locations: [OnR, OnS, OnTable, OnM],
                },
                None,
            ),
        ];

        let belief_changes = oamdp.get_belief_changes_mut(&trace);
        assert_approx_eq!(0.5, belief_changes[0][0].into_inner(), 1e-2);
        assert_approx_eq!(0.5, belief_changes[0][1].into_inner(), 1e-2);
        assert_approx_eq!(0.5, belief_changes[1][0].into_inner(), 1e-2);
        assert_approx_eq!(0.5, belief_changes[1][1].into_inner(), 1e-2);
        assert_approx_eq!(0.8486047176191396, belief_changes[2][0].into_inner(), 1e-2);
        assert_approx_eq!(0.1529, belief_changes[2][1].into_inner(), 1e-2);
        assert_approx_eq!(0.858, belief_changes[3][0].into_inner(), 1e-2);
        assert_approx_eq!(0.141, belief_changes[3][1].into_inner(), 1e-2);
        assert_approx_eq!(0.858, belief_changes[4][0].into_inner(), 1e-2);
        assert_approx_eq!(0.141, belief_changes[4][1].into_inner(), 1e-2);
        assert_approx_eq!(0.869, belief_changes[5][0].into_inner(), 1e-2);
        assert_approx_eq!(0.1308, belief_changes[5][1].into_inner(), 1e-2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_belief_changes_action_not_observed() {
        let A = Block::new(0);
        let M = Block::new(1);
        let S = Block::new(2);
        let R = Block::new(3);
        let OnA = Location::On(A);
        let OnS = Location::On(S);
        let OnM = Location::On(M);
        let OnR = Location::On(R);

        let partial_mdp =
            BlocksWorldPartialMDP::new([OnTable, OnTable, OnM, OnTable], 0.1, ['A', 'M', 'S', 'R']);
        let possible_goals = [[OnR, OnS, OnTable, OnM], [OnM, OnS, OnTable, OnA]];
        let softmax_policy = RTDPSoftmaxPolicyBuilder::new(1.0);

        let mut oamdp: OAMDP<
            ImplicitCommunicationModel<
                RTDPSoftmaxPolicy<BlocksWorldStateN<4>, ZeroHeuristic>,
                BlocksWorldMDPN<4>,
                2,
            >,
            _,
            _,
            2,
        > = OAMDP2::new_implicit_model(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            0,
            BeliefCostType::Euclidean,
            crate::belief_cost_function::Objective::BeliefCostOnly,
            ActionNotObservable,
        );

        let trace = vec![
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnM, OnTable],
                },
                Some(PickUp(R)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnM, OnHold],
                },
                Some(PutDown(R, OnA)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnM, OnA],
                },
                Some(PickUp(S)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnHold, OnA],
                },
                Some(PutDown(S, OnTable)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnTable, OnTable, OnA],
                },
                Some(PickUp(M)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnHold, OnTable, OnA],
                },
                Some(PutDown(M, OnS)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnS, OnTable, OnA],
                },
                Some(PickUp(R)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnS, OnTable, OnHold],
                },
                Some(PutDown(R, OnM)),
            ),
            (
                BlocksWorldState {
                    locations: [OnTable, OnS, OnTable, OnM],
                },
                Some(PickUp(A)),
            ),
            (
                BlocksWorldState {
                    locations: [OnHold, OnS, OnTable, OnM],
                },
                Some(PutDown(A, OnR)),
            ),
            (
                BlocksWorldState {
                    locations: [OnR, OnS, OnTable, OnM],
                },
                None,
            ),
        ];

        let belief_changes = oamdp.get_belief_changes_mut(&trace);
        println!("{:?}", belief_changes);
        assert_approx_eq!(0.5, belief_changes[0][0].into_inner(), 1e-2);
        assert_approx_eq!(0.5, belief_changes[0][1].into_inner(), 1e-2);
        assert_approx_eq!(0.5, belief_changes[1][0].into_inner(), 1e-2);
        assert_approx_eq!(0.5, belief_changes[1][1].into_inner(), 1e-2);
        assert_approx_eq!(0.8486047176191396, belief_changes[2][0].into_inner(), 1e-2);
        assert_approx_eq!(0.1529, belief_changes[2][1].into_inner(), 1e-2);
        assert_approx_eq!(0.858, belief_changes[3][0].into_inner(), 1e-2);
        assert_approx_eq!(0.141, belief_changes[3][1].into_inner(), 1e-2);
        assert_approx_eq!(0.858, belief_changes[4][0].into_inner(), 1e-2);
        assert_approx_eq!(0.141, belief_changes[4][1].into_inner(), 1e-2);
        assert_approx_eq!(0.869, belief_changes[5][0].into_inner(), 1e-2);
        assert_approx_eq!(0.1308, belief_changes[5][1].into_inner(), 1e-2);
    }
}
