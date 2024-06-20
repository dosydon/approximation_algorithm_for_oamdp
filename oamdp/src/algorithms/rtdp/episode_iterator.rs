use mdp::heuristic::HeuristicWithMDPMut;
use mdp::into_inner::Inner;
use mdp::mdp_traits::StatesActions;
use mdp::mdp_traits::*;
use std::fmt::Debug;
use std::hash::Hash;

use crate::oamdp::oamdp::OAMDP;
use crate::oamdp::BeliefState;
use crate::oamdp_d::{VState, OAMDPD};
use crate::traits::{BeliefOverGoal, ProbSassGivenTheta};

use super::RTDP_OAMDP;

pub struct EpisodeIterator<
    'a,
    OM,
    M: StatesActions,
    A: PartialEq + Eq + Copy + Clone + Debug + Hash,
    H,
    const N: usize,
> {
    rtdp: &'a mut RTDP_OAMDP<OM, M, A, H, N>,
    vs: VState<M::State, N>,
    bs: BeliefState<M::State, N>,
    t: usize,
    max_t: Option<usize>,
    rng: &'a mut rand::rngs::ThreadRng,
}

impl<
        'a,
        OM,
        M: StatesActions,
        A: PartialEq + Eq + Copy + Clone + Debug + Hash,
        H,
        const N: usize,
    > EpisodeIterator<'a, OM, M, A, H, N>
{
    pub fn from_initial_state(
        rtdp: &'a mut RTDP_OAMDP<OM, M, A, H, N>,
        vs: VState<M::State, N>,
        bs: BeliefState<M::State, N>,
        rng: &'a mut rand::rngs::ThreadRng,
    ) -> Self {
        EpisodeIterator {
            rtdp,
            vs,
            bs,
            t: 0,
            max_t: None,
            rng,
        }
    }

    pub fn set_max_t(mut self, max_t: Option<usize>) -> Self {
        self.max_t = max_t;
        self
    }
}

impl<
        'a,
        OM,
        M: StatesActions,
        A: PartialEq + Eq + Copy + Clone + Debug + Hash + Inner<Result = M::Action>,
        H,
        const N: usize,
    > Iterator for EpisodeIterator<'a, OM, M, A, H, N>
where
    OAMDPD<OM, M, A, N>: InitialState
        + StatesActions<State = VState<M::State, N>, Action = A>
        + PMassMut<f32>
        + Cost
        + IsTerminal
        + GetNextStateMut
        + ActionEnumerable
        + ActionAvailability,
    OAMDP<OM, M, A, N>: StatesActions<State = BeliefState<M::State, N>, Action = A>
        + GetNextStateMut
        + Cost
        + ActionEnumerable,
    H: HeuristicWithMDPMut<OAMDPD<OM, M, A, N>>,
    for<'b> &'b mut OM: ProbSassGivenTheta<M::State, A>,
{
    type Item = (BeliefState<M::State, N>, A, BeliefState<M::State, N>, f32);

    fn next(&mut self) -> Option<Self::Item> {
        //         println!("{:?}", self.bs);
        //         println!("{:?}", self.vs);
        //         println!("{:?}", self.rtdp.oamdp_d.to_belief_state(&self.vs));
        if self.rtdp.oamdp_d.is_terminal(&self.vs) {
            return None;
        }

        if let Some(max_t) = self.max_t {
            if self.t >= max_t {
                return None;
            }
        }

        if let Some(a) = self
            .rtdp
            .rtdp
            .best_action_mut(&self.vs, &mut self.rtdp.oamdp_d)
        {
            let c = self.rtdp.oamdp_d.oamdp.cost(&self.bs, &a);
            let next_bs = self
                .rtdp
                .oamdp_d
                .oamdp
                .get_next_state_mut(&self.bs, &a, self.rng);

            let b = self
                .rtdp
                .oamdp_d
                .to_belief_state(&self.vs)
                .get_belief_over_goal();
            let b_sas = self.rtdp.oamdp_d.oamdp.get_new_belief_mut(
                &b,
                &self.bs.inner(),
                &a,
                &next_bs.inner(),
            );
            let next_vs = self
                .rtdp
                .oamdp_d
                .random_transition_to_v_state(&BeliefState::new(next_bs.inner(), b_sas), self.rng);

            let prev_bs = self.bs;
            self.vs = next_vs;
            self.bs = next_bs;
            self.t += 1;

            Some((prev_bs, a, self.bs, c))
        } else {
            None
        }
    }
}
