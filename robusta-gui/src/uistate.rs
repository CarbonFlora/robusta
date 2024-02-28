use self::{
    phantom::{despawn_all_phantoms, PhantomPoint},
    rselection::Selected,
    snap::UpdateSnapPoints,
};
use crate::leaves::history::view_history;
use bevy::utils::hashbrown::HashMap;
use dxf::Drawing;
use std::path::PathBuf;

use super::*;

type LoadedFiles = HashMap<Option<String>, InterchangeFormat>;
/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct UiState {
    pub cad_state: CADState,
    pub loaded_files: LoadedFiles,
    pub dock_state: DockState<EguiWindow>,
    pub dock_buffer: DockBuffer,
}

#[derive(Debug, Default)]
pub struct DockBuffer {
    history: (Act, String),
    pub selected: Vec<REntity>,
}

impl DockBuffer {
    pub fn new() -> Self {
        DockBuffer {
            history: (Act::None, String::new()),
            selected: Vec::new(),
        }
    }
}

/// This is all available tabs to be accessed.
#[derive(Debug, PartialEq, Eq)]
pub enum EguiWindow {
    Empty,
    Points,
    Inspect,
    History,
    StateRibbon,
}

#[derive(Debug, Default)]
pub struct CADState {
    pub object_snapping: SnapSettings,
    pub mode: Mode,
    pub cad_term: Option<String>,
}

impl CADState {
    fn new() -> Self {
        CADState::default()
    }
}

#[derive(Resource, Default)]
pub struct TopZLayer(usize);

impl TopZLayer {
    pub fn new() -> Self {
        TopZLayer(0usize)
    }

    pub fn top(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

#[derive(Debug, Default)]
pub struct SnapSettings {
    pub endpoint: bool,
    pub midpoint: bool,
    pub nthpoint: (bool, usize),
    pub intersection: bool,
    pub perpendicular: bool,
    pub tangent: bool,
}

impl SnapSettings {
    pub fn any(&self) -> bool {
        self.endpoint
            || self.midpoint
            || self.nthpoint.0
            || self.intersection
            || self.perpendicular
            || self.tangent
    }

    pub fn flip_nth(&mut self, div: &usize) {
        flip(&mut self.nthpoint.0);
        if div > &0usize {
            self.nthpoint.1 = *div;
            if !self.nthpoint.0 {
                flip(&mut self.nthpoint.0);
            }
        }
    }
}

pub fn flip(boolean: &mut bool) {
    *boolean = !(*boolean);
}

#[derive(Debug, Clone, PartialEq)]
pub enum Snaps {
    Endpoint,
    Midpoint,
    Nthpoint(usize),
    Intersection,
    Perpendicular,
    Tangent,
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Typing,
}

impl UiState {
    pub fn new(path: &Option<String>) -> Self {
        Self {
            cad_state: CADState::new(),
            loaded_files: load_files(path),
            dock_state: ribbon_cadpanel(),
            dock_buffer: DockBuffer::new(),
        }
    }

    pub fn ui(&mut self, ctx: &mut egui::Context, act_write: EventWriter<Act>) {
        let mut tab_viewer = TabViewer {
            act_write,
            cad_state: &self.cad_state,
            dock_buffer: &self.dock_buffer,
        };
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }

    pub fn inspect(&mut self) {
        if let Some(b) = self.dock_state.find_tab(&EguiWindow::Inspect) {
            self.dock_state.set_active_tab(b);
        } else {
            self.dock_state.add_window(vec![EguiWindow::Inspect]);
        }
    }

    pub fn close_all(
        &mut self,
        co: &mut Commands,
        ewp: &Query<Entity, With<PhantomPoint>>,
        ewrsp: &mut EventWriter<UpdateSnapPoints>,
    ) {
        ewrsp.send(UpdateSnapPoints(false));
        self.cad_state.cad_term = None;
        self.cad_state.mode = Mode::Normal;
        despawn_all_phantoms(co, ewp);
    }

    pub fn push_history(&mut self, act: &Act) {
        let (latest, history) = &mut self.dock_buffer.history;
        let mut meta_data = String::new();

        if act == latest {
            return;
        }

        history.push_str(match act {
            Act::None => return,
            Act::Exit => "Cleaning up.",
            Act::QuitWithoutSaving => "Quit without saving.",
            Act::DeselectAll => "Deselecting everything.",
            Act::Confirm => "Action Confirmed.",
            Act::OpenCADTerm => "Terminal opened.",
            Act::TryAct(a) => {
                meta_data = format!("{a:?}");
                "Terminal submitted: "
            }
            Act::NewPoint => "Point created.",
            Act::ToggleSnap(a) => {
                meta_data = format!("{a:?}");
                "Snap configuration changed: "
            }
            Act::ToggleSnapOff => "All object snaps turned off.",
            Act::Inspect => "Inspecting.",
            Act::PullCameraFocus(_) => "Camera moved.",
            Act::FitView => "Fit view to all entities.",
            Act::MoveCamera(_) => "Camera moved.",
            Act::ZoomCamera(_) => "Camera zoomed.",
        });
        history.push_str(&meta_data);
        history.push('\n');

        self.dock_buffer.history.0 = act.clone();
    }
}

fn ribbon_cadpanel() -> DockState<EguiWindow> {
    let mut state = DockState::new(vec![EguiWindow::History]);
    let tree = state.main_surface_mut();
    let [old, _new] = tree.split_above(NodeIndex::root(), 0.1, vec![EguiWindow::StateRibbon]);
    let [_old, _new] = tree.split_left(old, 0.22, vec![EguiWindow::Inspect, EguiWindow::Points]);

    state
}

fn _debug_cadpanel() -> DockState<EguiWindow> {
    let mut state = DockState::new(vec![EguiWindow::Empty]);
    let tree = state.main_surface_mut();
    let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspect]);
    let [game, _points] = tree.split_left(game, 0.2, vec![EguiWindow::Points]);
    let [_game, _bottom] = tree.split_below(game, 0.8, vec![EguiWindow::Empty]);

