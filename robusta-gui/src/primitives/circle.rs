use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub definition: [crate::point::Point; 2], // [@ angle=0, center]
}

impl Circle {
    pub fn new(definition: [crate::point::Point; 2]) -> Self {
        Circle { definition }
    }

    pub fn specifications(&self) -> CircleSpec {
        let radius = (self.definition[0].coordinates.x - self.definition[1].coordinates.x).abs();

        CircleSpec { radius }
    }

    pub fn min_max(&self) -> (f32, f32, f32, f32) {
        crate::min_max(self.definition.as_ref())
    }

    pub fn center(&self) -> Vec<point::Point> {
        vec![self.definition[1].clone()]
    }

    pub fn nthpoints(&self, div: usize) -> Vec<point::Point> {
        let mut vp = Vec::new();
        let spec = self.specifications();
        let angle_div = 2. * PI / (div as f32 + 1.);

        vp.push(point::Point::new(
            self.definition[0].coordinates.x,
            self.definition[0].coordinates.y,
            0.,
        ));
        for n in 1..=div {
            let x = (angle_div * n as f32).cos() * spec.radius + self.definition[1].coordinates.x;
            let y = (angle_div * n as f32).sin() * spec.radius + self.definition[1].coordinates.y;
            vp.push(point::Point::new(x, y, 0.));
        }

        vp
    }

    pub fn mesh(
        &self,
        me: &mut ResMut<Assets<Mesh>>,
        ma: &mut ResMut<Assets<ColorMaterial>>,
        tz: &mut TopZLayer,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        let lw = 0.3f32;
        MaterialMesh2dBundle {
            mesh: me.add(circle_mesh(lw, self)).into(),
            material: ma.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., tz.top() as f32)),
            ..default()
        }
    }
}

fn circle_mesh(line_width: f32, circle: &circle::Circle) -> Mesh {
    let lw_half = line_width / 2.0f32;
    let num_segments = 30u32;
    let vertexes: Vec<[f32; 3]> = circle_vertexes(num_segments, circle, lw_half);
    let triangle_indexes: Vec<u32> = arc_indexes(num_segments);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; vertexes.len()])
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertexes)
    .with_inserted_indices(Indices::U32(triangle_indexes))
}

fn arc_indexes(num_segments: u32) -> Vec<u32> {
    let mut a = Vec::new();

    for i in 0..(num_segments * 2) {
        a.extend(vec![i, i + 1, i + 2]);
    }

    a
}

fn circle_vertexes(num_segments: u32, circle: &circle::Circle, lw_half: f32) -> Vec<[f32; 3]> {
    let mut vertexes = Vec::new();
    let spec = circle.specifications();
    let angle_increment = (2. * PI) / num_segments as f32;

    for i in 0..=num_segments {
        let angle_offset = angle_increment * i as f32;

        let x_outer =
            circle.definition[1].coordinates.x + (spec.radius + lw_half) * (angle_offset).cos();
        let y_outer =
            circle.definition[1].coordinates.y + (spec.radius + lw_half) * (angle_offset).sin();
        let x_inner =
            circle.definition[1].coordinates.x + (spec.radius - lw_half) * (angle_offset).cos();
        let y_inner =
            circle.definition[1].coordinates.y + (spec.radius - lw_half) * (angle_offset).sin();

        vertexes.push([x_outer, y_outer, 0.]);
        vertexes.push([x_inner, y_inner, 0.]);
    }

    vertexes
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CircleSpec {
    pub radius: f32,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Center: {}\n{}",
            self.definition[1],
            self.specifications()
        ))
    }
}

impl std::fmt::Display for CircleSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Radius: {:.4}", self.radius))
    }
}
