use crate::*;

/// Returns the two endpoints and center of an arc.
pub fn to_points(specific: &Arc) -> [Point; 3] {
    // presuming start angle is beginning of Q1.
    let x1 = specific.center.x + specific.start_angle.to_radians().cos() * specific.radius;
    let y1 = specific.center.y + specific.start_angle.to_radians().sin() * specific.radius;
    let point1 = Point::new(x1 as f32, y1 as f32, 0.);

    let x2 = specific.center.x + specific.end_angle.to_radians().cos() * specific.radius;
    let y2 = specific.center.y + specific.end_angle.to_radians().sin() * specific.radius;
    let point2 = Point::new(x2 as f32, y2 as f32, 0.);

    let point3 = Point::new(specific.center.x as f32, specific.center.y as f32, 0.);

    return [point1, point2, point3];
}
