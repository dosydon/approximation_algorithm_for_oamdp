use crate::mdp_traits::*;
use core::slice::Iter;
use mdp_derive::*;

use std::collections::HashMap;

#[derive(
    ActionAvailability,
    ActionEnumerable,
    Cost,
    DCost,
    InitialState,
    PreferredSuccessor,
    StatesActions,
    StateEnumerable,
)]
pub struct CacheWrapper<M: StatesActions> {
    pub mdp: M,
    p_mass_cache: HashMap<(M::State, M::Action), Vec<(M::State, f32)>>,
    p_cache: HashMap<(M::State, M::Action, M::State), f32>,
    p_mass_cache_64: HashMap<(M::State, M::Action), Vec<(M::State, f64)>>,
}

impl<M: GetNextState> GetNextState for CacheWrapper<M> {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Self::State {
        self.mdp.get_next_state(s, a, rng)
    }
}

impl<M: StatesActions> CacheWrapper<M> {
    pub fn new(mdp: M) -> CacheWrapper<M> {
        CacheWrapper {
            mdp: mdp,
            p_mass_cache: HashMap::new(),
            p_cache: HashMap::new(),
            p_mass_cache_64: HashMap::new(),
        }
    }
}

impl<M: IsTerminal> IsTerminal for CacheWrapper<M> {
    fn is_terminal(&self, s: &Self::State) -> bool {
        self.mdp.is_terminal(s)
    }
}

impl<M: PMassMut<f32, Distribution = Vec<(Self::State, f32)>>> PMassMut<f32> for CacheWrapper<M> {
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if let Some(v) = self.p_mass_cache.get(&(*s, *a)) {
            v.clone()
        } else {
            let v = self.mdp.p_mass_mut(s, a);
            self.p_mass_cache.insert((*s, *a), v.clone());
            v
        }
    }
}

impl<M: PMass<f64>> PMassMut<f64> for CacheWrapper<M>
where
    M: PMass<f64, Distribution = Vec<(Self::State, f64)>>,
{
    type Distribution = Vec<(Self::State, f64)>;
    fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f64)> {
        if let Some(v) = self.p_mass_cache_64.get(&(*s, *a)) {
            v.clone()
        } else {
            let v = PMass::<f64>::p_mass(&self.mdp, s, a);
            self.p_mass_cache_64.insert((*s, *a), v.clone());
            v
        }
    }
}

impl<M> PMass<f32> for CacheWrapper<M>
where
    M: PMass<f32, Distribution = Vec<(Self::State, f32)>>,
{
    type Distribution = Vec<(Self::State, f32)>;
    fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
        if let Some(v) = self.p_mass_cache.get(&(*s, *a)) {
            v.clone()
        } else {
            let v = self.mdp.p_mass(s, a);
            v
        }
    }
}

impl<M: ExplicitTransitionMut> ExplicitTransitionMut for CacheWrapper<M>
where
    M: PMass<f32, Distribution = Vec<(Self::State, f32)>>,
{
    fn p_mut(&mut self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        if let Some(p) = self.p_cache.get(&(*st, *a, *stt)) {
            *p
        } else {
            let p = self.mdp.p_mut(st, a, stt);
            self.p_cache.insert((*st, *a, *stt), p);
            p
        }
    }
}

impl<M: ExplicitTransition> ExplicitTransition for CacheWrapper<M>
where
    M: PMass<f32, Distribution = Vec<(Self::State, f32)>>,
{
    fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
        if let Some(p) = self.p_cache.get(&(*st, *a, *stt)) {
            *p
        } else {
            let p = self.mdp.p(st, a, stt);
            p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid_world::*;
    use crate::value_iteration::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_cache_wrapper_grid_world() {
        let mdp = GridWorldMDP::default();
        let wrapped = CacheWrapper::new(GridWorldMDP::default());
        let vt = value_iteration_ssp(&wrapped);
        assert_approx_eq!(vt.get_value(&mdp.initial_state()), 9.793055);
    }
}
