use super::*;

pub struct TagPlugin;
impl bevy::app::Plugin for TagPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TagCharacteristics::new());
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Component, Default, Clone)]
pub struct Tags {
    pub taglist: Vec<Tag>,
}

#[derive(Debug)]
pub struct TagFlags {
    color: Option<Color>,
    thickness: Option<f32>,
}

#[derive(Debug, Resource, Default)]
pub struct TagCharacteristics {
    tag_flags: HashMap<Tag, TagFlags>,
}

impl TagCharacteristics {
    pub fn new() -> Self {
        let mut tag_flags = HashMap::new();
        tag_flags.insert(Tag::new("Default".to_string()), TagFlags::default());

        Self { tag_flags }
    }

    pub fn flags(&mut self, t: &Tag) -> &TagFlags {
        if !self.tag_flags.contains_key(t) {
            self.tag_flags.insert(t.clone(), TagFlags::default());
        }
        self.tag_flags.get(t).unwrap()
    }
}

impl Default for TagFlags {
    fn default() -> Self {
        TagFlags {
            color: Some(Color::WHITE),
            thickness: Some(1.),
        }
    }
}
