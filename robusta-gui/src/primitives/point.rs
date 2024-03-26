use super::*;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn mesh(
        &self,
        tf: &TagFlags,
        me: &mut ResMut<Assets<Mesh>>,
        ma: &mut ResMut<Assets<ColorMaterial>>,
        tz: &mut TopZLayer,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: me.add(self.point_mesh(tf.thickness_or_default())).into(),
            material: ma.add(ColorMaterial::from(tf.color_or_default())),
            transform: Transform::from_translation(Vec3::new(
                self.coordinates.x,
                self.coordinates.y,
                tz.top() as f32,
            )),
            ..default()
        }
    }

    pub fn point_mesh(&self, thickness: f32) -> Mesh {
        bevy::math::primitives::Circle::new(thickness / 2.).into()
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

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coordinates.partial_cmp(&other.coordinates)
    }
}
