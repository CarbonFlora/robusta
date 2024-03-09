use std::f32::consts::PI;

use crate::point::Point;

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

    pub fn center(&self) -> Vec<Point> {
        vec![self.definition[1].clone()]
    }

    pub fn nthpoints(&self, div: usize) -> Vec<Point> {
        let mut vp = Vec::new();
        let spec = self.specifications();
        let angle_div = 2. * PI / (div as f32 + 1.);

        vp.push(Point::new(
            self.definition[0].coordinates.x,
            self.definition[0].coordinates.y,
            0.,
        ));
        for n in 1..=div {
            let x = (angle_div * n as f32).cos() * spec.radius + self.definition[1].coordinates.x;
            let y = (angle_div * n as f32).sin() * spec.radius + self.definition[1].coordinates.y;
            vp.push(Point::new(x, y, 0.));
        }

        vp
    }
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
