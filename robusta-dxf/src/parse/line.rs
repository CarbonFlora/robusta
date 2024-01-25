use crate::*;

/// Returns the two endpoints of a line.
pub fn to_points(specific: &Line) -> [Point; 2] {
    let point1 = Point::new(specific.p1.x as f32, specific.p1.y as f32, 0.);
    let point2 = Point::new(specific.p2.x as f32, specific.p2.y as f32, 0.);
    return [point1, point2];
}
