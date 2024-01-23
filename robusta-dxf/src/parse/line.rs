use crate::*;

/// Returns the two endpoints of a line.
pub fn to_points(specific: &Line) -> [Point; 2] {
    let point1 = Point::new(specific.p1.x, specific.p1.y, 0.);
    let point2 = Point::new(specific.p2.x, specific.p2.y, 0.);
    return [point1, point2];
}
