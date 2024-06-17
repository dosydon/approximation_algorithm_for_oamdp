use mdp::mdp_traits::*;

use mdp::policy::policy_traits::GetActionMut;

use rand::rngs::ThreadRng;

use crate::traits::RunEpisode;

use super::MCTS;

impl<M, P> RunEpisode for MCTS<M, P>
where
    M: StatesActions
        + IsTerminal
        + ActionAvailability
        + GetNextStateMut
        + Cost
        + DCost
        + InitialState
        + ActionEnumerable
        + DisplayState<M::State>,
    P: IntoEvalMut<M> + GetActionMut<M::State, M>,
{
    fn run_episode(&mut self, rng: &mut ThreadRng) -> f32 {
        let mut sum = 0.0;
        unsafe {
            let self_p = self as *const Self;
            for (s, a, _ss, c) in self.into_iter_with(rng) {
                println!("{:?}", a);
                (*self_p).display(&s);
                sum += c;
            }
        }
        sum
    }
}
