use mdp::{
    into_inner::Inner,
    mdp_traits::{
        ActionAvailability, ActionEnumerable, Cost, DCost, InitialState, IsTerminal, StatesActions,
    },
};

use std::hash::Hash;
use std::{fmt::Debug, slice::Iter};

use crate::{
    oamdp::{oamdp::OAMDP, BeliefState},
    regular_grid_translator::RegularGridTranslator,
};

use super::VState;

pub struct OAMDPD<OM, M: StatesActions, A: Eq + Debug + Hash + Copy, const N: usize> {
    pub oamdp: OAMDP<OM, M, A, N>,
    pub translator: RegularGridTranslator<N>,
}

impl<OM, M: StatesActions, A: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize>
    OAMDPD<OM, M, A, N>
{
    pub fn new(oamdp: OAMDP<OM, M, A, N>, k: usize) -> Self {
        OAMDPD {
            oamdp: oamdp,
            translator: RegularGridTranslator::new(k),
        }
    }
}

impl<OM, M: StatesActions, A: Eq + PartialEq + Debug + Copy + Clone + Hash, const N: usize>
    OAMDPD<OM, M, A, N>
{
    pub fn to_belief_state(&self, vs: &VState<M::State, N>) -> BeliefState<M::State, N> {
        BeliefState::new(vs.inner(), self.translator.v_to_b(&vs.v))
    }

    //     pub(crate) fn to_v_state(&self, bs: &BeliefState<M::State, N>) -> VState<M::State, N> {
    //         VState::new(
    //             bs.into_inner(),
    //             self.translator.b_to_v(&bs.get_belief_over_goal()),
    //         )
    //     }
}

impl<OM, M: StatesActions, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    StatesActions for OAMDPD<OM, M, A, N>
{
    type State = VState<M::State, N>;
    type Action = A;
}

impl<
        OM,
        M: StatesActions + IsTerminal,
        A: Eq + PartialEq + Hash + Debug + Clone + Copy,
        const N: usize,
    > IsTerminal for OAMDPD<OM, M, A, N>
where
    Self: StatesActions<State = VState<M::State, N>, Action = A>,
{
    fn is_terminal(&self, s: &Self::State) -> bool {
        self.oamdp.mdp.is_terminal(&s.inner())
    }
}

impl<OM, M: ActionEnumerable, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    ActionEnumerable for OAMDPD<OM, M, A, N>
where
    OAMDP<OM, M, A, N>: ActionEnumerable + StatesActions<Action = A>,
{
    fn enumerate_actions(&self) -> Iter<Self::Action> {
        self.oamdp.enumerate_actions()
    }
    fn num_actions(&self) -> usize {
        self.oamdp.num_actions()
    }
    fn id_to_action(&self, id: usize) -> &Self::Action {
        self.oamdp.id_to_action(id)
    }
}

impl<OM, M: StatesActions, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    ActionAvailability for OAMDPD<OM, M, A, N>
{
    fn action_available(&self, _s: &Self::State, _a: &Self::Action) -> bool {
        true
    }
}

impl<OM, M: InitialState, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize>
    InitialState for OAMDPD<OM, M, A, N>
{
    fn initial_state(&self) -> VState<M::State, N> {
        let s = self.oamdp.initial_state();
        VState {
            s: s.inner(),
            v: [0; N],
            is_dummy_initial_state: true,
        }
    }
}

impl<OM, M: InitialState, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize> Cost
    for OAMDPD<OM, M, A, N>
where
    OAMDP<OM, M, A, N>: Cost + StatesActions<State = BeliefState<M::State, N>, Action = A>,
{
    fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
        if s.is_dummy_initial_state {
            0.0
        } else {
            let bs = self.to_belief_state(s);
            self.oamdp.cost(&bs, a)
        }
    }
}

impl<OM, M: InitialState, A: Eq + PartialEq + Hash + Debug + Clone + Copy, const N: usize> DCost
    for OAMDPD<OM, M, A, N>
where
    OAMDP<OM, M, A, N>: DCost + StatesActions<State = BeliefState<M::State, N>, Action = A>,
{
    fn d_cost(&self, s: &Self::State, a: &Self::Action, ss: &Self::State) -> f32 {
        if s.is_dummy_initial_state {
            0.0
        } else {
            let bs = self.to_belief_state(s);
            let bss = self.to_belief_state(ss);
            self.oamdp.d_cost(&bs, a, &bss)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::belief_cost_function::{BeliefCostType, Objective};
    use crate::belief_update_type::ObserveabilityAssumption::*;
    use crate::oamdp::oamdp::OAMDP3;

    use mdp::baker_grid::*;

    use mdp::heuristic::ZeroHeuristic;
    use mdp::mdp_traits::PMass;
    use mdp::policy::softmax_policy::SoftmaxPolicyBuilder;
    use rand::thread_rng;
    use rtdp::rtdp::RTDP;

    #[test]
    fn test_oamdp_d_new() {
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

        let oamdp: OAMDP3<_, _> = OAMDP::new_implicit_model(
            &partial_mdp,
            &softmax_policy,
            possible_goals,
            0,
            BeliefCostType::Euclidean,
            Objective::BeliefCostOnly,
            ActionNotObservable,
        );

        let s = oamdp.initial_state();
        let mut oamdp_d = OAMDPD::new(oamdp, 4);
        let vs = oamdp_d.initial_state();

        println!("{:?}", s);
        println!("{:?}", vs);
        println!("{:?}", oamdp_d.p_mass(&vs, &BakerGridAction::North));

        let h = ZeroHeuristic {};
        let mut rng = thread_rng();
        let mut rtdp = RTDP::new(h);
        rtdp.solve(&mut oamdp_d, &mut rng, 100);

        println!("{}", rtdp.vt.get_value(&oamdp_d.initial_state()))
    }
}
