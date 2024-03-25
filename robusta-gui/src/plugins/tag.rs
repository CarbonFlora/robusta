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

#[derive(Debug, Component, Clone)]
pub struct TagList {
    pub taglist: Vec<Tag>,
    // pub ordered_taglist: Vec<Tag>,
}

impl Default for TagList {
    fn default() -> Self {
        let taglist = vec![Tag::new("Default".to_string())];
        // let mut taglist = HashSet::new();
        // taglist.insert(Tag::new("Default".to_string()));

        Self { taglist }
    }
}

impl TagList {
    pub fn remove_tag(&mut self, tag: &Tag) {
        self.taglist.retain(|x| x != tag);
    }
}

#[derive(Debug, Clone)]
pub struct TagFlags {
    pub color: Option<egui::Color32>,
    pub thickness: Option<f32>,
}

impl TagFlags {
    pub fn update(&mut self, flag: &Flag) {
        match flag {
            Flag::Color(sp) => {
                self.color = *sp;
            }
            Flag::Thickness(sp) => {
                self.thickness = *sp;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Flag {
    Color(Option<egui::Color32>),
    Thickness(Option<f32>),
}

#[derive(Debug, Resource, Default)]
pub struct TagCharacteristics {
    pub tag_flags: HashMap<Tag, TagFlags>,
}

impl TagCharacteristics {
    pub fn new() -> Self {
        let mut tag_flags = HashMap::new();
        tag_flags.insert(Tag::new("Default".to_string()), TagFlags::default());

        Self { tag_flags }
    }

    pub fn get(&mut self, t: &Tag) -> &TagFlags {
        if !self.tag_flags.contains_key(t) {
            self.tag_flags.insert(t.clone(), TagFlags::default());
        }
        self.tag_flags.get(t).unwrap()
    }

    pub fn get_mut(&mut self, t: &Tag) -> &mut TagFlags {
        if !self.tag_flags.contains_key(t) {
            self.tag_flags.insert(t.clone(), TagFlags::default());
        }
        self.tag_flags.get_mut(t).unwrap()
    }

    pub fn insert(&mut self, k: Tag, v: TagFlags) {
        self.tag_flags.insert(k, v);
    }

    pub fn remove(&mut self, k: &Tag) {
        self.tag_flags.remove(k);
    }

    pub fn len(&self) -> usize {
        self.tag_flags.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tag_flags.len() == 0
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
    NewColor(Tag, Option<Color32>),
}

pub fn update_act_tag(
    //Input
    mut era: EventReader<Act>,
    //Util
    mut es: Query<(&REntity, &mut TagList), With<Selected>>,
    //Output
    mut rmtc: ResMut<TagCharacteristics>,
    mut ewdbm: EventWriter<DockBufferModify>,
    mut ewrs: EventWriter<RefreshStyle>,
) {
    for act in era.read() {
        match act {
            Act::ModifyTag(re, tm) => {
                let mut ret = es
                    .iter_mut()
                    .find(|x| x.0 == re)
                    .expect("REntity in selection doesn't exist in world.");

                match tm {
                    TagModify::Add(sp) => {
                        ret.1.taglist.push(sp.clone());
                        ewdbm.send(DockBufferModify::AddTag(ret.0.clone(), sp.clone()));
                    }
                    TagModify::Remove(sp) => {
                        ret.1.remove_tag(sp);
                        ewdbm.send(DockBufferModify::RemoveTag(ret.0.clone(), sp.clone()));
                    }
                    TagModify::RemoveAll => {
                        ret.1.taglist.clear();
                        ewdbm.send(DockBufferModify::RemoveAllTags(ret.0.clone()));
                    }
                };
            }
            Act::ModifyTaglist(tlm) => {
                match tlm {
                    TagListModify::Add(t) => {
                        rmtc.insert(t.clone(), TagFlags::all_none());
                        ewdbm.send(DockBufferModify::TagListAdd(t.clone()));
                    }
                    TagListModify::Remove(t) => {
                        rmtc.remove(t);
                        ewdbm.send(DockBufferModify::TagListRemove(t.clone()));
                    }
                    TagListModify::NewColor(t, c32) => {
                        let tf = rmtc.get_mut(t);
                        tf.color = *c32;
                        ewrs.send(RefreshStyle::Color);
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
            TagListModify::NewColor(t, c32) => {
                format!("Changed tag \"{}\" color to {:?}", t.name, c32)
            }
        };
        f.write_fmt(format_args!("{b}"))
    }
}
