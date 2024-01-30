use bevy::utils::{HashMap, Uuid};
use robusta_dxf::wrapper::DXFWrapper;

use crate::*;

use crate::leaves::points::view_points;

#[derive(Eq, PartialEq)]
pub enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, UntypedAssetId),
}

/// This is all available tabs to be accessed.
#[derive(Debug, PartialEq, Eq)]
pub enum EguiWindow {
    CADView(ViewportState),
    Hierarchy,
    Resources,
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

type LoadedFiles = HashMap<Option<String>, DXFWrapper>;
/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct UiState {
    pub loaded_files: LoadedFiles,
    pub state: DockState<EguiWindow>,
    // pub viewport_rectangles: Vec<egui::Rect>,
    // pub viewport_rectangles: HashMap<Uuid, Viewport>,
    // selected_entities: SelectedEntities,
    pub selection: InspectorSelection,
}

impl UiState {
    /// This is currently the default gui layout.
    /// Future Features:
    /// - custom default layout in config.toml
    // pub fn new(cameras: Query<&mut Camera, With<ViewportCamera>>) -> Self {
    pub fn new(path: &Option<String>) -> Self {
        Self {
            loaded_files: load_files(path),
            state: default_dockstate(path),
            selection: InspectorSelection::Entities,
            // viewport_rectangles: HashMap::new(),
        }
    }

    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            loaded_files: &mut self.loaded_files,
            // viewport_rect: &mut self.viewport_rectangles,
            // selected_entities: &mut self.selected_entities,
            // selection: &mut self.selection,
        };
        DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

fn default_dockstate(path: &Option<String>) -> DockState<EguiWindow> {
    let mut state = DockState::new(vec![EguiWindow::CADView(ViewportState::new(path))]);
    let tree = state.main_surface_mut();
    let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspector]);
    let [game, _hierarchy] = tree.split_left(
        game,
        0.2,
        vec![
            EguiWindow::Hierarchy,
            EguiWindow::CADView(ViewportState::new(path)),
        ],
    );
    let [_game, _bottom] =
        tree.split_below(game, 0.8, vec![EguiWindow::Resources, EguiWindow::Points]);
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

pub fn cad_panel(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<CADPanel>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}

pub fn ui_system_update(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}

/// Each viewport should have their own respective camera.
#[derive(Component)]
pub struct ViewportCamera {
    pub id: bevy::utils::Uuid,
}

impl ViewportCamera {
    pub fn new(viewport_id: Uuid) -> Self {
        ViewportCamera { id: viewport_id }
    }
}

/// Turn off panning and zooming [`bevy_pancam`] when interacting with [`egui`].
pub fn unfreeze_camera_viewport(
    mut ui_state: ResMut<UiState>,
    mut cameras: Query<&mut bevy_pancam::PanCam>,
) {
    let focused_tab = ui_state.state.find_active_focused();
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
    world: &'a mut World,
    loaded_files: &'a LoadedFiles,
    // selected_entities: &'a mut SelectedEntities,
    // selection: &'a mut InspectorSelection,
    // viewport_rect: &'a mut egui::Rect,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        // let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        // let type_registry = type_registry.read();

        match window {
            EguiWindow::CADView(_) => (),
            EguiWindow::Hierarchy => (),
            EguiWindow::Resources => (),
            EguiWindow::Points => view_points(ui, self.loaded_files),
            EguiWindow::Inspector => (),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::CADView(_))
    }
}
