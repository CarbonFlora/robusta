use std::f32::consts::PI;

pub mod arc;
pub mod circle;
pub mod line;
pub mod point;
pub mod text;

#[derive(Debug, Clone, PartialEq)]
pub enum RobustaEntity {
    Arc(arc::Arc),
    Circle(circle::Circle),
    Line(line::Line),
    Point(point::Point),
    Text(text::Text),
}

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
    return angle_rad % (2. * PI);
}

pub fn rad_to_deg_string(rad: &f32) -> String {
    let deg = (rad % (2. * PI)) * 180. / PI;
    return format!("{}°", deg);
}

pub fn min_max(points: &Vec<crate::point::Point>) -> (f32, f32, f32, f32) {
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
