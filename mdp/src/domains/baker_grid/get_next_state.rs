use rand::rngs::ThreadRng;
use rand::Rng;

use crate::mdp_traits::{GetNextState, GetNextStateMutFromImmut, IsTerminal};

use super::BakerGridMDP;

impl GetNextState for BakerGridMDP {
    fn get_next_state(
        &self,
        s: &Self::State,
        a: &Self::Action,
        rng: &mut ThreadRng,
    ) -> Self::State {
        let dj = a.get_dj();
        let di = a.get_di();

        if self.is_terminal(s)
            || !self.grid2d.within_bound(s.i + di, s.j + dj)
            || self.grid2d.is_obstacled[(s.i + di) as usize][(s.j + dj) as usize]
        {
            return *s;
        } else {
            let r = rng.gen_range(0.0, 1.0);
            if r < self.prob_veering {
                let r = rng.gen_range(0.0, 1.0);
                if r < 0.5 {
                    return self.grid2d.veer_left(s, a);
                } else {
                    return self.grid2d.veer_right(s, a);
                }
            } else {
                return self.grid2d.success(s, a);
            }
        }
    }
}

impl GetNextStateMutFromImmut for BakerGridMDP {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rand::thread_rng;

    use crate::baker_grid::BakerGridAction::*;
    use crate::mdp_traits::{ExplicitTransition, PMass};
    use crate::policy::policy_traits::GetAction;
    use crate::policy::random_policy::RandomPolicy;
    use crate::{
        baker_grid::{BakerGridPartialMDP, BakerGridState},
        mdp_traits::BuildFrom,
    };

    use super::*;
    #[test]
    fn test_baker_grid_get_next_state() {
        let width = 17;
        let height = 9;
        let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

        let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles).set_prob_veering(0.5);
        let mdp: BakerGridMDP = partial_mdp.build_from(&BakerGridState::new(0, 16));

        let mut tally = HashMap::new();
        for _ in 0..1000 {
            let s = mdp.get_next_state(&BakerGridState::new(5, 6), &East, &mut rand::thread_rng());
            *tally.entry(s).or_insert(0) += 1;
        }

        assert!(tally[&BakerGridState::new(5, 7)] < 550);
        assert!(tally[&BakerGridState::new(5, 7)] > 450);
        assert!(tally[&BakerGridState::new(4, 7)] < 280);
        assert!(tally[&BakerGridState::new(4, 7)] > 220);
        assert!(tally[&BakerGridState::new(6, 7)] < 280);
        assert!(tally[&BakerGridState::new(6, 7)] > 220);
    }

    #[test]
    fn test_baker_grid_get_next_state_sanity_check() {
        let width = 17;
        let height = 9;
        let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

        let partial_mdp = BakerGridPartialMDP::new(height, width, obstacles).set_prob_veering(0.5);
        let mdp: BakerGridMDP = partial_mdp.build_from(&BakerGridState::new(0, 16));
        let random_policy = RandomPolicy {};
        let mut rng = thread_rng();

        for _ in 0..1000 {
            let s = BakerGridState::new(1, 8);
            let a = random_policy.get_action(&s, &mdp, &mut rng);
            let ss = mdp.get_next_state(&BakerGridState::new(1, 8), &a.unwrap(), &mut rng);
            assert!(
                mdp.p(&s, &a.unwrap(), &ss) > 0.0,
                "s: {:?}, a: {:?}, ss: {:?} p_mass: {:?}",
                s,
                a,
                ss,
                mdp.p_mass(&s, &a.unwrap())
            );
        }
    }
}
