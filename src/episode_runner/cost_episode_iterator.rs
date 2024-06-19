use crate::{
    mdp_traits::{DCost, GetNextState, GetNextStateMut, InitialState, IsTerminal, StatesActions},
    policy::policy_traits::{GetAction, GetActionMut},
};

use super::{episode_iterator::EpisodeIteratorMut, EpisodeIterator};

pub struct CostEpisodeIterator<'a, M: StatesActions, P> {
    inner: EpisodeIterator<'a, M, P>,
}

impl<'a, M: StatesActions, P> CostEpisodeIterator<'a, M, P> {
    pub fn new(
        env: &'a M,
        policy: &'a P,
        s: M::State,
        rng: &'a mut rand::rngs::ThreadRng,
        max_time_step: Option<usize>,
    ) -> Self {
        CostEpisodeIterator {
            inner: EpisodeIterator::new(env, policy, s, rng, max_time_step),
        }
    }
}

impl<'a, M: StatesActions + InitialState, P> CostEpisodeIterator<'a, M, P> {
    pub fn from_initial_state(
        env: &'a M,
        policy: &'a P,
        rng: &'a mut rand::rngs::ThreadRng,
    ) -> Self {
        CostEpisodeIterator {
            inner: EpisodeIterator::from_initial_state(env, policy, rng),
        }
    }
}

impl<M, P> Iterator for CostEpisodeIterator<'_, M, P>
where
    M: StatesActions + GetNextState + IsTerminal + DCost,
    P: GetAction<M::State, M>,
{
    type Item = (M::State, M::Action, M::State, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((s, a, ss)) = self.inner.next() {
            let c = self.inner.env.d_cost(&s, &a, &s);
            Some((s, a, ss, c))
        } else {
            None
        }
    }
}

pub struct CostEpisodeIteratorMut<'a, M: StatesActions, P> {
    inner: EpisodeIteratorMut<'a, M, P>,
}

impl<'a, M: StatesActions, P> CostEpisodeIteratorMut<'a, M, P> {
    pub fn new(
        env: &'a mut M,
        policy: &'a mut P,
        s: M::State,
        rng: &'a mut rand::rngs::ThreadRng,
        max_time_step: Option<usize>,
    ) -> Self {
        CostEpisodeIteratorMut {
            inner: EpisodeIteratorMut::new(env, policy, s, rng, max_time_step),
        }
    }
}

impl<'a, M: StatesActions + InitialState, P> CostEpisodeIteratorMut<'a, M, P>
where
    M::State: Copy,
{
    pub fn from_initial_state(
        env: &'a mut M,
        policy: &'a mut P,
        rng: &'a mut rand::rngs::ThreadRng,
    ) -> Self {
        CostEpisodeIteratorMut {
            inner: EpisodeIteratorMut::from_initial_state(env, policy, rng),
        }
    }
}

impl<M, P> Iterator for CostEpisodeIteratorMut<'_, M, P>
where
    M: StatesActions + GetNextStateMut + IsTerminal + DCost,
    P: GetActionMut<M::State, M>,
{
    type Item = (M::State, M::Action, M::State, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((s, a, ss)) = self.inner.next() {
            let c = self.inner.env.d_cost(&s, &a, &s);
            Some((s, a, ss, c))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::*;
    use crate::{
        baker_grid::{BakerGridMDP, BakerGridState},
        mdp_traits::InitialState,
        policy::tabular_policy::TabularPolicy,
        value_iteration::value_iteration_ssp,
    };

    #[test]
    fn test_episode_iterator() {
        let mdp = BakerGridMDP::new(
            5,
            5,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            BakerGridState::new(4, 4),
        );

        let vt = value_iteration_ssp(&mdp);
        let policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        let mut rng = thread_rng();
        let s = mdp.initial_state();

        for (s, a, ss, r) in CostEpisodeIterator::new(&mdp, &policy, s, &mut rng, None) {
            println!("{:?} {:?} {:?} {:?}", s, a, ss, r);
        }
    }
}
