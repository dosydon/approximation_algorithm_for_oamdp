use crate::episode_runner::EpisodeRunner;
use crate::episode_runner::EpisodeRunnerMut;
use crate::mdp_traits::*;
use crate::policy::policy_traits::GetAction;
use crate::policy::policy_traits::GetActionProbability;
use rand::prelude::*;

use super::private_traits::GetActionMutFrom;
use super::private_traits::Sealed;

pub struct RandomPolicy {}

impl<S, M: ActionEnumerable> GetAction<S, M> for RandomPolicy {
    fn get_action(&self, _s: &S, mdp: &M, rng: &mut ThreadRng) -> Option<M::Action> {
        match mdp.enumerate_actions().choose(rng) {
            Some(a) => Some(*a),
            None => None,
        }
    }
}

impl<S, M: ActionEnumerable> GetActionMutFrom<S, M> for RandomPolicy {}
impl Sealed for RandomPolicy {}

impl<M: DCost + IsTerminal + StatesActions + InitialState + GetNextState + ActionEnumerable>
    IntoEval<M> for RandomPolicy
{
    type Evaluator<'a> = EpisodeRunner<'a, M, RandomPolicy> where M: 'a;

    fn into_eval<'a>(
        &'a self,
        s: <M as StatesActions>::State,
        mdp: &'a mut M,
    ) -> Self::Evaluator<'a> {
        EpisodeRunner::new(mdp, &self, s)
    }
}

impl<M: DCost + IsTerminal + StatesActions + InitialState + GetNextStateMut + ActionEnumerable>
    IntoEvalMut<M> for RandomPolicy
{
    type Evaluator<'a> = EpisodeRunnerMut<'a, M, RandomPolicy> where M: 'a;

    fn into_eval_mut<'a>(
        &'a mut self,
        s: <M as StatesActions>::State,
        mdp: &'a mut M,
    ) -> Self::Evaluator<'a> {
        EpisodeRunnerMut::new(mdp, self, s)
    }
}

impl<M: ActionEnumerable> GetActionProbability<M::Action, M> for RandomPolicy {
    fn get_action_probability(&self, _s: &M::State, _a: &M::Action, mdp: &M) -> f32 {
        1.0 / (mdp.enumerate_actions().count() as f32)
    }
}
