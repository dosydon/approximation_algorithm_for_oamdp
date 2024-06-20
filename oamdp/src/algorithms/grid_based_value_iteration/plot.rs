use crate::algorithms::assoc_belief_point::AssocBeliefPointN;
use crate::algorithms::belief_point::BeliefPoint;
use crate::algorithms::regular_grid_belief_points::RegularGridBeliefPoints;
use core::fmt::Debug;
use gnuplot::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

impl<A: Copy + Debug + Hash + Eq> RegularGridBeliefPoints<AssocBeliefPointN<A, 2>, 2> {
    pub fn plot(
        &self,
        filename: &str,
        x_label: &str,
        y_min: f64,
        y_max: f64,
        color_map: &HashMap<Option<A>, String>,
    ) {
        let mut per_action_x = HashMap::<Option<A>, Vec<f32>>::new();
        let mut per_action_y = HashMap::<Option<A>, Vec<f32>>::new();
        //         let mut x = vec![];
        //         let mut y = vec![];
        for b in self.grid.values() {
            if let Some(x) = per_action_x.get_mut(&b.assoc) {
                x.push(b.inner()[0].into_inner());
            } else {
                per_action_x.insert(b.assoc, vec![b.inner()[0].into_inner()]);
            }

            if let Some(y) = per_action_y.get_mut(&b.assoc) {
                y.push(b.assoc_value().into_inner());
            } else {
                per_action_y.insert(b.assoc, vec![b.assoc_value().into_inner()]);
            }
        }

        let mut fg = Figure::new();
        let axes2d = fg
            .axes2d()
            .set_x_range(Fix(0.0), Fix(1.0))
            .set_y_range(Fix(y_min), Fix(y_max))
            .set_x_ticks(Some((Auto, 0)), &[], &[Font("Arial", 18.0)])
            .set_y_ticks(Some((Auto, 0)), &[], &[Font("Arial", 18.0)])
            .set_x_label(x_label, &[Font("", 18.0)])
            .set_y_label("Value", &[Font("", 18.0)]);

        for a in per_action_x.keys() {
            let x = per_action_x.get(a).unwrap();
            let y = per_action_y.get(a).unwrap();
            if let Some(c) = color_map.get(a) {
                axes2d.points(x, y, &[Caption(&format!("{:?}", a)), Color(c)]);
            } else {
                panic!("{:?}", a);
            }
        }
        //         axes2d.points(&x, &y, &[]);

        let fpath = Path::new(filename);
        fg.set_terminal("png", &*fpath.to_string_lossy());
        fg.show().unwrap();
    }
}
