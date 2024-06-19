use crate::{baker_grid::*, common::coordinate2::Coordinate2};

pub fn baker_factory_partial_mdp(instance_name: &str) -> BakerGridPartialMDP {
    match instance_name {
        "Large1" => {
            let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

            BakerGridPartialMDP::new(9, 17, obstacles)
        }
        "Large2" => {
            let obstacles = vec![(5, 8), (6, 8), (8, 8)];

            BakerGridPartialMDP::new(9, 17, obstacles)
        }
        "Large3" => {
            let obstacles = vec![(5, 8), (6, 8), (7, 8), (8, 8)];

            BakerGridPartialMDP::new(9, 17, obstacles)
        }
        "Tiny" => {
            let obstacles = vec![];

            BakerGridPartialMDP::new(5, 5, obstacles)
        }
        "Tiny2" => {
            let obstacles = vec![(1, 1)];

            BakerGridPartialMDP::new(3, 4, obstacles)
        }
        "Small1" => {
            let obstacles = vec![(4, 4), (3, 4)];

            BakerGridPartialMDP::new(5, 10, obstacles)
        }
        "Small2" => {
            let obstacles = vec![(4, 2), (3, 2)];
            BakerGridPartialMDP::new(5, 10, obstacles)
        }
        "Small3" => {
            let obstacles = vec![(6, 1), (5, 1)];
            BakerGridPartialMDP::new(7, 7, obstacles)
        }
        "TShape" => {
            let obstacles = vec![
                (4, 1),
                (3, 1),
                (2, 1),
                (1, 1),
                (4, 3),
                (3, 3),
                (2, 3),
                (1, 3),
            ];
            BakerGridPartialMDP::new(5, 5, obstacles)
        }
        &_ => unimplemented!(),
    }
}

pub fn baker_factory(instance_name: &str) -> (BakerGridPartialMDP, Vec<BakerGridState>) {
    let partial_mdp = baker_factory_partial_mdp(instance_name);
    match instance_name {
        "Large1" => {
            let possible_goals = vec![
                BakerGridState::new(0, 16),
                BakerGridState::new(8, 16),
                BakerGridState::new(0, 9),
            ];
            (partial_mdp, possible_goals)
        }
        "Large2" => {
            let possible_goals = vec![
                BakerGridState::new(0, 16),
                BakerGridState::new(8, 16),
                BakerGridState::new(0, 9),
            ];
            (partial_mdp, possible_goals)
        }
        "Large3" => {
            let possible_goals = vec![
                BakerGridState::new(0, 16),
                BakerGridState::new(8, 16),
                BakerGridState::new(0, 8),
            ];
            (partial_mdp, possible_goals)
        }
        "Tiny" => {
            let possible_goals = vec![BakerGridState::new(0, 0), BakerGridState::new(0, 4)];
            (
                partial_mdp
                    .set_initial_state(Coordinate2::new(4, 2))
                    .set_prob_veering(0.3),
                possible_goals,
            )
        }
        "Tiny2" => {
            let possible_goals = vec![BakerGridState::new(0, 1), BakerGridState::new(0, 2)];
            (
                partial_mdp
                    .set_initial_state(Coordinate2::new(2, 0))
                    .set_prob_veering(0.3),
                possible_goals,
            )
        }
        "Small1" => {
            let possible_goals = vec![
                BakerGridState::new(0, 9),
                BakerGridState::new(4, 9),
                BakerGridState::new(0, 4),
            ];
            (partial_mdp, possible_goals)
        }
        "Small2" => {
            let possible_goals = vec![BakerGridState::new(4, 4), BakerGridState::new(0, 0)];

            (partial_mdp, possible_goals)
        }
        "Small3" => {
            let possible_goals = vec![BakerGridState::new(6, 6), BakerGridState::new(0, 0)];

            (partial_mdp, possible_goals)
        }
        "TShape" => {
            let possible_goals = vec![BakerGridState::new(0, 4), BakerGridState::new(0, 0)];

            (partial_mdp, possible_goals)
        }
        &_ => unimplemented!(),
    }
}
