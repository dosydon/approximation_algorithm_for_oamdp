use mdp::baker_grid::{BakerGridMDP, BakerGridState};

use tiny_skia::{Paint, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform};

use crate::{domains::baker_grid::Shape, oamdp::oamdp::OAMDP};

use super::{
    communication_model::BakerCommunicationModel, BakerCommunicationAction, BakerJointAction,
};

static A_BLUE_CIRCLE_BYTES: &[u8; 7239] = include_bytes!("imgs/A_blue_circle.drawio.png");
static B_BLUE_CIRCLE_BYTES: &[u8; 7299] = include_bytes!("imgs/B_blue_circle.drawio.png");
static C_BLUE_CIRCLE_BYTES: &[u8; 7789] = include_bytes!("imgs/C_blue_circle.drawio.png");
static D_BLUE_CIRCLE_BYTES: &[u8; 7270] = include_bytes!("imgs/D_blue_circle.drawio.png");
static E_BLUE_CIRCLE_BYTES: &[u8; 6438] = include_bytes!("imgs/E_blue_circle.drawio.png");

static A_GREEN_CIRCLE_BYTES: &[u8; 6393] = include_bytes!("imgs/A_green_circle.drawio.png");
static B_GREEN_CIRCLE_BYTES: &[u8; 6412] = include_bytes!("imgs/B_green_circle.drawio.png");
static C_GREEN_CIRCLE_BYTES: &[u8; 6848] = include_bytes!("imgs/C_green_circle.drawio.png");
static D_GREEN_CIRCLE_BYTES: &[u8; 6429] = include_bytes!("imgs/D_green_circle.drawio.png");
static E_GREEN_CIRCLE_BYTES: &[u8; 5652] = include_bytes!("imgs/E_green_circle.drawio.png");

static A_BLUE_SQUARE_BYTES: &[u8; 5836] = include_bytes!("imgs/A_blue_square.drawio.png");
static B_BLUE_SQUARE_BYTES: &[u8; 5786] = include_bytes!("imgs/B_blue_square.drawio.png");
static C_BLUE_SQUARE_BYTES: &[u8; 6237] = include_bytes!("imgs/C_blue_square.drawio.png");
static D_BLUE_SQUARE_BYTES: &[u8; 5710] = include_bytes!("imgs/D_blue_square.drawio.png");
static E_BLUE_SQUARE_BYTES: &[u8; 4899] = include_bytes!("imgs/E_blue_square.drawio.png");

static A_GREEN_SQUARE_BYTES: &[u8; 5835] = include_bytes!("imgs/A_green_square.drawio.png");
static B_GREEN_SQUARE_BYTES: &[u8; 5666] = include_bytes!("imgs/B_green_square.drawio.png");
static C_GREEN_SQUARE_BYTES: &[u8; 6223] = include_bytes!("imgs/C_green_square.drawio.png");
static D_GREEN_SQUARE_BYTES: &[u8; 5693] = include_bytes!("imgs/D_green_square.drawio.png");
static E_GREEN_SQUARE_BYTES: &[u8; 4819] = include_bytes!("imgs/E_green_square.drawio.png");

static ANNOUCE_BLUE_BYTES: &[u8; 9552] = include_bytes!("imgs/announce_blue.drawio.png");
static ANNOUCE_GREEN_BYTES: &[u8; 9583] = include_bytes!("imgs/announce_green.drawio.png");
static ANNOUCE_SQUARE_BYTES: &[u8; 6202] = include_bytes!("imgs/announce_square.drawio.png");
static ANNOUCE_CIRCLE_BYTES: &[u8; 11027] = include_bytes!("imgs/announce_circle.drawio.png");

static SPEAKER_BYTES: &[u8; 7640] = include_bytes!("imgs/speaker-filled-audio-tool.png");

fn to_pixel(i: i32, offset: f32) -> f32 {
    (i as f32) * 40.0 + offset
}

impl<const N: usize> OAMDP<BakerCommunicationModel<N>, BakerGridMDP, BakerJointAction, N> {
    fn draw_icon(&self, i: i32, j: i32, sx: f32, sy: f32, icon: &Pixmap, pixmap: &mut Pixmap) {
        let mut paint = PixmapPaint::default();
        paint.opacity = 0.8;

        pixmap.draw_pixmap(
            0,
            0,
            icon.as_ref(),
            &paint,
            Transform::from_scale(sx, sy).post_translate(to_pixel(j, 0.0), to_pixel(i, 0.0)),
            None,
        );
    }

