use crate::*;

/// Returns all the vertices in a polyline.
pub fn to_points(specific: &LwPolyline) -> Vec<Point> {
    let mut points = Vec::new();
    for vertex in &specific.vertices {
        points.push(Point::new(vertex.x as f32, vertex.y as f32, 0.));
    }

    points
}

// Returns a vector of line segments.
// pub fn to_segments(specific: &LwPolyline) -> Vec<robusta_core::RobustaEntity> {
//     let mut lines = Vec::new();
//     let mut iter = specific.vertices.iter();

//     if let Some(mut lagging) = iter.next() {
//         for latest in iter {
//             lines.push(robusta_core::RobustaEntity::Line(
//                 robusta_core::line::Line::new([
//                     Point::new(lagging.x as f32, lagging.y as f32, 0.),
//                     Point::new(latest.x as f32, latest.y as f32, 0.),
//                 ]),
//             ));

//             lagging = latest;
//         }
//     }

//     lines
// }
