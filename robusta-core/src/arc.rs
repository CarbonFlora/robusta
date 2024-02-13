use crate::{angle_full_circle, PI};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Arc {
    pub definition: [crate::point::Point; 3],
}

impl Arc {
    pub fn new(definition: [crate::point::Point; 3]) -> Self {
        return Arc { definition };
    }

    pub fn specifications(&self) -> ArcSpec {
        let (radius, center) = circle_specs(self.definition);
        let start_angle_rad = angle_full_circle(
            self.definition[0].coordinates.x - center[0],
            self.definition[0].coordinates.y - center[1],
        );
        let end_angle_rad = angle_full_circle(
            self.definition[1].coordinates.x - center[0],
            self.definition[1].coordinates.y - center[1],
        );

        let mut angle = (end_angle_rad - start_angle_rad).abs();
        if end_angle_rad < start_angle_rad {
            angle = (2. * PI) - angle;
        }

        return ArcSpec {
            radius,
            center,
            start_angle_rad,
            end_angle_rad,
            angle,
        };
    }
}

fn circle_specs(definition: [crate::point::Point; 3]) -> (f32, [f32; 3]) {
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
    return (radius, [x_center, y_center, 0.]);
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ArcSpec {
    pub radius: f32,
    pub center: [f32; 3],
    pub start_angle_rad: f32,
    pub end_angle_rad: f32,
    pub angle: f32,
}
