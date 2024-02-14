#[derive(Debug, Clone, PartialEq, Default, PartialOrd)]
pub struct Point {
    // // This collects all the meshes' ids that rely on this entity.
    // pub bevy_ids: Vec<bevy::ecs::entity::Entity>,
    pub coordinates: nalgebra::Point3<f32>,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Point {
            // bevy_ids: Vec::new(),
            coordinates: nalgebra::Point3::new(x, y, z),
        };
    }

    pub fn origin() -> Self {
        return Point::new(0., 0., 0.);
    }

    pub fn xyz(&self) -> [f32; 3] {
        return [self.coordinates.x, self.coordinates.y, self.coordinates.z];
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
