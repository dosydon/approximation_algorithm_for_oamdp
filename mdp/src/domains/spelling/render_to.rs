use crate::mdp_traits::RenderTo;
use tiny_skia::*;

use super::{SpellingMDP, SpellingState};

static GRID_SIZE: f32 = 40.0;
static PAD: f32 = 3.0;
fn to_pixel(i: usize) -> f32 {
    (i as f32) * GRID_SIZE
}

impl<const N: usize> SpellingMDP<N> {
    pub fn draw_background(&self, pixmap: &mut Pixmap) {
        let mut pb = PathBuilder::new();
        pb.move_to(0.0, 0.0);
        pb.line_to(to_pixel(self.env.grid2d.width), 0.0);
        pb.line_to(
            to_pixel(self.env.grid2d.width),
            to_pixel(self.env.grid2d.height),
        );
        pb.line_to(0.0, to_pixel(self.env.grid2d.height));
        pb.line_to(0.0, 0.0);
        let path = pb.finish().unwrap();

        let mut background = Paint::default();
        background.set_color_rgba8(255, 255, 255, 255);

        pixmap.fill_path(
            &path,
            &background,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }

    pub fn draw_grids(&self, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        for i in 0..=self.env.grid2d.height {
            let mut pb = PathBuilder::new();
            pb.move_to(0.0, to_pixel(i));
            pb.line_to(to_pixel(self.env.grid2d.width), to_pixel(i));
            let path = pb.finish().unwrap();
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }

        for j in 0..=self.env.grid2d.width {
            let mut pb = PathBuilder::new();
            pb.move_to(to_pixel(j), 0.0);
            pb.line_to(to_pixel(j), to_pixel(self.env.grid2d.height));
            let path = pb.finish().unwrap();
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }

    fn draw_agent(&self, i: usize, j: usize, pixmap: &mut Pixmap) {
        let path = PathBuilder::from_circle(
            to_pixel(j) + GRID_SIZE / 2.0,
            to_pixel(i) + GRID_SIZE / 2.0,
            10.0,
        )
        .unwrap();
        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 100);

        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }

    fn draw_a(&self, i: usize, j: usize, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        let mut pb = PathBuilder::new();
        pb.move_to(to_pixel(j) + GRID_SIZE / 2.0, to_pixel(i));
        pb.line_to(to_pixel(j), to_pixel(i + 1));

        pb.move_to(to_pixel(j) + GRID_SIZE / 2.0, to_pixel(i));
        pb.line_to(to_pixel(j + 1), to_pixel(i + 1));

        pb.move_to(to_pixel(j) + GRID_SIZE / 4.0, to_pixel(i) + GRID_SIZE / 2.0);
        pb.line_to(
            to_pixel(j) + 3.0 * GRID_SIZE / 4.0,
            to_pixel(i) + GRID_SIZE / 2.0,
        );
        let path = pb.finish().unwrap();
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }

    fn draw_m(&self, i: usize, j: usize, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        let mut pb = PathBuilder::new();
        pb.move_to(to_pixel(j) + PAD, to_pixel(i + 1) - PAD);
        pb.line_to(to_pixel(j) + PAD, to_pixel(i) + PAD);
        pb.line_to(to_pixel(j) + GRID_SIZE / 2.0, to_pixel(i + 1) - PAD);
        pb.line_to(to_pixel(j + 1) - PAD, to_pixel(i) + PAD);
        pb.line_to(to_pixel(j + 1) - PAD, to_pixel(i + 1) - PAD);

        let path = pb.finish().unwrap();
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }

    fn draw_r(&self, i: usize, j: usize, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        let mut pb = PathBuilder::new();
        pb.move_to(to_pixel(j) + PAD, to_pixel(i + 1) - PAD);
        pb.line_to(to_pixel(j) + PAD, to_pixel(i) + PAD);
        pb.line_to(to_pixel(j + 1) - PAD, to_pixel(i) + PAD);
        pb.line_to(to_pixel(j + 1) - PAD, to_pixel(i) + GRID_SIZE / 2.0);
        pb.line_to(to_pixel(j) + PAD, to_pixel(i) + GRID_SIZE / 2.0);
        pb.line_to(to_pixel(j + 1) - PAD, to_pixel(i + 1) - PAD);

        let path = pb.finish().unwrap();
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }

    fn draw_s(&self, i: usize, j: usize, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;

        let mut pb = PathBuilder::new();
        pb.move_to(to_pixel(j + 1) - PAD, to_pixel(i) + PAD);
        pb.line_to(to_pixel(j) + PAD, to_pixel(i) + PAD);
        pb.line_to(to_pixel(j) + PAD, to_pixel(i) + GRID_SIZE / 2.0);
        pb.line_to(to_pixel(j + 1) - PAD, to_pixel(i) + GRID_SIZE / 2.0);
        pb.line_to(to_pixel(j + 1) - PAD, to_pixel(i + 1) - PAD);
        pb.line_to(to_pixel(j) + PAD, to_pixel(i + 1) - PAD);

        let path = pb.finish().unwrap();
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
}

impl<const N: usize> RenderTo for SpellingMDP<N> {
    fn render_to(&self, s: &SpellingState<N>, path: &str) {
        let mut pixmap = Pixmap::new(
            (self.env.grid2d.height * 40) as u32,
            (self.env.grid2d.width * 40) as u32,
        )
        .unwrap();

        self.draw_background(&mut pixmap);
        self.draw_grids(&mut pixmap);
        self.draw_agent(s.coord.i as usize, s.coord.j as usize, &mut pixmap);
        for (i, loc) in self.env.letter_locs.iter().enumerate() {
            match s.letters[i] {
                crate::domains::spelling::letter::Letter::A => {
                    self.draw_a(loc.0, loc.1, &mut pixmap);
                }
                crate::domains::spelling::letter::Letter::R => {
                    self.draw_r(loc.0, loc.1, &mut pixmap);
                }
                crate::domains::spelling::letter::Letter::M => {
                    self.draw_m(loc.0, loc.1, &mut pixmap);
                }
                crate::domains::spelling::letter::Letter::S => {
                    self.draw_s(loc.0, loc.1, &mut pixmap);
                }
            }
        }

        pixmap.save_png(path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::common::coordinate2::Coordinate2;
    use crate::domains::spelling::letter::Letter::*;

    use super::*;

    #[test]
    fn test_spelling_render_to() {
        let mdp = SpellingMDP::new(
            5,
            5,
            vec![],
            [A, R, M, S],
            [(0, 0), (0, 4), (4, 0), (4, 4)],
            SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]),
        );

        let s = SpellingState::new(Coordinate2::new(0, 0), [A, A, A, A]);
        mdp.render_to(&s, "spelling.png");
    }
}