    pub fn render_trace(&self, trace: &[(BakerGridState, Option<BakerJointAction>)], path: &str) {
        let mdp = &self.mdp;
        println!("{} {}", mdp.width(), mdp.height());
        let mut pixmap =
            Pixmap::new((mdp.width() * 40) as u32, (mdp.height() * 40) as u32).unwrap();

        mdp.draw_background(&mut pixmap);
        mdp.draw_grids(&mut pixmap);
        mdp.draw_walls(&mut pixmap);

        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        for ((s, a), (ss, _aa)) in trace.iter().zip(trace.iter().skip(1)) {
            print!("{:?} -> {:?}\n", s, ss);
            let mut pb = PathBuilder::new();
            pb.move_to(to_pixel(s.j, 20.0), to_pixel(s.i, 20.0));
            pb.line_to(to_pixel(ss.j, 20.0), to_pixel(ss.i, 20.0));
            let path = pb.finish().unwrap();
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

            if let Some(a) = a {
                match a.communication_action {
                    BakerCommunicationAction::None => {}
                    _ => {
                        let speaker = Pixmap::decode_png(SPEAKER_BYTES).unwrap();
                        println!("{} {}", ss.i, ss.j);
                        self.draw_icon(ss.i, ss.j, 0.08, 0.08, &speaker, &mut pixmap);
                    }
                }
            }
        }

        let a_s = self.assumed_model.communication_model.possible_goals[0];
        let b_s = self.assumed_model.communication_model.possible_goals[1];
        let c_s = self.assumed_model.communication_model.possible_goals[2];

        let a = match self.assumed_model.communication_model.shapes[0] {
            Shape::BlueCircle => Pixmap::decode_png(A_BLUE_CIRCLE_BYTES).unwrap(),
            Shape::BlueSquare => Pixmap::decode_png(A_BLUE_SQUARE_BYTES).unwrap(),
            Shape::GreenCircle => Pixmap::decode_png(A_GREEN_CIRCLE_BYTES).unwrap(),
            Shape::GreenSquare => Pixmap::decode_png(A_GREEN_SQUARE_BYTES).unwrap(),
        };
        let b = match self.assumed_model.communication_model.shapes[1] {
            Shape::BlueCircle => Pixmap::decode_png(B_BLUE_CIRCLE_BYTES).unwrap(),
            Shape::BlueSquare => Pixmap::decode_png(B_BLUE_SQUARE_BYTES).unwrap(),
            Shape::GreenCircle => Pixmap::decode_png(B_GREEN_CIRCLE_BYTES).unwrap(),
            Shape::GreenSquare => Pixmap::decode_png(B_GREEN_SQUARE_BYTES).unwrap(),
        };
        let c = match self.assumed_model.communication_model.shapes[2] {
            Shape::BlueCircle => Pixmap::decode_png(C_BLUE_CIRCLE_BYTES).unwrap(),
            Shape::BlueSquare => Pixmap::decode_png(C_BLUE_SQUARE_BYTES).unwrap(),
            Shape::GreenCircle => Pixmap::decode_png(C_GREEN_CIRCLE_BYTES).unwrap(),
            Shape::GreenSquare => Pixmap::decode_png(C_GREEN_SQUARE_BYTES).unwrap(),
        };

        self.draw_icon(c_s.i, c_s.j, 0.23, 0.23, &c, &mut pixmap);
        self.draw_icon(b_s.i, b_s.j, 0.23, 0.23, &b, &mut pixmap);
        self.draw_icon(a_s.i, a_s.j, 0.28, 0.28, &a, &mut pixmap);

        pixmap.save_png(path).unwrap();
    }

