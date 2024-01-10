#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Line {
    pub definition: [crate::point::Point;2]
}

impl Line {
    pub fn new(definition: [crate::point::Point;2]) -> Self {
        return Line {
            definition
        };
    }
}