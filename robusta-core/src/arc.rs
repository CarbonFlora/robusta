use crate::{angle_full_circle, point::Point, PI};

#[derive(Debug, Clone, PartialEq, Default)]
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
}

fn circle_specs(definition: &[crate::point::Point; 3]) -> (f32, Point) {
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
    (radius, Point::new(x_center, y_center, 0.))
}

#[derive(Debug, Clone, PartialEq, Default)]
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