    pub fn render_trace5(&self, trace: &[(BakerGridState, Option<BakerJointAction>)], path: &str) {
        let mdp = &self.mdp;
        println!("{} {}", mdp.width(), mdp.height());
        let mut pixmap =
            Pixmap::new((mdp.width() * 40) as u32, (mdp.height() * 40) as u32).unwrap();

        mdp.draw_background(&mut pixmap);
        mdp.draw_grids(&mut pixmap);
        mdp.draw_walls(&mut pixmap);

        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        for ((s, a), (ss, _aa)) in trace.iter().zip(trace.iter().skip(1)) {
            print!("{:?} -> {:?}\n", s, ss);
            let mut pb = PathBuilder::new();
            pb.move_to(to_pixel(s.j, 20.0), to_pixel(s.i, 20.0));
            pb.line_to(to_pixel(ss.j, 20.0), to_pixel(ss.i, 20.0));
            let path = pb.finish().unwrap();
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

            if let Some(a) = a {
                match a.communication_action {
                    BakerCommunicationAction::None => {}
                    BakerCommunicationAction::Blue => {
                        let speaker = Pixmap::decode_png(ANNOUCE_BLUE_BYTES).unwrap();
                        println!("{} {}", ss.i, ss.j);
                        self.draw_icon(ss.i, ss.j, 0.15, 0.15, &speaker, &mut pixmap);
                    }
                    BakerCommunicationAction::Green => {
                        let speaker = Pixmap::decode_png(ANNOUCE_GREEN_BYTES).unwrap();
                        println!("{} {}", ss.i, ss.j);
                        self.draw_icon(ss.i, ss.j, 0.15, 0.15, &speaker, &mut pixmap);
                    }
                    BakerCommunicationAction::Square => {
                        let speaker = Pixmap::decode_png(ANNOUCE_SQUARE_BYTES).unwrap();
                        println!("{} {}", ss.i, ss.j);
                        self.draw_icon(ss.i, ss.j, 0.15, 0.15, &speaker, &mut pixmap);
                    }
                    BakerCommunicationAction::Circle => {
                        let speaker = Pixmap::decode_png(ANNOUCE_CIRCLE_BYTES).unwrap();
                        println!("{} {}", ss.i, ss.j);
                        self.draw_icon(ss.i, ss.j, 0.15, 0.15, &speaker, &mut pixmap);
                    }
                }
            }
        }

        let a_s = self.assumed_model.communication_model.possible_goals[0];
        let b_s = self.assumed_model.communication_model.possible_goals[1];
        let c_s = self.assumed_model.communication_model.possible_goals[2];
        let d_s = self.assumed_model.communication_model.possible_goals[3];
        let e_s = self.assumed_model.communication_model.possible_goals[4];

        let a = match self.assumed_model.communication_model.shapes[0] {
            Shape::BlueCircle => Pixmap::decode_png(A_BLUE_CIRCLE_BYTES).unwrap(),
            Shape::BlueSquare => Pixmap::decode_png(A_BLUE_SQUARE_BYTES).unwrap(),
            Shape::GreenCircle => Pixmap::decode_png(A_GREEN_CIRCLE_BYTES).unwrap(),
            Shape::GreenSquare => Pixmap::decode_png(A_GREEN_SQUARE_BYTES).unwrap(),
        };
        let b = match self.assumed_model.communication_model.shapes[1] {
            Shape::BlueCircle => Pixmap::decode_png(B_BLUE_CIRCLE_BYTES).unwrap(),
            Shape::BlueSquare => Pixmap::decode_png(B_BLUE_SQUARE_BYTES).unwrap(),
            Shape::GreenCircle => Pixmap::decode_png(B_GREEN_CIRCLE_BYTES).unwrap(),
            Shape::GreenSquare => Pixmap::decode_png(B_GREEN_SQUARE_BYTES).unwrap(),
        };
        let c = match self.assumed_model.communication_model.shapes[2] {
            Shape::BlueCircle => Pixmap::decode_png(C_BLUE_CIRCLE_BYTES).unwrap(),
            Shape::BlueSquare => Pixmap::decode_png(C_BLUE_SQUARE_BYTES).unwrap(),
            Shape::GreenCircle => Pixmap::decode_png(C_GREEN_CIRCLE_BYTES).unwrap(),
            Shape::GreenSquare => Pixmap::decode_png(C_GREEN_SQUARE_BYTES).unwrap(),
        };
        let d = match self.assumed_model.communication_model.shapes[3] {
            Shape::BlueCircle => Pixmap::decode_png(D_BLUE_CIRCLE_BYTES).unwrap(),
            Shape::BlueSquare => Pixmap::decode_png(D_BLUE_SQUARE_BYTES).unwrap(),
            Shape::GreenCircle => Pixmap::decode_png(D_GREEN_CIRCLE_BYTES).unwrap(),
            Shape::GreenSquare => Pixmap::decode_png(D_GREEN_SQUARE_BYTES).unwrap(),
        };
        let e = match self.assumed_model.communication_model.shapes[4] {
            Shape::BlueCircle => Pixmap::decode_png(E_BLUE_CIRCLE_BYTES).unwrap(),
            Shape::BlueSquare => Pixmap::decode_png(E_BLUE_SQUARE_BYTES).unwrap(),
            Shape::GreenCircle => Pixmap::decode_png(E_GREEN_CIRCLE_BYTES).unwrap(),
            Shape::GreenSquare => Pixmap::decode_png(E_GREEN_SQUARE_BYTES).unwrap(),
        };

        self.draw_icon(e_s.i, e_s.j, 0.23, 0.23, &e, &mut pixmap);
        self.draw_icon(d_s.i, d_s.j, 0.23, 0.23, &d, &mut pixmap);
        self.draw_icon(c_s.i, c_s.j, 0.23, 0.23, &c, &mut pixmap);
        self.draw_icon(b_s.i, b_s.j, 0.23, 0.23, &b, &mut pixmap);
        self.draw_icon(a_s.i, a_s.j, 0.28, 0.28, &a, &mut pixmap);

        pixmap.save_png(path).unwrap();
    }
}
