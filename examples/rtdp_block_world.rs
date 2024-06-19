use mdp::blocks_world::{Block, BlocksWorldPartialMDP, LetterManager, Location};
use mdp::blocks_world::{BlocksWorldMDPN, Location::*};
use mdp::episode_runner::EpisodeRunnerMut;
use mdp::heuristic::HminHeuristic;
use mdp::mdp_traits::{BuildFrom, InitialState};
use rand::thread_rng;
use rtdp::rtdp::RTDP;

#[allow(non_snake_case)]
fn main() {
<<<<<<< HEAD
    let _A = Block::new(0);
=======
>>>>>>> e8d7b112ef8d27f7088f2fd8450dab5d02614616
    let M = Block::new(1);
    let S = Block::new(2);
    let R = Block::new(3);
    let OnS = Location::On(S);
    let OnM = Location::On(M);
    let OnR = Location::On(R);
    let lm = LetterManager::new(['A', 'M', 'S', 'R']);
    let partial_mdp = BlocksWorldPartialMDP::new([OnR, OnTable, OnM, OnS], 0.1, lm.letters);
    let goal = lm.letters_to_goal(['R', 'A', 'M', 'S']);
    println!("{:?}", goal);

    let mut rng = thread_rng();
    let mut mdp = partial_mdp.build_from(&goal);
    let mut lrtdp = RTDP::new(HminHeuristic::new());
    lrtdp.lrtdp(&mut mdp, 0, &mut rng, 1e-3);

    let s = mdp.initial_state();

    unsafe {
        let mdp_p = &mut mdp as *mut BlocksWorldMDPN<4>;
        let mut runner = EpisodeRunnerMut::new(&mut mdp, &mut lrtdp, s);
        for (s, a, _, c) in runner.into_cost_iter_with_mut(&mut rng) {
            println!("{:?} {:?} {}", s, a, c);
            (*mdp_p).display(&s);
        }
    }
}
