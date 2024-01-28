use crate::*;

/// Returns a point at angle=0 on the radius, and the center of a circle.
pub fn to_points(specific: &Circle) -> [Point; 2] {
    let point1 = Point::new(
        (specific.center.x + specific.radius) as f32,
        specific.center.y as f32,
        0.,
    );
    let point2 = Point::new(specific.center.x as f32, specific.center.y as f32, 0.);

    return [point1, point2];
}