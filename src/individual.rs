

use crate::polygon;
use crate::polygon::Polygon;
#[derive(Clone)]
pub struct Individual {
    pub chromosomes : Vec<Polygon>,
    pub fitness : f64
}

use imageproc::drawing::{draw_polygon_mut, Canvas};

use tiny_skia::*;

use tiny_skia::*;

pub fn draw_into_buffer(individual: &Individual, pixmap: &mut Pixmap) {
    for poly in &individual.chromosomes {
        let mut pb = PathBuilder::new();

        if let Some(first) = poly.points.first() {
            pb.move_to(first.x as f32, first.y as f32);

            for point in poly.points.iter().skip(1) {
                pb.line_to(point.x as f32, point.y as f32);
            }
            pb.close();
        }

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();

            paint.set_color_rgba8(
                poly.color[0],
                poly.color[1],
                poly.color[2],
                poly.color[3]
            );

            paint.anti_alias = false;

            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None
            );
        }
    }
}
pub fn initIndividual(start_count: i16,width : u32, height : u32) -> Individual {

    let mut polygons = vec![];
    for i in 0..start_count{
        polygons.push(polygon::CreatePolygon(7, width as u16, height as u16))
    }

    let mut indi = Individual{
        chromosomes: polygons,
        fitness : 0.0
    };

    indi
}


use image::{Pixel};
use rayon::prelude::*;
pub fn evaluate_with_buffer(img: &Pixmap, target_bytes: &[u8]) -> f64 {
    let img_bytes = img.data();
    let mut total_diff: u64 = 0;

    for (p1, p2) in img_bytes.chunks_exact(4).zip(target_bytes.chunks_exact(4)) {
        total_diff += (p1[0] as i32 - p2[0] as i32).abs() as u64;
        total_diff += (p1[1] as i32 - p2[1] as i32).abs() as u64;
        total_diff += (p1[2] as i32 - p2[2] as i32).abs() as u64;
    }

    let max_diff = (img.width() * img.height() * 3 * 255) as f64;
    1.0 - (total_diff as f64 / max_diff)
}