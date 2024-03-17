use self::leaves::{history::HistoryBuffer, inspection::InspectionBuffer};

use super::*;

pub struct DockPlugin;
impl bevy::app::Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DockBuffer::new())
            .add_event::<DockBufferModify>()
            .add_systems(Startup, spawn_window)
            .add_systems(Update, update_dock_buffer)
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

fn update_dock(
    act_write: EventWriter<Act>,
    mut ui_state: ResMut<UiState>,
    ss: Res<SnapSettings>,
    qec: Query<&mut EguiContext, With<CADPanel>>,
    mut db: ResMut<DockBuffer>,
    mut tc: ResMut<TagCharacteristics>,
) {
    if let Ok(mut w) = qec.get_single().cloned() {
        ui_state.ui(w.get_mut(), act_write, &mut db, &ss, &mut tc);
    }
}

fn update_dock_buffer(
    mut ewdbm: EventReader<DockBufferModify>,
    mut db: ResMut<DockBuffer>,
    qret: Query<(&REntity, &Tags)>,
) {
    for dbm in ewdbm.read() {
        match dbm {
            DockBufferModify::AddSelected(e) => {
                let b = qret.get(*e).unwrap();
                db.inspection.selected.push((b.0.clone(), b.1.clone()));
            }
            DockBufferModify::RemoveSelected(e) => {
                let b = qret.get(*e).unwrap();
                let i = db
                    .inspection
                    .selected
                    .iter()
                    .position(|x| &x.0 == b.0)
                    .unwrap();
                db.inspection.selected.remove(i);
            }
        }
    }
}
