#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    pub definition: [crate::point::Point; 1],
    pub body: String,
    pub rotation: f32,
    pub height: f32,
}

impl Text {
    pub fn new(origin: crate::point::Point) -> Self {
        Text {
            definition: [origin],
            body: String::new(),
            rotation: 0.,
            height: 10.,
        }
    }

    pub fn min_max(&self) -> (f32, f32, f32, f32) {
        // This is temp as text is not implimented.
        crate::min_max(&self.definition.clone())
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Origin: {}", self.definition[0]))
    }
}
