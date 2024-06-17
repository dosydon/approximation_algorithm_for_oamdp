use aostar::aostar::AOStar;
use mdp::blocks_world::Block;
use mdp::blocks_world::BlocksWorldMDPN;
use mdp::blocks_world::Location::*;

use mdp::heuristic::ZeroHeuristic;

#[test]
fn test_laostar_blocks_world() {
    let mdp = BlocksWorldMDPN::new(
        [OnTable, OnTable, On(Block::new(1)), OnTable],
        [
            On(Block::new(3)),
            On(Block::new(0)),
            OnTable,
            On(Block::new(2)),
        ],
        0.0,
        ['A', 'M', 'S', 'R'],
    );
    let zero_heuristic = ZeroHeuristic {};
    let err = 1e-1;
    let mut aostar = AOStar::new(mdp, zero_heuristic);
    aostar.ilaostar(err);
    assert_eq!(7.0, aostar.root_f());
}

#[test]
fn test_laostar_blocks_world6() {
    let mdp = BlocksWorldMDPN::new(
        [
            OnTable,
            On(Block::new(0)),
            On(Block::new(1)),
            On(Block::new(2)),
            On(Block::new(3)),
            On(Block::new(4)),
        ],
        [OnTable, OnTable, OnTable, OnTable, OnTable, OnTable],
        0.0,
        ['A', 'B', 'C', 'M', 'S', 'R'],
    );
    let zero_heuristic = ZeroHeuristic {};
    let err = 1e-1;
    let mut aostar = AOStar::new(mdp, zero_heuristic);
    aostar.ilaostar(err);

    //     let mut rng = thread_rng();
    //     let policy = aostar.to_policy();
    //     let runner = EpisodeRunner::new();
    //     let result = runner.run_episode(&aostar.mdp,&policy, &mut rng);
    //     for (s, _a) in result.0 {
    //         aostar.mdp.display(&s);
    //     }
}
