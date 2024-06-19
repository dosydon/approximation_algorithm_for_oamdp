use crate::mdp_traits::*;
use crate::policy::policy_traits::*;
use rand::distributions::Uniform;
use rand::prelude::*;

pub struct EpsilonPolicy<T> {
    pub policy: T,
    epsilon: f32,
}

impl<T> EpsilonPolicy<T> {
    pub fn new(policy: T, epsilon: f32) -> EpsilonPolicy<T> {
        EpsilonPolicy {
            policy: policy,
            epsilon: epsilon,
        }
    }
}

impl<M: StatesActions + ActionEnumerable, T: GetAction<M::State, M>> GetAction<M::State, M>
    for EpsilonPolicy<T>
{
    fn get_action(&self, s: &M::State, mdp: &M, rng: &mut ThreadRng) -> Option<M::Action> {
        let e = rng.sample(Uniform::new_inclusive(0.0, 1.0));
        if e <= self.epsilon {
            mdp.enumerate_actions().cloned().choose(rng)
        } else {
            self.policy.get_action(s, mdp, rng)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid_world::GridWorldMDP;
    use crate::value_iteration::value_iteration;
    //     use crate::value_iteration::value_iteration_ssp;
    use crate::policy::tabular_policy::TabularPolicy;

    #[test]
    fn test_e_greedy_policy() {
        let mut rng = thread_rng();
        let mdp = GridWorldMDP::default();
        let vt = value_iteration(&mdp);
        let policy = EpsilonPolicy::new(TabularPolicy::from_value_table(&mdp, &vt), 0.1);
        let s = mdp.initial_state();
        for _i in 0..10 {
            println!("{:?}", policy.get_action(&s, &mdp, &mut rng));
        }
    }
}
