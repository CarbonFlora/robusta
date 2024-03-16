use std::ops::Index;

use bevy::utils::hashbrown::HashSet;

use super::*;

pub struct TagPlugin;
impl bevy::app::Plugin for TagPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TagCharacteristics::new())
            .add_systems(PreUpdate, update_act_tag);
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

    pub fn placeholder() -> Self {
        Self {
            name: "Untitled".to_string(),
        }
    }
}

#[derive(Debug, Component, Default, Clone)]
pub struct Tags {
    pub taglist: HashSet<Tag>,
}

#[derive(Debug, Clone)]
pub struct TagFlags {
    color: Option<Color>,
    thickness: Option<f32>,
}

#[derive(Debug, Resource, Default)]
pub struct TagCharacteristics {
    tag_flags: HashMap<Tag, TagFlags>,
    ordered_tag_list: Vec<(Tag, TagFlags)>,
}

impl TagCharacteristics {
    pub fn new() -> Self {
        let mut tag_flags = HashMap::new();
        tag_flags.insert(Tag::new("Default".to_string()), TagFlags::default());
        let ordered_tag_list = Vec::new();

        let mut a = Self {
            tag_flags,
            ordered_tag_list,
        };
        a.update_order();
        a
    }

    pub fn get(&mut self, t: &Tag) -> &TagFlags {
        if !self.tag_flags.contains_key(t) {
            self.tag_flags.insert(t.clone(), TagFlags::default());
        }
        self.tag_flags.get(t).unwrap()
    }

    /// Also can be used to update an existing entry.
    pub fn insert(&mut self, k: Tag, v: TagFlags) {
        self.tag_flags.insert(k, v);
        self.update_order();
    }

    pub fn remove(&mut self, k: &Tag) {
        self.tag_flags.remove(k);
        self.update_order();
    }

    fn update_order(&mut self) {
        let mut pairs = self
            .tag_flags
            .iter()
            .map(|x| (x.0.clone(), x.1.clone()))
            .collect::<Vec<_>>();
        pairs.sort_by(|a, b| a.0.name.cmp(&b.0.name));
        self.ordered_tag_list = pairs;
    }

    pub fn len(&self) -> usize {
        self.tag_flags.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tag_flags.len() == 0
    }

    pub fn ordered_tag_list(&self) -> &[(Tag, TagFlags)] {
        &self.ordered_tag_list
    }
}

impl Index<usize> for TagCharacteristics {
    type Output = (Tag, TagFlags);

    fn index(&self, index: usize) -> &Self::Output {
        &self.ordered_tag_list[index]
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

#[derive(Debug, PartialEq, Clone)]
pub enum TagModify {
    Add(Tag),
    AddPlaceholder,
    Remove(Tag),
    RemoveAll,
}

pub fn update_act_tag(
    mut era: EventReader<Act>,
    mut es: Query<(&REntity, &mut Tags), With<Selected>>,
) {
    for act in era.read() {
        if let Act::ModifyTag(re, tm) = act {
            let mut ret = es
                .iter_mut()
                .find(|x| x.0 == re)
                .expect("REntity in selection doesn't exist in world.");

            match tm {
                TagModify::Add(sp) => ret.1.taglist.insert(sp.clone()),
                TagModify::AddPlaceholder => ret.1.taglist.insert(Tag::placeholder()),
                TagModify::Remove(sp) => ret.1.taglist.remove(sp),
                TagModify::RemoveAll => {
                    ret.1.taglist.clear();
                    true
                }
            };
        }
    }
}

impl std::fmt::Display for TagModify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = match self {
            TagModify::Add(sp) => format!("Added tag, {}", sp.name),
            TagModify::AddPlaceholder => "Added placeholder tag to selection.".to_string(),
            TagModify::Remove(sp) => format!("Removed tag, {}", sp.name),
            TagModify::RemoveAll => "removed all tags from selection.".to_string(),
        };
        f.write_fmt(format_args!("{b}"))
    }
}
