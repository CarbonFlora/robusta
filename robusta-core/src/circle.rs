#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Circle {
    pub definition: [crate::point::Point; 2],
}

impl Circle {
    pub fn new(definition: [crate::point::Point; 2]) -> Self {
        return Circle { definition };
    }

    pub fn specifications(&self) -> CircleSpec {
        let center = [
            self.definition[1].coordinates.x,
            self.definition[1].coordinates.y,
        ];
        let radius = (self.definition[0].coordinates.x - self.definition[1].coordinates.x).abs();

        return CircleSpec { radius, center };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CircleSpec {
    pub radius: f32,
    pub center: [f32; 2],
}
