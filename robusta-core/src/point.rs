#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub coordinates: nalgebra::Point3<f64>,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Point {
            coordinates: nalgebra::Point3::new(x, y, z),
        };
    }

    pub fn origin() -> Self {
        return Point::new(0., 0., 0.);
    }
}
