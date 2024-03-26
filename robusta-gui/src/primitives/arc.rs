use self::plugins::tag::TagFlags;

use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Arc {
    pub definition: [crate::point::Point; 3],
}

impl Arc {
    pub fn new(definition: [crate::point::Point; 3]) -> Self {
        Arc { definition }
    }

    pub fn specifications(&self) -> ArcSpec {
        let (radius, center) = circle_specs(&self.definition);
        let start_angle_rad = angle_full_circle(
            self.definition[0].coordinates.x - center.coordinates.x,
            self.definition[0].coordinates.y - center.coordinates.y,
        );
        let end_angle_rad = angle_full_circle(
            self.definition[1].coordinates.x - center.coordinates.x,
            self.definition[1].coordinates.y - center.coordinates.y,
        );

        let mut angle = (end_angle_rad - start_angle_rad).abs();
        if end_angle_rad < start_angle_rad {
            angle = (2. * PI) - angle;
        }

        ArcSpec {
            radius,
            center,
            start_angle_rad,
            end_angle_rad,
            angle,
        }
    }

    pub fn min_max(&self) -> (f32, f32, f32, f32) {
        crate::min_max(self.definition.as_ref())
    }

    pub fn endpoints(&self) -> Vec<point::Point> {
        vec![self.definition[0].clone(), self.definition[1].clone()]
    }

    pub fn midpoints(&self) -> Vec<point::Point> {
        let spec = self.specifications();
        let mut end_angle = spec.start_angle_rad;
        if spec.start_angle_rad > spec.end_angle_rad {
            end_angle += 2. * PI;
        }
        let mid_angle = (end_angle + spec.end_angle_rad) / 2.;
        let x = mid_angle.cos() * spec.radius + spec.center.coordinates.x;
        let y = mid_angle.sin() * spec.radius + spec.center.coordinates.y;
        let mid_point = point::Point::new(x, y, 0.);

        vec![mid_point]
    }

    pub fn center(&self) -> Vec<point::Point> {
        let (_radius, center) = circle_specs(&self.definition);
        vec![center]
    }

    pub fn nthpoints(&self, div: usize) -> Vec<point::Point> {
        let mut points = Vec::new();
        let spec = self.specifications();
        let mut end_angle = spec.end_angle_rad;
        if spec.start_angle_rad > spec.end_angle_rad {
            end_angle += 2. * PI;
        }
        let angle_div = (end_angle - spec.start_angle_rad) / (div as f32 + 1.);
        for n in 1..=div {
            let x = (n as f32 * angle_div + spec.start_angle_rad).cos() * spec.radius
                + spec.center.coordinates.x;
            let y = (n as f32 * angle_div + spec.start_angle_rad).sin() * spec.radius
                + spec.center.coordinates.y;
            points.push(point::Point::new(x, y, 0.));
        }

        points
    }

    pub fn mesh(
        &self,
        tf: &TagFlags,
        me: &mut ResMut<Assets<Mesh>>,
        ma: &mut ResMut<Assets<ColorMaterial>>,
        tz: &mut TopZLayer,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: me.add(self.arc_mesh(tf.thickness_or_default())).into(),
            material: ma.add(ColorMaterial::from(tf.color_or_default())),
            transform: Transform::from_translation(Vec3::new(0., 0., tz.top() as f32)),
            ..default()
        }
    }

    pub fn arc_mesh(&self, line_width: f32) -> Mesh {
        let lw_half = line_width / 2.0f32;
        let num_segments = 30u32;
        let vertexes: Vec<[f32; 3]> = arc_vertexes(num_segments, self, lw_half);
        let triangle_indexes: Vec<u32> = arc_indexes(num_segments);

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; vertexes.len()])
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertexes)
        .with_inserted_indices(Indices::U32(triangle_indexes))
    }
}

fn arc_vertexes(num_segments: u32, arc: &arc::Arc, lw_half: f32) -> Vec<[f32; 3]> {
    let mut vertexes = Vec::new();
    let spec = arc.specifications();
    let angle_increment = spec.angle / num_segments as f32;

    for i in 0..=num_segments {
        let angle_offset = spec.start_angle_rad + angle_increment * i as f32;

        let x_outer = spec.center.coordinates.x + (spec.radius + lw_half) * (angle_offset).cos();
        let y_outer = spec.center.coordinates.y + (spec.radius + lw_half) * (angle_offset).sin();
        let x_inner = spec.center.coordinates.x + (spec.radius - lw_half) * (angle_offset).cos();
        let y_inner = spec.center.coordinates.y + (spec.radius - lw_half) * (angle_offset).sin();

        vertexes.push([x_outer, y_outer, 0.]);
        vertexes.push([x_inner, y_inner, 0.]);
    }

    vertexes
}

fn arc_indexes(num_segments: u32) -> Vec<u32> {
    let mut a = Vec::new();

    for i in 0..(num_segments * 2) {
        a.extend(vec![i, i + 1, i + 2]);
    }

    a
}

fn circle_specs(definition: &[crate::point::Point; 3]) -> (f32, point::Point) {
    let i_11 = definition[0].coordinates.x.powi(2) + definition[0].coordinates.y.powi(2);
    let i_21 = definition[1].coordinates.x.powi(2) + definition[1].coordinates.y.powi(2);
    let i_31 = definition[2].coordinates.x.powi(2) + definition[2].coordinates.y.powi(2);

    let m_14 = nalgebra::Matrix3::new(
        i_11,
        definition[0].coordinates.x,
        definition[0].coordinates.y,
        i_21,
        definition[1].coordinates.x,
        definition[1].coordinates.y,
        i_31,
        definition[2].coordinates.x,
        definition[2].coordinates.y,
    )
    .determinant();
    let m_13 = nalgebra::Matrix3::new(
        i_11,
        definition[0].coordinates.x,
        1.,
        i_21,
        definition[1].coordinates.x,
        1.,
        i_31,
        definition[2].coordinates.x,
        1.,
    )
    .determinant();
    let m_12 = nalgebra::Matrix3::new(
        i_11,
        definition[0].coordinates.y,
        1.,
        i_21,
        definition[1].coordinates.y,
        1.,
        i_31,
        definition[2].coordinates.y,
        1.,
    )
    .determinant();
    let m_11 = nalgebra::Matrix3::new(
        definition[0].coordinates.x,
        definition[0].coordinates.y,
        1.,
        definition[1].coordinates.x,
        definition[1].coordinates.y,
        1.,
        definition[2].coordinates.x,
        definition[2].coordinates.y,
        1.,
    )
    .determinant();

    if m_11 == 0. {
        panic!("not a circle.");
    }

    let x_center = 1. / 2. * m_12 / m_11;
    let y_center = -1. / 2. * m_13 / m_11;
    let radius = (x_center.powi(2) + y_center.powi(2) + m_14 / m_11).sqrt();
    (radius, point::Point::new(x_center, y_center, 0.))
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArcSpec {
    pub radius: f32,
    pub center: crate::point::Point,
    pub start_angle_rad: f32,
    pub end_angle_rad: f32,
    pub angle: f32,
}

impl std::fmt::Display for Arc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "1: {}\n2: {}\n{}",
            self.definition[0],
            self.definition[1],
            self.specifications()
        ))
    }
}

impl std::fmt::Display for ArcSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Center: {}\nRadius: {:.4}\nAngle: {:.4}\nStart Angle: {:.4}\nEnd Angle: {:.4}",
            self.center, self.radius, self.angle, self.start_angle_rad, self.end_angle_rad
        ))
    }
}
