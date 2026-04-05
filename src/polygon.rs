
use image::{Rgba, RgbaImage}; // Switched to Rgba

use imageproc::point::Point;
use rand::Rng;
#[derive(Clone)]
pub struct Polygon{
    pub(crate) points: Vec<Point<i32>>,
    pub(crate) color: Rgba<u8>
}

pub fn CreatePolygon(maxSides : u8, width : u16, height: u16) -> Polygon{

    let mut points = vec![];
    let mut rng = rand::thread_rng();
    let sides = rand::thread_rng().gen_range(3..=maxSides);

    let center = vec![rng.gen_range(50..=width-50),rng.gen_range(50..=width-50)];

    for _i in 0..sides {
        let rnx = center[0] + rng.gen_range(0..=70)-35;
        let rny = center[1] + rng.gen_range(0..=70)-35;
        let point = Point::new(rnx as i32, rny as i32);
        points.push(point);
    }
    let r =rng.gen_range(20..=220);
    let g=rng.gen_range(20..=220);

    let b=rng.gen_range(20..=220);
    let a=rng.gen_range(20..=220);

    let mut avg_x = 0.0 ;
    let mut avg_y = 0.0;
    for p in &points{
        avg_x += p.x as f64;
        avg_y += p.y as f64;
    }
    avg_x /= points.len() as f64;
    avg_y /= points.len() as f64;

    points.sort_by(|a, b| {
        let angle_a = (a.y as f64 - avg_y).atan2(a.x as f64  - avg_x);
        let angle_b = (b.y  as f64 - avg_y).atan2(b.x as f64  - avg_x);
        angle_a.total_cmp(&angle_b)
    });


    let polygon = Polygon{
        points : points,
        color : Rgba([r, g, b,a])
    };

    polygon
}