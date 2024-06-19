use rand::rngs::ThreadRng;

use crate::{
    mdp_traits::{
        DCost, Eval, GetNextState, GetNextStateMut, InitialState, IsTerminal, Rsas, SetMaxHorizon,
        StatesActions,
    },
    policy::policy_traits::{GetAction, GetActionMut},
};

use super::{
    cost_episode_iterator::CostEpisodeIteratorMut, reward_episode_iterator::RewardEpisodeIterator,
    CostEpisodeIterator,
};

pub struct EpisodeRunner<'a, M: StatesActions, P> {
    env: &'a M,
    policy: &'a P,
    s: M::State,
    max_time_step: Option<usize>,
}

impl<'a, M: StatesActions, P> EpisodeRunner<'a, M, P> {
    pub fn new(env: &'a M, policy: &'a P, s: M::State) -> Self {
        EpisodeRunner {
            env,
            policy,
            s,
            max_time_step: None,
        }
    }

    //     pub fn set_max_step(mut self, max_time_step: usize) -> Self {
    //         self.max_time_step = Some(max_time_step);
    //         self
    //     }
}

impl<'a, M: StatesActions, P> SetMaxHorizon for EpisodeRunner<'a, M, P> {
    fn set_max_horizon(mut self, max_horizon: Option<usize>) -> Self {
        self.max_time_step = max_horizon;
        self
    }
}

impl<'a, M: StatesActions + InitialState, P> EpisodeRunner<'a, M, P> {
    pub fn from_initial_state(env: &'a M, policy: &'a P) -> Self {
        EpisodeRunner {
            env,
            policy,
            s: env.initial_state(),
            max_time_step: None,
        }
    }
}

impl<'a, M, P> EpisodeRunner<'a, M, P>
where
    M: StatesActions + InitialState + IsTerminal + GetNextState + DCost,
    P: GetAction<M::State, M>,
{
    pub fn into_iter_with<'b>(&'b mut self, rng: &'b mut ThreadRng) -> CostEpisodeIterator<'b, M, P>
    where
        'a: 'b,
    {
        CostEpisodeIterator::new(self.env, self.policy, self.s, rng, self.max_time_step)
    }
}

impl<M, P> Eval for EpisodeRunner<'_, M, P>
where
    M: StatesActions + InitialState + IsTerminal + GetNextState + DCost,
    P: GetAction<M::State, M>,
{
    fn eval(&mut self, rng: &mut ThreadRng) -> f32 {
        let mut cumulative_cost = 0.0;
        for (_, _, _, c) in self.into_iter_with(rng) {
            cumulative_cost += c;
        }
        cumulative_cost
    }
}

pub struct EpisodeRunnerMut<'a, M: StatesActions, P> {
    pub env: &'a mut M,
    pub policy: &'a mut P,
    pub s: M::State,
    pub max_time_step: Option<usize>,
}

impl<'a, M: StatesActions, P> EpisodeRunnerMut<'a, M, P> {
    pub fn new(env: &'a mut M, policy: &'a mut P, s: M::State) -> Self {
        EpisodeRunnerMut {
            env,
            policy,
            s,
            max_time_step: None,
        }
    }
}

impl<'a, M: StatesActions, P> SetMaxHorizon for EpisodeRunnerMut<'a, M, P> {
    fn set_max_horizon(mut self, max_horizon: Option<usize>) -> Self {
        self.max_time_step = max_horizon;
        self
    }
}

impl<'a, M: StatesActions + InitialState, P> EpisodeRunnerMut<'a, M, P> {
    pub fn from_initial_state(env: &'a mut M, policy: &'a mut P) -> Self {
        let s = env.initial_state();
        EpisodeRunnerMut {
            env,
            policy,
            s,
            max_time_step: None,
        }
    }
}

impl<'a, M, P> EpisodeRunnerMut<'a, M, P>
where
    M: StatesActions + InitialState + IsTerminal + GetNextStateMut + DCost,
    P: GetActionMut<M::State, M>,
{
    pub fn into_cost_iter_with_mut<'b>(
        &'b mut self,
        rng: &'b mut ThreadRng,
    ) -> CostEpisodeIteratorMut<'b, M, P>
    where
        'a: 'b,
    {
        CostEpisodeIteratorMut::new(self.env, self.policy, self.s, rng, self.max_time_step)
    }
}

impl<M, P> Eval for EpisodeRunnerMut<'_, M, P>
where
    M: StatesActions + InitialState + IsTerminal + GetNextStateMut + DCost,
    P: GetActionMut<M::State, M>,
{
    fn eval(&mut self, rng: &mut ThreadRng) -> f32 {
        let mut cumulative_cost = 0.0;
        for (_, _, _, c) in self.into_cost_iter_with_mut(rng) {
            cumulative_cost += c;
        }
        cumulative_cost
    }
}

impl<'a, M, P> EpisodeRunner<'a, M, P>
where
    M: StatesActions + IsTerminal + GetNextState + Rsas,
    P: GetAction<M::State, M>,
{
    pub fn into_reward_iter_with<'b>(
        &'b mut self,
        rng: &'b mut ThreadRng,
    ) -> RewardEpisodeIterator<'b, M, P>
    where
        'a: 'b,
    {
        RewardEpisodeIterator::new(self.env, self.policy, self.s, rng, self.max_time_step)
    }
}
