use crate::*;

/// Returns all the vertices in a polyline.
pub fn to_points(specific: &LwPolyline) -> Vec<Point> {
    let mut points = Vec::new();
    for vertex in &specific.vertices {
        points.push(Point::new(vertex.x, vertex.y, 0.));
    }

    return points;
}
