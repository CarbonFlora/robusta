use self::{
    keystroke::ModalResources,
    leaves::{history::HistoryBuffer, inspection::InspectionBuffer},
};

use super::*;

pub struct DockPlugin;
impl bevy::app::Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DockBuffer::new())
            .add_event::<DockBufferModify>()
            .add_systems(Startup, spawn_window)
            .add_systems(Update, event_entity_to_rentity)
            .add_systems(Update, update_dock);
    }
}

#[derive(Debug, Default, Resource)]
pub struct DockBuffer {
    pub history: HistoryBuffer,
    pub inspection: InspectionBuffer,
    pub nth_n: String,
    pub egui_selection: HashMap<usize, Tag>,
    pub is_selection_mode: bool,
}

impl DockBuffer {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Event)]
pub enum DockBufferModify {
    AddSelected(Entity),
    RemoveSelected(Entity),
    AddTag(REntity, Tag),
    RemoveTag(REntity, Tag),
    RemoveAllTags(REntity),
}

/// Spawn a new window with reasonable defaults.
fn spawn_window(mut co: Commands) {
    co.spawn((
        window::Window {
            title: String::from("CADPanel"),
            // present_mode: bevy_window::PresentMode::AutoNoVsync,
            focused: false,
            ..Default::default()
        },
        CADPanel::default(),
    ));
}

#[allow(clippy::too_many_arguments)]
fn update_dock(
    act_write: EventWriter<Act>,
    ewdbm: EventWriter<DockBufferModify>,
    mut ewm: ResMut<ModalResources>,
    mut ui_state: ResMut<UiState>,
    ss: Res<SnapSettings>,
    qec: Query<&mut EguiContext, With<CADPanel>>,
    mut db: ResMut<DockBuffer>,
    mut tc: ResMut<TagCharacteristics>,
) {
    if let Ok(mut w) = qec.get_single().cloned() {
        ui_state.ui(
            w.get_mut(),
            act_write,
            ewdbm,
            &mut ewm,
            &mut db,
            &ss,
            &mut tc,
        );
    }
}

fn event_entity_to_rentity(
    mut ewdbm: EventReader<DockBufferModify>,
    mut db: ResMut<DockBuffer>,
    qret: Query<(&REntity, &TagList)>,
) {
    for dbm in ewdbm.read() {
        match dbm {
            DockBufferModify::AddSelected(e) => {
                let b = qret.get(*e).unwrap();
                db.inspection
                    .selected_list
                    .push((b.0.clone(), b.1.clone(), HashSet::new()));
            }
            DockBufferModify::RemoveSelected(e) => {
                let b = qret.get(*e).unwrap();
                let i = db
                    .inspection
                    .selected_list
                    .iter()
                    .position(|x| &x.0 == b.0)
                    .unwrap();
                db.inspection.selected_list.remove(i);
            }
            DockBufferModify::AddTag(rentity, tag) => {
                for i in &mut db.inspection.selected_list {
                    if &i.0 == rentity {
                        i.1.taglist.insert(tag.clone());
                    }
                }
            }
            DockBufferModify::RemoveTag(rentity, tag) => {
                for i in &mut db.inspection.selected_list {
                    if &i.0 == rentity {
                        i.1.taglist.remove(tag);
                    }
                }
            }
            DockBufferModify::RemoveAllTags(rentity) => {
                for i in &mut db.inspection.selected_list {
                    if &i.0 == rentity {
                        i.1.taglist.clear();
                    }
                }
            }
        }
    }
}

// fn update_inspector_buffer(mut era: EventReader<Act>, mut a: ResMut<InspectionBuffer>) {
//     for act in era.read() {
//         match act {
//             Act::ModifyTag(re, tm) => {
//                 let mut ret = es
//                     .iter_mut()
//                     .find(|x| x.0 == re)
//                     .expect("REntity in selection doesn't exist in world.");

//                 match tm {
//                     TagModify::Add(sp) => ret.1.taglist.insert(sp.clone()),
//                     TagModify::Remove(sp) => ret.1.taglist.remove(sp),
//                     TagModify::RemoveAll => {
//                         ret.1.taglist.clear();
//                         true
//                     }
//                 };
//             }
//             _ => (),
//         }
//     }
// }
