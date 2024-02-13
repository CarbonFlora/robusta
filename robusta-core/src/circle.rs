#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Circle {
    pub definition: [crate::point::Point; 2],
}

impl Circle {
    pub fn new(definition: [crate::point::Point; 2]) -> Self {
        return Circle { definition };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CircleSpec {}
