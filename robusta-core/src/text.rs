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
