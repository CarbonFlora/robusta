use super::*;
// use bevy_render::mesh::{Mesh, PrimitiveTopology};

use crate::{angle_full_circle, point::Point};

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub definition: [crate::point::Point; 2],
}

impl Line {
    pub fn new(definition: [crate::point::Point; 2]) -> Self {
        Line { definition }
    }

    pub fn specifications(&self) -> LineSpec {
        let delta_x = self.definition[1].coordinates.x - self.definition[0].coordinates.x;
        let delta_y = self.definition[1].coordinates.y - self.definition[0].coordinates.y;
        // let slope = delta_y / delta_x;
        let slope = match delta_x {
            y if y == 0. => None,
            _ => Some(delta_y / delta_x),
        };
        let length = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        let h_angle = angle_full_circle(delta_x, delta_y);

        LineSpec {
            slope,
            h_angle,
            length,
        }
    }

    pub fn min_max(&self) -> (f32, f32, f32, f32) {
        crate::min_max(self.definition.as_ref())
    }

    pub fn endpoints(&self) -> Vec<Point> {
        vec![self.definition[0].clone(), self.definition[1].clone()]
    }

    pub fn midpoints(&self) -> Vec<Point> {
        let p0 = self.definition[0].coordinates;
        let p1 = self.definition[1].coordinates;
        let x = (p0.x + p1.x) / 2.;
        let y = (p0.y + p1.y) / 2.;
        vec![Point::new(x, y, 0.)]
    }

    pub fn nthpoints(&self, div: usize) -> Vec<Point> {
        let mut vp = Vec::new();
        let p0 = self.definition[0].coordinates;
        let p1 = self.definition[1].coordinates;
        let delta_x = (p1.x - p0.x) / (div as f32 + 1.);
        let delta_y = (p1.y - p0.y) / (div as f32 + 1.);
        for n in 1..=div {
            vp.push(Point::new(
                p0.x + delta_x * n as f32,
                p0.y + delta_y * n as f32,
                0.,
            ));
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
        let spec = self.specifications();
        MaterialMesh2dBundle {
            mesh: me.add(line_mesh(lw, spec.length, spec.h_angle)).into(),
            material: ma.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                self.definition[0].coordinates.x,
                self.definition[0].coordinates.y,
                tz.top() as f32,
            )),
            ..default()
        }
    }
}

fn line_mesh(line_width: f32, length: f32, angle_rad: f32) -> Mesh {
    let lw_half = line_width / 2.0f32;
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-lw_half * angle_rad.sin(), lw_half * angle_rad.cos(), 0.0],
            [lw_half * angle_rad.sin(), -lw_half * angle_rad.cos(), 0.0],
            [
                length * angle_rad.cos() + lw_half * angle_rad.sin(),
                length * angle_rad.sin() - lw_half * angle_rad.cos(),
                0.0,
            ],
            [
                length * angle_rad.cos() - lw_half * angle_rad.sin(),
                length * angle_rad.sin() + lw_half * angle_rad.cos(),
                0.0,
            ],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 3, 1, 1, 3, 2]))
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LineSpec {
    pub slope: Option<f32>, // rise / run
    pub h_angle: f32,       // in rad
    pub length: f32,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "1: {}\n2: {}\n{}",
            self.definition[0],
            self.definition[1],
            self.specifications()
        ))
    }
}

impl std::fmt::Display for LineSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut slope_real = String::new();
        let slope = match self.slope {
            None => "Undefined",
            Some(a) => {
                slope_real = format!("{a:.4}");
                ""
            }
        };
        f.write_fmt(format_args!(
            "Slope: {}{}\nRadians: {}\nLength: {:.4}",
            slope, slope_real, self.h_angle, self.length
        ))
    }
}

impl From<&dxf::entities::Line> for Line {
    fn from(sp: &dxf::entities::Line) -> Self {
        let point1 = Point::new(sp.p1.x as f32, sp.p1.y as f32, 0.);
        let point2 = Point::new(sp.p2.x as f32, sp.p2.y as f32, 0.);
        Line::new([point1, point2])
    }
}
