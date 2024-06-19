use crate::episode_runner::EpisodeRunner;
use crate::episode_runner::EpisodeRunnerMut;
use crate::mdp_traits::*;
use crate::policy::policy_traits::GetAction;
use crate::policy::policy_traits::GetActionProbability;
use rand::prelude::*;

use super::private_traits::GetActionMutFrom;
use super::private_traits::Sealed;

pub struct RandomFromCandidatesPolicy<A> {
    candidates: Vec<A>,
}

impl<A> RandomFromCandidatesPolicy<A> {
    pub fn new(candidates: Vec<A>) -> RandomFromCandidatesPolicy<A> {
        RandomFromCandidatesPolicy {
            candidates: candidates,
        }
    }
}

impl<S, M: StatesActions> GetAction<S, M> for RandomFromCandidatesPolicy<M::Action> {
    fn get_action(&self, _s: &S, _mdp: &M, rng: &mut ThreadRng) -> Option<M::Action> {
        match self.candidates.choose(rng) {
            Some(a) => Some(*a),
            None => None,
        }
    }
}

impl<A> Sealed for RandomFromCandidatesPolicy<A> {}
impl<S, M: StatesActions> GetActionMutFrom<S, M> for RandomFromCandidatesPolicy<M::Action> {}

impl<M: StatesActions + InitialState + IsTerminal + GetNextState + DCost> IntoEval<M>
    for RandomFromCandidatesPolicy<M::Action>
{
    type Evaluator<'a> = EpisodeRunner<'a, M, RandomFromCandidatesPolicy<M::Action>> where M: 'a;

    fn into_eval<'a>(&'a self, s: M::State, mdp: &'a mut M) -> Self::Evaluator<'a> {
        EpisodeRunner::new(mdp, self, s)
    }
}

impl<M: StatesActions + InitialState + IsTerminal + GetNextStateMut + DCost> IntoEvalMut<M>
    for RandomFromCandidatesPolicy<M::Action>
{
    type Evaluator<'a> = EpisodeRunnerMut<'a, M, RandomFromCandidatesPolicy<M::Action>> where M: 'a;

    fn into_eval_mut<'a>(&'a mut self, s: M::State, mdp: &'a mut M) -> Self::Evaluator<'a> {
        EpisodeRunnerMut::new(mdp, self, s)
    }
}

impl<M: StatesActions> GetActionProbability<M::Action, M>
    for RandomFromCandidatesPolicy<M::Action>
{
    fn get_action_probability(&self, _s: &M::State, _a: &M::Action, _mdp: &M) -> f32 {
        1.0 / (self.candidates.len() as f32)
    }
}
