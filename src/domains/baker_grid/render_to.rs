use crate::mdp_traits::RenderTo;
use tiny_skia::*;

use super::{BakerGridMDP, BakerGridState};

static GRID_SIZE: f32 = 40.0;

fn to_pixel(i: usize) -> f32 {
    (i as f32) * GRID_SIZE
}

impl BakerGridMDP {
    pub fn draw_background(&self, pixmap: &mut Pixmap) {
        let mut pb = PathBuilder::new();
        pb.move_to(0.0, 0.0);
        pb.line_to(to_pixel(self.grid2d.width), 0.0);
        pb.line_to(to_pixel(self.grid2d.width), to_pixel(self.grid2d.height));
        pb.line_to(0.0, to_pixel(self.grid2d.height));
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

        for i in 0..=self.grid2d.height {
            let mut pb = PathBuilder::new();
            pb.move_to(0.0, to_pixel(i));
            pb.line_to(to_pixel(self.grid2d.width), to_pixel(i));
            let path = pb.finish().unwrap();
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }

        for j in 0..=self.grid2d.width {
            let mut pb = PathBuilder::new();
            pb.move_to(to_pixel(j), 0.0);
            pb.line_to(to_pixel(j), to_pixel(self.grid2d.height));
            let path = pb.finish().unwrap();
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }

    pub fn draw_walls(&self, pixmap: &mut Pixmap) {
        let mut wall_color = Paint::default();
        wall_color.set_color_rgba8(170, 170, 170, 220);

        let mut stroke = Stroke::default();
        stroke.width = 2.0;
        for i in 0..self.grid2d.height {
            for j in 0..self.grid2d.width {
                if self.grid2d.is_obstacled[i][j] {
                    let mut pb = PathBuilder::new();
                    pb.move_to(to_pixel(j), to_pixel(i));
                    pb.line_to(to_pixel(j + 1), to_pixel(i));
                    pb.line_to(to_pixel(j + 1), to_pixel(i + 1));
                    pb.line_to(to_pixel(j), to_pixel(i + 1));
                    pb.line_to(to_pixel(j), to_pixel(i));
                    let path = pb.finish().unwrap();
                    pixmap.fill_path(
                        &path,
                        &wall_color,
                        FillRule::Winding,
                        Transform::identity(),
                        None,
                    );
                }
            }
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
}

impl RenderTo for BakerGridMDP {
    fn render_to(&self, s: &BakerGridState, path: &str) {
        let mut pixmap = Pixmap::new(
            (self.grid2d.width * 40) as u32,
            (self.grid2d.height * 40) as u32,
        )
        .unwrap();

        self.draw_background(&mut pixmap);
        self.draw_grids(&mut pixmap);
        self.draw_walls(&mut pixmap);
        self.draw_agent(s.i as usize, s.j as usize, &mut pixmap);

        pixmap.save_png(path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baker_grid_render_to() {
        let mdp = BakerGridMDP::new(
            5,
            8,
            vec![BakerGridState::new(4, 2), BakerGridState::new(3, 2)],
            BakerGridState::new(4, 4),
        );

        let s = BakerGridState::new(2, 2);
        mdp.render_to(&s, "baker.png");
    }
}
