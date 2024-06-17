use mdp::mdp_traits::Eval;
use rand::rngs::ThreadRng;

use crate::Budget;
pub trait MCTSTrait: Eval + SetMCTSParams + RunEpisode {}

pub trait SetMCTSParams {
    fn set_c(&mut self, c: f32);
    fn set_num_rollouts(&mut self, num_rollouts: usize);
    fn set_budget(&mut self, budget: Budget);
    fn set_lookahead(&mut self, horizon: Option<usize>);
}

pub trait RunEpisode {
    fn run_episode(&mut self, rng: &mut ThreadRng) -> f32;
}
