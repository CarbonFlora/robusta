use crate::angle_full_circle;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Line {
    pub definition: [crate::point::Point; 2],
}

impl Line {
    pub fn new(definition: [crate::point::Point; 2]) -> Self {
        return Line { definition };
    }

    pub fn specifications(&self) -> LineSpec {
        let delta_x = self.definition[1].coordinates.x - self.definition[0].coordinates.x;
        let delta_y = self.definition[1].coordinates.y - self.definition[0].coordinates.y;
        let slope = delta_y / delta_x;
        let length = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        let h_angle = angle_full_circle(delta_x, delta_y);

        return LineSpec {
            slope,
            h_angle,
            length,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LineSpec {
    pub slope: f32,   // rise / run
    pub h_angle: f32, // in rad
    pub length: f32,
}
