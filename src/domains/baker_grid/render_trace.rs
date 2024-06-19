use crate::baker_grid::{BakerGridMDP, BakerGridState};
use tiny_skia::{Paint, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform};
static A_BLUE_SQUARE_BYTES: &[u8; 5836] = include_bytes!("imgs/A_blue_square.drawio.png");
static B_BLUE_SQUARE_BYTES: &[u8; 5786] = include_bytes!("imgs/B_blue_square.drawio.png");
static C_BLUE_SQUARE_BYTES: &[u8; 6237] = include_bytes!("imgs/C_blue_square.drawio.png");
static D_BLUE_SQUARE_BYTES: &[u8; 5710] = include_bytes!("imgs/D_blue_square.drawio.png");
static E_BLUE_SQUARE_BYTES: &[u8; 4899] = include_bytes!("imgs/E_blue_square.drawio.png");

use super::BakerGridAction;

fn to_pixel(i: i32, offset: f32) -> f32 {
    (i as f32) * 40.0 + offset
}

impl BakerGridMDP {
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

    pub fn render_trace(
        &self,
        trace: &[(BakerGridState, Option<BakerGridAction>)],
        path: &str,
        possible_goals: Vec<(usize, usize)>,
    ) {
        println!("{} {}", self.width(), self.height());
        let mut pixmap =
            Pixmap::new((self.width() * 40) as u32, (self.height() * 40) as u32).unwrap();

        self.draw_background(&mut pixmap);
        self.draw_grids(&mut pixmap);
        self.draw_walls(&mut pixmap);

        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        for ((s, _a), (ss, _aa)) in trace.iter().zip(trace.iter().skip(1)) {
            print!("{:?} -> {:?}\n", s, ss);
            let mut pb = PathBuilder::new();
            pb.move_to(to_pixel(s.j, 20.0), to_pixel(s.i, 20.0));
            pb.line_to(to_pixel(ss.j, 20.0), to_pixel(ss.i, 20.0));
            let path = pb.finish().unwrap();
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }

        let a_s = possible_goals[0];
        let b_s = possible_goals[1];
        let c_s = possible_goals[2];

        let a = Pixmap::decode_png(A_BLUE_SQUARE_BYTES).unwrap();
        let b = Pixmap::decode_png(B_BLUE_SQUARE_BYTES).unwrap();
        let c = Pixmap::decode_png(C_BLUE_SQUARE_BYTES).unwrap();

        self.draw_icon(c_s.0 as i32, c_s.1 as i32, 0.23, 0.23, &c, &mut pixmap);
        self.draw_icon(b_s.0 as i32, b_s.1 as i32, 0.23, 0.23, &b, &mut pixmap);
        self.draw_icon(a_s.0 as i32, a_s.1 as i32, 0.23, 0.23, &a, &mut pixmap);

        pixmap.save_png(path).unwrap();
    }

    pub fn render_trace5(
        &self,
        trace: &[(BakerGridState, Option<BakerGridAction>)],
        path: &str,
        possible_goals: Vec<(usize, usize)>,
    ) {
        println!("{} {}", self.width(), self.height());
        let mut pixmap =
            Pixmap::new((self.width() * 40) as u32, (self.height() * 40) as u32).unwrap();

        self.draw_background(&mut pixmap);
        self.draw_grids(&mut pixmap);
        self.draw_walls(&mut pixmap);

        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        for ((s, _a), (ss, _aa)) in trace.iter().zip(trace.iter().skip(1)) {
            print!("{:?} -> {:?}\n", s, ss);
            let mut pb = PathBuilder::new();
            pb.move_to(to_pixel(s.j, 20.0), to_pixel(s.i, 20.0));
            pb.line_to(to_pixel(ss.j, 20.0), to_pixel(ss.i, 20.0));
            let path = pb.finish().unwrap();
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }

        let a_s = possible_goals[0];
        let b_s = possible_goals[1];
        let c_s = possible_goals[2];
        let d_s = possible_goals[3];
        let e_s = possible_goals[4];

        let a = Pixmap::decode_png(A_BLUE_SQUARE_BYTES).unwrap();
        let b = Pixmap::decode_png(B_BLUE_SQUARE_BYTES).unwrap();
        let c = Pixmap::decode_png(C_BLUE_SQUARE_BYTES).unwrap();
        let d = Pixmap::decode_png(D_BLUE_SQUARE_BYTES).unwrap();
        let e = Pixmap::decode_png(E_BLUE_SQUARE_BYTES).unwrap();

        self.draw_icon(e_s.0 as i32, e_s.1 as i32, 0.23, 0.23, &e, &mut pixmap);

        self.draw_icon(d_s.0 as i32, d_s.1 as i32, 0.23, 0.23, &d, &mut pixmap);

        self.draw_icon(c_s.0 as i32, c_s.1 as i32, 0.23, 0.23, &c, &mut pixmap);
        self.draw_icon(b_s.0 as i32, b_s.1 as i32, 0.23, 0.23, &b, &mut pixmap);
        self.draw_icon(a_s.0 as i32, a_s.1 as i32, 0.23, 0.23, &a, &mut pixmap);

        pixmap.save_png(path).unwrap();
    }
}