    state
}

fn load_files(path: &Option<String>) -> HashMap<Option<String>, InterchangeFormat> {
    let loaded_file = parse_dxf(path);
    let mut loaded_files = HashMap::new();
    loaded_files.insert(path.clone(), InterchangeFormat::DXF(loaded_file));
    loaded_files.insert(None, InterchangeFormat::DXF(new_dxf()));

    loaded_files
}

pub fn open_from_path(path: PathBuf) -> Drawing {
    let drawing = Drawing::load_file(path);
    match drawing {
        Ok(d) => d,
        Err(_e) => Drawing::new(),
    }
}

pub fn parse_dxf(path: &Option<String>) -> Drawing {
    open_from_path(path.clone().unwrap_or_default().into())
}

pub fn new_dxf() -> Drawing {
    Drawing::new()
}

pub enum InterchangeFormat {
    DXF(Drawing),
}

#[derive(Component, Default)]
pub struct CADPanel {}

pub fn update_dock(
    act_write: EventWriter<Act>,
    mut ui_state: ResMut<UiState>,
    qec: Query<&mut EguiContext, With<CADPanel>>,
    qre: Query<&REntity, With<Selected>>,
) {
    ui_state.dock_buffer.selected = qre.iter().cloned().collect::<Vec<REntity>>();

    if let Ok(mut w) = qec.get_single().cloned() {
        ui_state.ui(w.get_mut(), act_write);
    }
}

/// This is a [`egui_dock`] implimentation. This also directly shows all the available tabs.
struct TabViewer<'a> {
    act_write: EventWriter<'a, Act>,
    cad_state: &'a CADState,
    dock_buffer: &'a DockBuffer,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        // let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        // let type_registry = type_registry.read();

        match window {
            EguiWindow::Empty => (),
            EguiWindow::History => view_history(ui, &self.dock_buffer.history),
            EguiWindow::Points => (),
            EguiWindow::Inspect => {
                view_inspection(ui, &self.dock_buffer.selected, &mut self.act_write)
            }
            EguiWindow::StateRibbon => view_stateribbon(ui, self.cad_state),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, _window: &Self::Tab) -> bool {
        true
    }
}

fn view_stateribbon(ui: &mut egui::Ui, cad_state: &CADState) {
    ui.label(format!("{:?}", cad_state.mode));
    ui.label(format!("{:?}", cad_state.object_snapping));
}
// Each viewport should have their own respective camera.
// #[derive(Component)]
// pub struct ViewportCamera {
//     pub id: bevy::utils::Uuid,
// }

// impl ViewportCamera {
//     pub fn new(viewport_id: Uuid) -> Self {
//         ViewportCamera { id: viewport_id }
//     }
// }

// Turn off panning and zooming [`bevy_pancam`] when interacting with [`egui`].
// pub fn unfreeze_camera_viewport(
//     mut ui_state: ResMut<UiState>,
//     mut cameras: Query<&mut bevy_pancam::PanCam>,
// ) {
//     let focused_tab = ui_state.dock_state.find_active_focused();
//     match focused_tab {
//         None => (),
//         Some(tab) => cameras.for_each_mut(|mut x| {
//             x.enabled = match tab.1 {
//                 EguiWindow::CADView(_) => true,
//                 _ => false,
//             }
//         }),
//     }
// }

// /// This is the `Bevy` resource containing all the custom GUI elements.
// #[derive(Resource, Debug, PartialEq, Eq)]
// pub struct ViewportState {
//     pub viewport_id: Uuid,
//     pub opened_file_path: Option<String>,
//     // pub points: Vec<robusta_core::point::Point>,
// }

// impl ViewportState {
//     pub fn new(path: &Option<String>) -> Self {
//         ViewportState {
//             viewport_id: Uuid::new_v4(),
//             opened_file_path: path.clone(),
//             // points: loaded_file.points,
//         }
//     }
// }
