use mdp::mdp_traits::Eval;
use rand::rngs::ThreadRng;

pub trait RTDPNumStates {
    fn num_states(&self) -> usize;

    fn num_domain_states(&self) -> usize;
}

pub trait RTDPRootValue {
    fn root_value(&mut self) -> f32;
}

pub trait RunEpisode {
    fn run_episode(&mut self, rng: &mut ThreadRng);
}

pub trait RTDPTrait {
    fn rtdp(&mut self, num_trials: usize, rng: &mut ThreadRng);

    fn lrtdp(&mut self, num_trials: usize, rng: &mut ThreadRng);
}

pub trait RTDPTraitAll: RTDPNumStates + RTDPRootValue + RunEpisode + RTDPTrait + Eval {}
