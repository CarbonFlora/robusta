use bevy::utils::{HashMap, Uuid};
use bevy_mod_picking::prelude::*;
use robusta_dxf::wrapper::DXFWrapper;

use crate::*;

use crate::keystrokes::Actions;
use crate::leaves::inspection::view_inspection;
use crate::leaves::keystrokes::view_pressed_keys;
use crate::leaves::points::view_points;

use self::leaves::term::open_term_egui;

// #[derive(Eq, PartialEq)]
// pub enum InspectorSelection {
//     Entities,
//     Resource(TypeId, String),
//     Asset(TypeId, String, UntypedAssetId),
// }

/// This is all available tabs to be accessed.
#[derive(Debug, PartialEq, Eq)]
pub enum EguiWindow {
    CADView(ViewportState),
    Hierarchy,
    Debug,
    Points,
    Inspector,
}

/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource, Debug, PartialEq, Eq)]
pub struct ViewportState {
    pub viewport_id: Uuid,
    pub opened_file_path: Option<String>,
    // pub points: Vec<robusta_core::point::Point>,
}

impl ViewportState {
    pub fn new(path: &Option<String>) -> Self {
        ViewportState {
            viewport_id: Uuid::new_v4(),
            opened_file_path: path.clone(),
            // points: loaded_file.points,
        }
    }
}

#[derive(Event, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelectionInstance(Entity, SelectionAddRemove);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionAddRemove {
    Add,
    Remove,
}

impl From<ListenerInput<Pointer<Select>>> for SelectionInstance {
    fn from(event: ListenerInput<Pointer<Select>>) -> Self {
        SelectionInstance(event.target, SelectionAddRemove::Add)
    }
}

impl From<ListenerInput<Pointer<Deselect>>> for SelectionInstance {
    fn from(event: ListenerInput<Pointer<Deselect>>) -> Self {
        SelectionInstance(event.target, SelectionAddRemove::Remove)
    }
}

// impl DoSomethingComplex {
//     fn new() -> Self {
//         return DoSomethingComplex(Entity::PLACEHOLDER, 0.0);
//     }
// }

/// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
// fn receive_greetings(mut greetings: EventReader<DoSomethingComplex>) {
//     for event in greetings.read() {
//         info!(
//             "Hello {:?}, you are {:?} depth units away from the pointer",
//             event.0, event.1
//         );
//     }
// }

type LoadedFiles = HashMap<Option<String>, DXFWrapper>;
/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct UiState {
    pub pressed_keys: [Option<KeyCode>; 2], //this is mainly for debug.
    pub actions: Actions,
    pub cad_state: CADState,
    pub loaded_files: LoadedFiles,
    pub dock_state: DockState<EguiWindow>,
    // pub viewport_rectangles: Vec<egui::Rect>,
    // pub viewport_rectangles: HashMap<Uuid, Viewport>,
    pub selected_entities: Vec<SelectionInstance>,
    // pub selection: InspectorSelection,
}

#[derive(Debug, Default)]
pub struct CADState {
    pub cad_term: (bool, String),
}

impl CADState {
    fn new() -> Self {
        return CADState::default();
    }

    fn close_all(&mut self) {
        self.cad_term = (false, String::new());
    }
}

impl UiState {
    /// This is currently the default gui layout.
    /// Future Features:
    /// - custom default layout in config.toml
    // pub fn new(cameras: Query<&mut Camera, With<ViewportCamera>>) -> Self {
    pub fn new(path: &Option<String>) -> Self {
        Self {
            pressed_keys: [None; 2],
            actions: Actions::None,
            cad_state: CADState::new(),
            loaded_files: load_files(path),
            dock_state: default_cadpanel(path),
            selected_entities: Vec::new(),
        }
    }

