use std::ops::{Index, IndexMut};

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

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn placeholder(n: Option<usize>) -> Self {
        let name = match n {
            Some(sp) => format!("Untitled-{}", sp),
            None => "Untitled".to_string(),
        };

        Self { name }
    }
}

#[derive(Debug, Component, Default, Clone)]
pub struct Tags {
    // taglist: HashSet<Tag>,
    pub ordered_taglist: Vec<Tag>,
}

// impl Tags {
//     pub fn new() -> Self {
//         let mut a = Self {
//             taglist: HashSet::new(),
//             ordered_taglist: Vec::new(),
//         };
//         a.update_order();
//         a
//     }

//     fn update_order(&mut self) {
//         let mut a = self.taglist.iter().map(|x| x.clone()).collect::<Vec<_>>();
//         a.sort_by(|a, b| a.name.cmp(&b.name));
//         self.ordered_taglist = a;
//     }

//     /// Also can be used to update an existing entry.
//     pub fn insert(&mut self, k: Tag) {
//         self.taglist.insert(k);
//         self.update_order();
//     }

//     pub fn remove(&mut self, k: &Tag) {
//         self.taglist.remove(k);
//         self.update_order();
//     }

//     pub fn len(&self) -> usize {
//         self.taglist.len()
//     }

//     #[must_use]
//     pub fn is_empty(&self) -> bool {
//         self.taglist.len() == 0
//     }

//     pub fn ordered_tag_list(&self) -> &[Tag] {
//         &self.ordered_taglist
//     }
// }

#[derive(Debug, Clone)]
pub struct TagFlags {
    pub color: Option<egui::Color32>,
    thickness: Option<f32>,
}

#[derive(Debug, Resource, Default)]
pub struct TagCharacteristics {
    tag_flags: HashMap<Tag, TagFlags>,
    ordered_tag_flags: Vec<(Tag, TagFlags)>,
}

impl TagCharacteristics {
    pub fn new() -> Self {
        let mut tag_flags = HashMap::new();
        tag_flags.insert(Tag::new("Default".to_string()), TagFlags::default());
        let ordered_tag_list = Vec::new();

        let mut a = Self {
            tag_flags,
            ordered_tag_flags: ordered_tag_list,
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
        self.ordered_tag_flags = pairs;
    }

    pub fn len(&self) -> usize {
        self.tag_flags.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tag_flags.len() == 0
    }

    pub fn ordered_tag_list(&self) -> &[(Tag, TagFlags)] {
        &self.ordered_tag_flags
    }
}

impl Index<usize> for TagCharacteristics {
    type Output = (Tag, TagFlags);

    fn index(&self, index: usize) -> &Self::Output {
        &self.ordered_tag_flags[index]
    }
}

impl IndexMut<usize> for TagCharacteristics {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ordered_tag_flags[index]
    }
}

impl Index<usize> for Tags {
    type Output = Tag;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ordered_taglist[index]
    }
}

impl IndexMut<usize> for Tags {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ordered_taglist[index]
    }
}

impl Default for TagFlags {
    fn default() -> Self {
        Self {
            color: Some(egui::Color32::WHITE),
            thickness: Some(1.),
        }
    }
}

impl TagFlags {
    pub fn all_none() -> Self {
        TagFlags {
            color: None,
            thickness: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TagModify {
    Add(Tag),
    Remove(Tag),
    RemoveAll,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TagListModify {
    Add(Tag),
    Remove(Tag),
}

pub fn update_act_tag(
    mut era: EventReader<Act>,
    mut es: Query<(&REntity, &mut Tags), With<Selected>>,
    mut rmtc: ResMut<TagCharacteristics>,
    mut db: ResMut<DockBuffer>,
) {
    for act in era.read() {
        match act {
            Act::ModifyTag(re, tm) => {
                let mut ret = es
                    .iter_mut()
                    .find(|x| x.0 == re)
                    .expect("REntity in selection doesn't exist in world.");

                match tm {
                    TagModify::Add(sp) => ret.1.ordered_taglist.push(sp.clone()),
                    TagModify::Remove(sp) => {
                        let index = ret.1.ordered_taglist.binary_search(sp);
                        if let Ok(w) = index {
                            ret.1.ordered_taglist.remove(w);
                        }
                    }
                    TagModify::RemoveAll => ret.1.ordered_taglist.clear(),
                };
            }
            Act::ModifyTaglist(tlm) => {
                match tlm {
                    TagListModify::Add(t) => {
                        rmtc.insert(t.clone(), TagFlags::all_none());
                    }
                    TagListModify::Remove(t) => {
                        rmtc.remove(t);
                        db.egui_selection.clear();
                    }
                };
            }
            _ => (),
        }
    }
}

impl std::fmt::Display for TagModify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = match self {
            TagModify::Add(sp) => format!("Added tag, {}", sp.name),
            TagModify::Remove(sp) => format!("Removed tag, {}", sp.name),
            TagModify::RemoveAll => "removed all tags from selection.".to_string(),
        };
        f.write_fmt(format_args!("{b}"))
    }
}

impl std::fmt::Display for TagListModify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = match self {
            TagListModify::Add(sp) => format!("Added tag to list: {}", sp.name),
            TagListModify::Remove(sp) => format!("Removed tag from list: {}", sp.name),
        };
        f.write_fmt(format_args!("{b}"))
    }
}
