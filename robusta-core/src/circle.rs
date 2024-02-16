#[derive(Debug, Clone, PartialEq, Default)]
pub struct Circle {
    pub definition: [crate::point::Point; 2], // [@ angle=0, center]
}

impl Circle {
    pub fn new(definition: [crate::point::Point; 2]) -> Self {
        return Circle { definition };
    }

    pub fn specifications(&self) -> CircleSpec {
        let radius = (self.definition[0].coordinates.x - self.definition[1].coordinates.x).abs();

        return CircleSpec { radius };
    }

    pub fn min_max(&self) -> (f32, f32, f32, f32) {
        return crate::min_max(&self.definition.to_vec());
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
