#[derive(Debug, Clone, PartialEq, Default, PartialOrd)]
pub struct Text {
    pub coordinates: crate::point::Point,
    pub body: String,
}

impl Text {
    pub fn new(origin: crate::point::Point) -> Self {
        return Text {
            coordinates: origin,
            body: String::new(),
        };
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Origin: {}", self.coordinates))
    }
}
