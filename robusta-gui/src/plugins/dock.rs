use self::{
    leaves::{
        history::HistoryBuffer,
        inspection::InspectionBuffer,
        taglist::{view_taglist, TaglistBuffer},
    },
    tag::TagFlags,
};

use super::*;

#[derive(Debug, Resource)]
pub struct RDockState(DockState<EguiWindow>);

pub struct DockPlugin;
impl bevy::app::Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DockBuffer::new())
            .insert_resource(RDockState::default_preset())
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
    pub other: OtherBuffer,
}

#[derive(Debug, Resource, Clone, Default)]
pub struct OtherBuffer {
    pub snaps: SnapSettings,
}

impl DockBuffer {
    pub fn new() -> Self {
        Self {
            history: HistoryBuffer::default(),
            inspection: InspectionBuffer::default(),
            taglist: TaglistBuffer::default(),
            other: OtherBuffer::default(),
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
    mut rmrds: ResMut<RDockState>,
    mut qec: Query<&mut EguiContext, With<CADPanel>>,
    //carryover
    mut db: ResMut<DockBuffer>,
    ewa: EventWriter<Act>,
) {
    let mut ctx = match qec.get_single_mut() {
        Ok(w) => w,
        Err(_) => return,
    };

    let db = db.bypass_change_detection();
    DockArea::new(&mut rmrds.0)
        .style(Style::from_egui(ctx.get().style().as_ref()))
        .show(ctx.get_mut(), &mut TabViewer { db, ewa });
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
                        i.1 .0.push(tag.clone());
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
                        i.1 .0.clear();
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
            }
        }
    }
}

#[derive(Component, Default)]
pub struct CADPanel {}

/// This is a [`egui_dock`] implimentation. This also directly shows all the available tabs.
struct TabViewer<'a> {
    db: &'a mut DockBuffer,
    ewa: EventWriter<'a, Act>,
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
            EguiWindow::Inspect => view_inspection(ui, &mut self.db.inspection, &mut self.ewa),
            EguiWindow::StateRibbon => view_stateribbon(ui, &self.db.other),
            EguiWindow::Taglist => view_taglist(ui, &mut self.db.taglist, &mut self.ewa),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, _window: &Self::Tab) -> bool {
        true
    }
}

fn view_stateribbon(ui: &mut egui::Ui, ob: &OtherBuffer) {
    ui.label(format!("{:?}", ob.snaps));
}

impl RDockState {
    pub fn default_preset() -> Self {
        let mut dock_state = DockState::new(vec![EguiWindow::History, EguiWindow::Taglist]);
        let tree = dock_state.main_surface_mut();
        let [old, _new] = tree.split_above(NodeIndex::root(), 0.1, vec![EguiWindow::StateRibbon]);
        let [_old, _new] =
            tree.split_left(old, 0.22, vec![EguiWindow::Inspect, EguiWindow::Points]);

        Self(dock_state)
    }

    pub fn new_focus(&mut self, ew: &EguiWindow) {
        if let Some(b) = self.0.find_tab(ew) {
            self.0.set_active_tab(b);
        } else {
            self.0.add_window(vec![ew.clone()]);
        }
    }
}