    pub fn ui(&mut self, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            loaded_files: &mut self.loaded_files,
            pressed_keys: &self.pressed_keys,
            actions: &self.actions,
            selected_entities: &mut self.selected_entities,
        };
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

fn default_cadpanel(path: &Option<String>) -> DockState<EguiWindow> {
    let mut state = DockState::new(vec![EguiWindow::CADView(ViewportState::new(path))]);
    let tree = state.main_surface_mut();
    let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspector]);
    let [game, _points] = tree.split_left(game, 0.2, vec![EguiWindow::Points]);
    let [_game, _bottom] = tree.split_below(game, 0.8, vec![EguiWindow::Debug]);

    return state;
}

fn load_files(path: &Option<String>) -> LoadedFiles {
    let loaded_file = robusta_dxf::open::parse_dxf(path);
    let mut loaded_files = HashMap::new();
    loaded_files.insert(path.clone(), loaded_file);
    return loaded_files;
}

#[derive(Component, Default)]
pub struct CADPanel {}

// pub fn cad_panel(world: &mut World) {
//     let Ok(egui_context) = world
//         .query_filtered::<&mut EguiContext, With<CADPanel>>()
//         .get_single(world)
//     else {
//         return;
//     };
//     let mut egui_context = egui_context.clone();

//     world.resource_scope::<UiState, _>(|_world, mut ui_state| ui_state.ui(egui_context.get_mut()));
// }

pub fn update_dock(
    mut ui_state: ResMut<UiState>,
    egui_context_cadpanel: Query<&mut EguiContext, With<CADPanel>>,
    // egui_context_primary: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut greetings: EventReader<SelectionInstance>,
) {
    let buf = greetings
        .read()
        .map(|x| x.clone())
        .collect::<Vec<SelectionInstance>>();

    for i in buf {
        match i.1 {
            SelectionAddRemove::Add => ui_state.selected_entities.push(i),
            SelectionAddRemove::Remove => ui_state.selected_entities.retain(|x| x.0 != i.0),
        };
    }
    if let Ok(mut w) = egui_context_cadpanel.get_single().cloned() {
        ui_state.ui(w.get_mut());
    }
}

pub fn update_cad_ui(
    mut ui_state: ResMut<UiState>,
    egui_context_primary: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    match ui_state.actions {
        Actions::OpenCADTerm => ui_state.cad_state.cad_term = (true, String::new()),
        Actions::Exit => ui_state.cad_state.close_all(),
        _ => (),
    }

    if ui_state.cad_state.cad_term.0 {
        open_term_egui(&mut ui_state, egui_context_primary);
    }
}

/// Each viewport should have their own respective camera.
// #[derive(Component)]
// pub struct ViewportCamera {
//     pub id: bevy::utils::Uuid,
// }

// impl ViewportCamera {
//     pub fn new(viewport_id: Uuid) -> Self {
//         ViewportCamera { id: viewport_id }
//     }
// }

/// Turn off panning and zooming [`bevy_pancam`] when interacting with [`egui`].
pub fn unfreeze_camera_viewport(
    mut ui_state: ResMut<UiState>,
    mut cameras: Query<&mut bevy_pancam::PanCam>,
) {
    let focused_tab = ui_state.dock_state.find_active_focused();
    match focused_tab {
        None => (),
        Some(tab) => cameras.for_each_mut(|mut x| {
            x.enabled = match tab.1 {
                EguiWindow::CADView(_) => true,
                _ => false,
            }
        }),
    }
}

/// This is a [`egui_dock`] implimentation. This also directly shows all the available tabs.
struct TabViewer<'a> {
    loaded_files: &'a LoadedFiles,
    pressed_keys: &'a [Option<KeyCode>; 2],
    actions: &'a Actions,
    selected_entities: &'a mut Vec<SelectionInstance>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        // let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        // let type_registry = type_registry.read();

        match window {
            EguiWindow::CADView(_) => (),
            EguiWindow::Hierarchy => (),
            EguiWindow::Debug => view_pressed_keys(ui, self.pressed_keys, self.actions),
            EguiWindow::Points => view_points(ui, self.loaded_files),
            EguiWindow::Inspector => view_inspection(ui, self.selected_entities),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, _window: &Self::Tab) -> bool {
        true
    }
}
