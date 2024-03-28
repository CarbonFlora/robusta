use self::{
    leaves::{insert::update_insert_egui, snap::update_snap_egui},
    phantom::PhantomAct,
};

use super::*;

pub struct CameraUIPlugin;
impl bevy::app::Plugin for CameraUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraUIBuffer::new())
            .add_systems(Update, update_camera_ui);
    }
}

#[derive(Debug, Default, Resource, Event, Clone)]
pub struct CameraUIBuffer {
    pub menu: CameraUiMenu,
}

impl CameraUIBuffer {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum CameraUiMenu {
    #[default]
    NoMenu,
    CadTerm(String),
    InsertMenu(Option<ConstructType>),
    SnapMenu((Option<SnapType>, String)),
}

impl CameraUiMenu {
    pub fn new() -> Self {
        CameraUiMenu::default()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_camera_ui(
    //Input
    mut rmcb: ResMut<CameraUIBuffer>,
    //Output
    mut ewa: EventWriter<Act>,
    mut ss: ResMut<SnapSettings>,
    mut context: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    match &mut rmcb.menu {
        CameraUiMenu::NoMenu => (),
        CameraUiMenu::CadTerm(buffer) => update_terminal_egui(&mut ewa, buffer, &mut context),
        CameraUiMenu::InsertMenu(_buffer) => update_insert_egui(&mut ewa, &mut context),
        CameraUiMenu::SnapMenu(buffer) => update_snap_egui(&mut ewa, &mut ss, buffer, &mut context),
    }
}

fn close_all(
    ewa: &mut EventWriter<Act>,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
    rmcb: &mut ResMut<ConstructionBuffer>,
    ewpa: &mut EventWriter<PhantomAct>,
) {
    ewa.send(Act::CameraUIMenu(CameraUiMenu::NoMenu)); //cameraui plugin
    ewrsp.send(UpdateSnapPoints(false)); //snap plugin
    rmcb.as_mut().reset(); //construction plugin
    ewpa.send(PhantomAct::DespawnAll); //phantom plugin
}

impl std::fmt::Display for CameraUiMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            CameraUiMenu::NoMenu => "Exited menu.",
            CameraUiMenu::CadTerm(_) => "Opened CADTerm.",
            CameraUiMenu::InsertMenu(_) => "Opened Insert menu.",
            CameraUiMenu::SnapMenu(_) => "Opened Snap menu.",
        };
        f.write_fmt(format_args!("{}", a))
    }
}
