use self::{
    keystroke::ModalResources,
    leaves::{
        history::HistoryBuffer,
        inspection::InspectionBuffer,
        taglist::{view_taglist, TaglistBuffer},
    },
    tag::TagFlags,
};

use super::*;

pub struct DockPlugin;
impl bevy::app::Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DockBuffer::new())
            .add_event::<DockBufferModify>()
            .add_systems(Startup, spawn_window)
            .add_systems(Update, update_dockbuffer)
            .add_systems(Update, update_dock);
    }
}

#[derive(Debug, Default, Resource)]
pub struct DockBuffer {
    pub history: HistoryBuffer,
    pub inspection: InspectionBuffer,
    pub taglist: TaglistBuffer,
}

impl DockBuffer {
    pub fn new() -> Self {
        Self {
            history: HistoryBuffer::default(),
            inspection: InspectionBuffer::default(),
            taglist: TaglistBuffer::default(),
        }
    }
}

#[derive(Debug, Event)]
pub enum DockBufferModify {
    AddSelected(Entity),
    RemoveSelected(Entity),
    AddTag(REntity, Tag),
    RemoveTag(REntity, Tag),
    RemoveAllTags(REntity),
    TagListAdd(Tag),
    TagListRemove(Tag),
    // TagListFlagUpdate(Tag, Flag),
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
    //Util
    mut ewm: ResMut<ModalResources>,
    mut ui_state: ResMut<UiState>,
    ss: Res<SnapSettings>,
    mut qec: Query<&mut EguiContext, With<CADPanel>>,
    mut db: ResMut<DockBuffer>,
    //Output
    act_write: EventWriter<Act>,
) {
    let ctx = match qec.get_single_mut() {
        Ok(mut w) => w.get_mut(),
        Err(_) => todo!(),
    };

    let mut tab_viewer = TabViewer {
        act_write: act_write,
        ewm,
        db: dock_buffer,
        ss,
    };
    DockArea::new(&mut this.dock_state)
        .style(Style::from_egui(ctx.style().as_ref()))
        .show(ctx, &mut tab_viewer);
}

///This updates the dockbuffer with what is actually true in Resources.
fn update_dockbuffer(
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
                        i.1.taglist.push(tag.clone());
                    }
                }
            }
            DockBufferModify::RemoveTag(rentity, tag) => {
                for i in &mut db.inspection.selected_list {
                    if &i.0 == rentity {
                        i.1.remove_tag(tag);
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
            DockBufferModify::TagListAdd(t) => {
                db.taglist
                    .ordered_tag_flags
                    .push((t.clone(), TagFlags::all_none(), false));
            }
            DockBufferModify::TagListRemove(t) => {
                let i = db
                    .taglist
                    .ordered_tag_flags
                    .iter()
                    .position(|x| &x.0 == t)
                    .unwrap();
                db.taglist.ordered_tag_flags.remove(i);
            } // DockBufferModify::TagListFlagUpdate(t, f) => {
              //     for i in &mut db.taglist.ordered_tag_flags {
              //         if &i.0 == t {
              //             i.1.update(f);
              //         }
              //     }
              // }
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
#[derive(Component, Default)]
pub struct CADPanel {}

/// This is a [`egui_dock`] implimentation. This also directly shows all the available tabs.
struct TabViewer<'a> {
    act_write: EventWriter<'a, Act>,
    ewm: &'a mut ModalResources,
    ss: &'a SnapSettings,
    db: &'a mut DockBuffer,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        // let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        // let type_registry = type_registry.read();

        match window {
            EguiWindow::Empty => (),
            EguiWindow::History => view_history(ui, &self.db.history),
            EguiWindow::Points => (),
            EguiWindow::Inspect => view_inspection(
                ui,
                &mut self.db.inspection,
                self.ewm,
                &mut self.act_write,
                // &mut self.ewdbm,
            ),
            EguiWindow::StateRibbon => view_stateribbon(ui, self.ss),
            EguiWindow::Taglist => {
                view_taglist(ui, &mut self.db.taglist, self.ewm, &mut self.act_write)
            }
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, _window: &Self::Tab) -> bool {
        true
    }
}

fn view_stateribbon(ui: &mut egui::Ui, ss: &SnapSettings) {
    ui.label(format!("{:?}", ss));
}

#[derive(Debug, Resource)]
pub struct RDockState {
    pub dock_state: DockState<EguiWindow>,
}

impl RDockState {
    pub fn default_preset() -> Self {
        let mut dock_state = DockState::new(vec![EguiWindow::History, EguiWindow::Taglist]);
        let tree = dock_state.main_surface_mut();
        let [old, _new] = tree.split_above(NodeIndex::root(), 0.1, vec![EguiWindow::StateRibbon]);
        let [_old, _new] =
            tree.split_left(old, 0.22, vec![EguiWindow::Inspect, EguiWindow::Points]);

        Self { dock_state }
    }

    pub fn new_focus(&mut self, ew: &EguiWindow) {
        if let Some(b) = self.dock_state.find_tab(ew) {
            self.dock_state.set_active_tab(b);
        } else {
            self.dock_state.add_window(vec![ew.clone()]);
        }
    }
}
