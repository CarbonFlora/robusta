#[derive(Debug, Clone, PartialEq, Default, PartialOrd)]
pub struct Point {
    pub coordinates: nalgebra::Point3<f32>,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point {
            coordinates: nalgebra::Point3::new(x, y, z),
        }
    }

    pub fn xyz(&self) -> [f32; 3] {
        [self.coordinates.x, self.coordinates.y, self.coordinates.z]
    }

    pub fn xy_mut(&mut self, x: f32, y: f32) {
        self.coordinates.x = x;
        self.coordinates.y = y;
    }

    pub fn xyz_mut(&mut self, x: f32, y: f32, z: f32) {
        self.coordinates.x = x;
        self.coordinates.y = y;
        self.coordinates.z = z;
    }

    pub fn min_max(&self) -> (f32, f32, f32, f32) {
        (
            self.coordinates.x,
            self.coordinates.y,
            self.coordinates.x,
            self.coordinates.y,
        )
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({:.4}, {:.4}, {:.4})",
            self.coordinates.x, self.coordinates.y, self.coordinates.z
        ))
    }
}
