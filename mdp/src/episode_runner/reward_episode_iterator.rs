use crate::{
    mdp_traits::{GetNextState, InitialState, IsTerminal, Rsas, StatesActions},
    policy::policy_traits::GetAction,
};

pub struct RewardEpisodeIterator<'a, M: StatesActions, P> {
    env: &'a M,
    policy: &'a P,
    s: M::State,
    rng: &'a mut rand::rngs::ThreadRng,
    t: usize,
    max_time_step: Option<usize>,
}

impl<'a, M: StatesActions, P> RewardEpisodeIterator<'a, M, P> {
    pub fn new(
        env: &'a M,
        policy: &'a P,
        s: M::State,
        rng: &'a mut rand::rngs::ThreadRng,
        max_time_step: Option<usize>,
    ) -> Self {
        RewardEpisodeIterator {
            env,
            policy,
            s,
            rng,
            t: 0,
            max_time_step,
        }
    }
}

impl<'a, M: StatesActions + InitialState, P> RewardEpisodeIterator<'a, M, P> {
    pub fn from_initial_state(
        env: &'a M,
        policy: &'a P,
        rng: &'a mut rand::rngs::ThreadRng,
    ) -> Self {
        RewardEpisodeIterator {
            env,
            policy,
            s: env.initial_state(),
            rng,
            t: 0,
            max_time_step: None,
        }
    }
}

impl<'a, M: StatesActions + IsTerminal, P> RewardEpisodeIterator<'a, M, P> {
    pub(crate) fn terminal_condition(&self, s: &M::State, mdp: &M, time_step: usize) -> bool {
        if let Some(max_time_step) = self.max_time_step {
            mdp.is_terminal(s) || time_step > max_time_step
        } else {
            mdp.is_terminal(s)
        }
    }
}

impl<M, P> Iterator for RewardEpisodeIterator<'_, M, P>
where
    M: StatesActions + GetNextState + IsTerminal + Rsas,
    P: GetAction<M::State, M>,
{
    type Item = (M::State, M::Action, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.terminal_condition(&self.s, &self.env, self.t) {
            None
        } else {
            let a = self
                .policy
                .get_action(&self.s, &self.env, &mut self.rng)
                .expect("no action");
            let s_prime = self.env.get_next_state(&self.s, &a, &mut self.rng);
            let r = self.env.rsas(&self.s, &a, &s_prime);
            self.s = s_prime;
            self.t += 1;
            Some((self.s.clone(), a, r))
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
    fn test_reward_episode_iterator() {
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

        for (s, a, r) in RewardEpisodeIterator::new(&mdp, &policy, s, &mut rng, None) {
            println!("{:?} {:?} {:?}", s, a, r);
        }
    }
}
