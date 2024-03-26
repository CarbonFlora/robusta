use self::plugins::tag::TagFlags;

use super::*;
use std::f32::consts::PI;

pub mod arc;
pub mod circle;
pub mod line;
pub mod point;
pub mod text;

pub fn angle_full_circle(delta_x: f32, delta_y: f32) -> f32 {
    if delta_x == 0. && delta_y == 0. {
        panic!("At least two points are identical.")
    } else if delta_x == 0. {
        match delta_y.is_sign_positive() {
            true => return PI * 0.5,
            false => return PI * 1.5,
        }
    }

    let mut angle_rad = (delta_y / delta_x).atan();
    if angle_rad.is_sign_negative() {
        angle_rad += 2.0 * PI;
    }
    if delta_x.is_sign_negative() {
        angle_rad += PI;
    }
    angle_rad % (2. * PI)
}

pub fn rad_to_deg_string(rad: &f32) -> String {
    let deg = (rad % (2. * PI)) * 180. / PI;
    format!("{}°", deg)
}

pub fn min_max(points: &[crate::point::Point]) -> (f32, f32, f32, f32) {
    let mut min_x = points[0].coordinates.x;
    let mut max_x = points[0].coordinates.x;
    let mut min_y = points[0].coordinates.y;
    let mut max_y = points[0].coordinates.y;

    for point in points.iter() {
        if point.coordinates.x < min_x {
            min_x = point.coordinates.x;
        }
        if point.coordinates.x > max_x {
            max_x = point.coordinates.x;
        }
        if point.coordinates.y < min_y {
            min_y = point.coordinates.y;
        }
        if point.coordinates.y > max_y {
            max_y = point.coordinates.y;
        }
    }

    (min_x, min_y, max_x, max_y)
}

// pub trait RPrimitive {
//     pub fn new(definition: [crate::point::Point]) -> Self {}

//     pub fn specifications(&self) -> ArcSpec {
//         let (radius, center) = circle_specs(&self.definition);
//         let start_angle_rad = angle_full_circle(
//             self.definition[0].coordinates.x - center.coordinates.x,
//             self.definition[0].coordinates.y - center.coordinates.y,
//         );
//         let end_angle_rad = angle_full_circle(
//             self.definition[1].coordinates.x - center.coordinates.x,
//             self.definition[1].coordinates.y - center.coordinates.y,
//         );

//         let mut angle = (end_angle_rad - start_angle_rad).abs();
//         if end_angle_rad < start_angle_rad {
//             angle = (2. * PI) - angle;
//         }

//         ArcSpec {
//             radius,
//             center,
//             start_angle_rad,
//             end_angle_rad,
//             angle,
//         }
//     }

//     pub fn min_max(&self) -> (f32, f32, f32, f32) {
//         crate::min_max(self.definition.as_ref())
//     }

//     pub fn endpoints(&self) -> Vec<point::Point> {
//         vec![self.definition[0].clone(), self.definition[1].clone()]
//     }

//     pub fn midpoints(&self) -> Vec<point::Point> {
//         let spec = self.specifications();
//         let mut end_angle = spec.start_angle_rad;
//         if spec.start_angle_rad > spec.end_angle_rad {
//             end_angle += 2. * PI;
//         }
//         let mid_angle = (end_angle + spec.end_angle_rad) / 2.;
//         let x = mid_angle.cos() * spec.radius + spec.center.coordinates.x;
//         let y = mid_angle.sin() * spec.radius + spec.center.coordinates.y;
//         let mid_point = point::Point::new(x, y, 0.);

//         vec![mid_point]
//     }

//     pub fn center(&self) -> Vec<point::Point> {
//         let (_radius, center) = circle_specs(&self.definition);
//         vec![center]
//     }

//     pub fn nthpoints(&self, div: usize) -> Vec<point::Point> {
//         let mut points = Vec::new();
//         let spec = self.specifications();
//         let mut end_angle = spec.end_angle_rad;
//         if spec.start_angle_rad > spec.end_angle_rad {
//             end_angle += 2. * PI;
//         }
//         let angle_div = (end_angle - spec.start_angle_rad) / (div as f32 + 1.);
//         for n in 1..=div {
//             let x = (n as f32 * angle_div + spec.start_angle_rad).cos() * spec.radius
//                 + spec.center.coordinates.x;
//             let y = (n as f32 * angle_div + spec.start_angle_rad).sin() * spec.radius
//                 + spec.center.coordinates.y;
//             points.push(point::Point::new(x, y, 0.));
//         }

//         points
//     }

//     fn mesh_bundle(
//         &self,
//         tf: &TagFlags,
//         me: &mut ResMut<Assets<Mesh>>,
//         ma: &mut ResMut<Assets<ColorMaterial>>,
//         tz: &mut TopZLayer,
//     ) -> MaterialMesh2dBundle<ColorMaterial> {
//         panic!("Mesh bundle implementation required.")
//     }

//     fn mesh(&self, line_width: f32) -> Mesh {
//         panic!("Mesh implementation required.")
//     }
// }
