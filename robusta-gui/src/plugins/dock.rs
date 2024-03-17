use bevy::utils::hashbrown::HashSet;

use super::*;

pub struct DockPlugin;
impl bevy::app::Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DockBuffer::new())
            // .add_event::<EguiUpdate>()
            .add_systems(Startup, spawn_window)
            // .add_systems(PreUpdate, update_dock_buffer)
            .add_systems(Update, update_dock);
    }
}

#[derive(Debug, Default, Resource)]
pub struct DockBuffer {
    pub history: (Act, String),
    pub selected: Vec<(REntity, Tags)>,
    pub nth_n: String,
    pub egui_selection: HashMap<usize, Tag>,
    pub is_selection_mode: bool,
    pub editing_tag: HashSet<Tag>,
    pub temporary_name: String,
}

impl DockBuffer {
    pub fn new() -> Self {
        Self::default()
    }
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

// pub fn update_dock_buffer(mut ertleu: EventReader<EguiUpdate>, mut db: ResMut<DockBuffer>) {
//     for act in ertleu.read() {
//         match act {
//             EguiUpdate::ToggleRowSelection(row_index) => {
//                 match db.egui_selection.contains_key(&row_index.0) {
//                     true => db.egui_selection.remove(&row_index.0),
//                     false => db.egui_selection.insert(row_index.0, row_index.1.clone()),
//                 };
//             }
//             EguiUpdate::NewColor(color) => db.color = *color,
//         }
//     }
// }
