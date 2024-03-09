use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub coordinates: nalgebra::Point3<f32>,
    pub appearance: PointAppearance,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointAppearance {
    color: Color,
    relative_size: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point {
            coordinates: nalgebra::Point3::new(x, y, z),
            appearance: PointAppearance {
                color: Color::WHITE,
                relative_size: 1.0f32,
            },
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

    // pub fn to_snap_type(&self) -> Point {
    //     Point {
    //         coordinates: self.coordinates,
    //         appearance: PointAppearance {
    //             color: Color::ORANGE,
    //             relative_size: 0.2,
    //         },
    //     }
    // }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({:.4}, {:.4}, {:.4})",
            self.coordinates.x, self.coordinates.y, self.coordinates.z
        ))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coordinates.partial_cmp(&other.coordinates)
    }
}
