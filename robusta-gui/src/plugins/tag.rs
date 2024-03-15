use bevy::utils::HashMap;

use super::*;

pub struct TagPlugin;
impl bevy::app::Plugin for TagPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TagCharacteristics::new());
    }
}

#[derive(Debug, Default)]
pub struct Tag {
    name: String,
}

#[derive(Debug, Component, Default)]
pub struct Tags {
    taglist: Vec<Tag>,
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
        Self {
            tag_flags: HashMap::new(),
        }
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
