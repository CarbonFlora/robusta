use std::f64::consts::PI;

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

    let mut p3_angle_rad = ((specific.start_angle + specific.end_angle) / 2.).to_radians();
    if specific.start_angle > specific.end_angle {
        p3_angle_rad -= PI;
    }

    let (p3_x, p3_y) = (
        specific.center.x + specific.radius * p3_angle_rad.cos(),
        specific.center.y + specific.radius * p3_angle_rad.sin(),
    );
    let lazy_point = Point::new(p3_x as f32, p3_y as f32, 0.);

    [point1, point2, lazy_point]
}

/// Returns a arc segment.
pub fn to_segment(specific: &Arc) -> robusta_core::RobustaEntity {
    robusta_core::RobustaEntity::Arc(robusta_core::arc::Arc {
        definition: to_points(specific),
    })
}
