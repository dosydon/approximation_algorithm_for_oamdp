use tiny_skia::*;

use crate::domains::recycle::{Location, RecycleCommunicationAction};

use super::{mdp::RecycleMDP, RecycleJointAction, RecycleState};

static COMPOSTABLE_BYTES: &[u8; 27574] = include_bytes!("pngs/compostable.png");
static TRASH_BYTES: &[u8; 8886] = include_bytes!("pngs/delete.png");
static RECYCLING_BYTES: &[u8; 18757] = include_bytes!("pngs/recycling.png");
static WATER_BYTES: &[u8; 15291] = include_bytes!("pngs/water.png");
static WASTE_BYTES: &[u8; 21571] = include_bytes!("pngs/waste.png");
static DIAPER_BYTES: &[u8; 19009] = include_bytes!("pngs/diaper.png");

impl<const K: usize> RecycleMDP<K> {
    fn draw_icon(&self, x: f32, y: f32, icon: &Pixmap, pixmap: &mut Pixmap) {
        let mut paint = PixmapPaint::default();
        paint.opacity = 0.8;

        pixmap.draw_pixmap(
            0,
            0,
            icon.as_ref(),
            &paint,
            Transform::from_scale(0.1, 0.1).post_translate(x, y),
            None,
        );
    }
}

fn to_x(i: usize, j: usize) -> f32 {
    200.0 * i as f32 + 60.0 * j as f32
}

fn to_x_y(object_id: usize, location: Location) -> (f32, f32) {
    let y = if object_id < 3 { 150.0 } else { 250.0 };
    match location {
        Location::Compost => (to_x(0, object_id % 3), y),
        Location::Trash => (to_x(1, object_id % 3), y),
        Location::Recycle => (to_x(2, object_id % 3), y),
        _ => panic!("invalid location"),
    }
}

impl<const K: usize> RecycleMDP<K> {
    pub fn render_to(&self, s: &RecycleState<K>, a: &RecycleJointAction, file_path: &str) {
        println!("rendering to {}", file_path);
        let mut pixmap = Pixmap::new(600, 400).unwrap();
        let mut stroke = Stroke::default();
        stroke.width = 2.0;
        let mut paint = Paint::default();
        paint.set_color_rgba8(255, 255, 255, 255); // Set the desired background color (e.g., light blue)

        // Create a background rectangle that covers the entire canvas
        let background_rect = Rect::from_xywh(0.0, 0.0, 600.0, 400.0).unwrap();
        // Draw the background rectangle onto the canvas
        pixmap.fill_rect(
            background_rect,
            &paint,
            tiny_skia::Transform::identity(),
            None,
        );

        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 200);
        let mut pb = PathBuilder::new();
        pb.move_to(200.0, 100.0);
        pb.line_to(200.0, 400.0);
        let path = pb.finish().unwrap();
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

        paint.set_color_rgba8(0, 0, 0, 200);
        let mut pb = PathBuilder::new();
        pb.move_to(400.0, 100.0);
        pb.line_to(400.0, 400.0);
        let path = pb.finish().unwrap();
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

        let compost = Pixmap::decode_png(COMPOSTABLE_BYTES).unwrap();
        self.draw_icon(80.0, 350.0, &compost, &mut pixmap);
        let trash = Pixmap::decode_png(TRASH_BYTES).unwrap();
        self.draw_icon(280.0, 350.0, &trash, &mut pixmap);
        let recycle = Pixmap::decode_png(RECYCLING_BYTES).unwrap();
        self.draw_icon(480.0, 350.0, &recycle, &mut pixmap);

        match a.communication_action {
            RecycleCommunicationAction::Announce(Location::Compost) => {
                self.draw_icon(100.0, 10.0, &compost, &mut pixmap);
            }
            RecycleCommunicationAction::Announce(Location::Trash) => {
                self.draw_icon(100.0, 10.0, &trash, &mut pixmap);
            }
            RecycleCommunicationAction::Announce(Location::Recycle) => {
                self.draw_icon(100.0, 10.0, &recycle, &mut pixmap);
            }
            _ => (),
        }

        let waste = Pixmap::decode_png(WASTE_BYTES).unwrap();
        let diaper = Pixmap::decode_png(DIAPER_BYTES).unwrap();
        let water = Pixmap::decode_png(WATER_BYTES).unwrap();

        let icons = [waste, water, diaper];

        for item in 0..K {
            let icon = &icons[self.kinds[item]];
            match s.locs[item] {
                Location::Compost => {
                    let (x, y) = to_x_y(item, s.locs[item]);
                    self.draw_icon(x, y, icon, &mut pixmap);
                }
                Location::Trash => {
                    let (x, y) = to_x_y(item, s.locs[item]);
                    self.draw_icon(x, y, icon, &mut pixmap);
                }
                Location::Recycle => {
                    let (x, y) = to_x_y(item, s.locs[item]);
                    self.draw_icon(x, y, icon, &mut pixmap);
                }
                Location::InHand => {
                    self.draw_icon(300.0, 10.0, icon, &mut pixmap);
                }
            }
        }

        pixmap.save_png(file_path).unwrap();
    }
}
