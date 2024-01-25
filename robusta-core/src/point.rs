#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub coordinates: nalgebra::Point3<f32>,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Point {
            coordinates: nalgebra::Point3::new(x, y, z),
        };
    }

    pub fn origin() -> Self {
        return Point::new(0., 0., 0.);
    }
}
