#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Arc {
    pub definition: [crate::point::Point; 3],
}

impl Arc {
    pub fn new(definition: [crate::point::Point; 3]) -> Self {
        return Arc { definition };
    }
}
