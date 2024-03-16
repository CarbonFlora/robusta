use self::tag::Tag;

use super::*;

pub struct DockPlugin;
impl bevy::app::Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DockBuffer::new())
            .add_systems(Startup, spawn_window)
            .add_systems(PreUpdate, update_dock_buffer)
            .add_systems(Update, update_dock);
    }
}

#[derive(Debug, Default, Resource)]
pub struct DockBuffer {
    pub history: (Act, String),
    pub selected: Vec<(REntity, Tags)>,
    pub nth_n: String,
    pub egui_selection: HashMap<usize, Tag>,
}

impl DockBuffer {
    pub fn new() -> Self {
        DockBuffer {
            history: (Act::None, String::new()),
            selected: Vec::new(),
            nth_n: String::new(),
            egui_selection: HashMap::new(),
        }
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

pub fn update_dock_buffer(mut era: EventReader<Act>, mut db: ResMut<DockBuffer>) {
    for act in era.read() {
        if let Act::ToggleRowSelection(row_index) = act {
            match db.egui_selection.contains_key(&row_index.0) {
                true => db.egui_selection.remove(&row_index.0),
                false => db.egui_selection.insert(row_index.0, row_index.1.clone()),
            };
        }
    }
}
