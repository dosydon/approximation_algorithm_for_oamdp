use gnuplot::*;
use ordered_float::NotNan;

pub fn plot_belief_changes<const N: usize>(
    belief_changes: &[[NotNan<f32>; N]],
    filename: &str,
    captions: &[PlotOption<&str>],
    colors: &[PlotOption<&str>],
    line_styles: &[PlotOption<&str>],
) {
    let mut fg = Figure::new();
    let axes2d = fg
        .axes2d()
        .set_legend(Graph(1.0), Graph(1.0), &[], &[Font("Arial", 18.0)])
        .set_x_ticks(Some((Auto, 0)), &[], &[Font("Arial", 18.0)])
        .set_y_ticks(Some((Auto, 0)), &[], &[Font("Arial", 18.0)])
        .set_x_label("Time Steps", &[Font("", 18.0)])
        .set_y_label("Observer's Belief", &[Font("", 18.0)])
        .set_y_range(Fix(0.0), Fix(1.0));

    let x = (0..belief_changes.len()).collect::<Vec<_>>();
    for (((goal_id, caption), color), line_style) in
        (0..N).zip(captions).zip(colors).zip(line_styles)
    {
        let belief_change = belief_changes.iter().map(|item| item[goal_id].into_inner());
        axes2d.lines_points(
            &x,
            belief_change,
            &[
                *color,
                *line_style,
                LineWidth(2.0),
                PointSymbol('x'),
                PointSize(1.0),
                *caption,
            ],
        );
    }

    fg.set_terminal("png", filename);
    fg.show().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::domains::baker_grid::{
        BakerCOAMDPBuilder, BakerCommunicationAction, BakerJointAction,
    };

    use super::*;
    use gnuplot::PlotOption::Caption;
    use mdp::{
        baker_grid::{BakerGridAction, BakerGridState},
        mdp_traits::Build,
    };

    //     #[test]
    //     fn test_get_belief_changes() {
    //         let oamdp = rsa_example(
    //             CommunicationType::RSA,
    //             vec![
    //                 BakerCommunicationAction::Blue,
    //                 BakerCommunicationAction::None,
    //             ],
    //         );
    //         let trace = vec![
    //             (
    //                 BakerGridState::new(3, 0),
    //                 Some(BakerJointAction::new(
    //                     BakerGridAction::NorthEast,
    //                     BakerCommunicationAction::None,
    //                 )),
    //             ),
    //             (
    //                 BakerGridState::new(2, 1),
    //                 Some(BakerJointAction::new(
    //                     BakerGridAction::NorthEast,
    //                     BakerCommunicationAction::None,
    //                 )),
    //             ),
    //             (
    //                 BakerGridState::new(1, 2),
    //                 Some(BakerJointAction::new(
    //                     BakerGridAction::NorthEast,
    //                     BakerCommunicationAction::None,
    //                 )),
    //             ),
    //             (
    //                 BakerGridState::new(1, 3),
    //                 Some(BakerJointAction::new(
    //                     BakerGridAction::East,
    //                     BakerCommunicationAction::None,
    //                 )),
    //             ),
    //             (
    //                 BakerGridState::new(2, 4),
    //                 Some(BakerJointAction::new(
    //                     BakerGridAction::SouthEast,
    //                     BakerCommunicationAction::None,
    //                 )),
    //             ),
    //             (
    //                 BakerGridState::new(3, 5),
    //                 Some(BakerJointAction::new(
    //                     BakerGridAction::SouthEast,
    //                     BakerCommunicationAction::None,
    //                 )),
    //             ),
    //         ];
    //         let belief = oamdp.mdp.get_belief_changes(&trace);
    //         println!("{:?}", belief);
    //         plot_belief_changes(
    //             &belief,
    //             "belief_changes.png",
    //             &[Caption("A"), Caption("B"), Caption("C")],
    //             &[Color("blue"), Color("green"), Color("blue")],
    //             &[
    //                 LineStyle(DashType::SmallDot),
    //                 LineStyle(DashType::Dash),
    //                 LineStyle(DashType::Solid),
    //             ],
    //         );
    //     }

    #[test]
    fn test_get_belief_changes_communication() {
        let builder = BakerCOAMDPBuilder::new(1);
        let oamdp = builder.build();
        let trace = vec![
            (
                BakerGridState::new(3, 0),
                Some(BakerJointAction::new(
                    BakerGridAction::NorthEast,
                    BakerCommunicationAction::Blue,
                )),
            ),
            (
                BakerGridState::new(2, 1),
                Some(BakerJointAction::new(
                    BakerGridAction::NorthEast,
                    BakerCommunicationAction::None,
                )),
            ),
            (
                BakerGridState::new(1, 2),
                Some(BakerJointAction::new(
                    BakerGridAction::NorthEast,
                    BakerCommunicationAction::None,
                )),
            ),
            (
                BakerGridState::new(1, 3),
                Some(BakerJointAction::new(
                    BakerGridAction::East,
                    BakerCommunicationAction::None,
                )),
            ),
            (
                BakerGridState::new(2, 4),
                Some(BakerJointAction::new(
                    BakerGridAction::SouthEast,
                    BakerCommunicationAction::None,
                )),
            ),
            (
                BakerGridState::new(3, 5),
                Some(BakerJointAction::new(
                    BakerGridAction::SouthEast,
                    BakerCommunicationAction::None,
                )),
            ),
        ];
        let belief = oamdp.mdp.get_belief_changes(&trace);
        println!("{:?}", belief);
        plot_belief_changes(
            &belief,
            "belief_changes_communication.png",
            &[Caption("A"), Caption("B"), Caption("C")],
            &[Color("blue"), Color("green"), Color("blue")],
            &[
                LineStyle(DashType::SmallDot),
                LineStyle(DashType::Dash),
                LineStyle(DashType::Solid),
            ],
        );
    }
}
