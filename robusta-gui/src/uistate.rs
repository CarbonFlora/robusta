use crate::*;

use crate::leaves::asset::select_asset;
use crate::leaves::resource::select_resource;

pub fn pancam_setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(bevy_pancam::PanCam {
            zoom_to_cursor: false,
            ..default()
        });
}

#[derive(Eq, PartialEq)]
pub enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, UntypedAssetId),
}

/// This is all available tabs to be accessed.
#[derive(Debug)]
pub enum EguiWindow {
    CADView,
    Hierarchy,
    Resources,
    Assets,
    Inspector,
}

/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct UiState {
    pub state: DockState<EguiWindow>,
    pub viewport_rect: egui::Rect,
    // selected_entities: SelectedEntities,
    pub selection: InspectorSelection,
}

impl UiState {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![EguiWindow::CADView]);
        let tree = state.main_surface_mut();
        let [game, _inspector] =
            tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspector]);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![EguiWindow::Hierarchy]);
        let [_game, _bottom] =
            tree.split_below(game, 0.8, vec![EguiWindow::Resources, EguiWindow::Assets]);

        Self {
            state,
            // selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            viewport_rect: egui::Rect::NOTHING,
        }
    }

    /// This currently creates a new DockArea on every `Update` cycle. This pulls data from `TabViewer` and transfers it over to `UiState`.
    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            // selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
        };
        DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

pub fn show_ui_system(world: &mut World) {
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

// make camera only render to view not obstructed by UI
pub fn set_camera_viewport(
    ui_state: Res<UiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<bevy_pancam::PanCam>>,
) {
    let mut cam = cameras.single_mut();
    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor as f32;

    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        depth: 0.0..1.0,
    });
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
                EguiWindow::CADView => true,
                _ => false,
            }
        }),
    }
}

/// This is a [`egui_dock`] implimentation. This also directly shows all the available tabs.
struct TabViewer<'a> {
    world: &'a mut World,
    // selected_entities: &'a mut SelectedEntities,
    selection: &'a mut InspectorSelection,
    viewport_rect: &'a mut egui::Rect,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match window {
            EguiWindow::CADView => {
                *self.viewport_rect = ui.clip_rect();

                // draw_gizmo(ui, self.world, self.selected_entities, self.gizmo_mode);
            }
            EguiWindow::Hierarchy => (),
            EguiWindow::Resources => select_resource(ui, &type_registry, self.selection),
            EguiWindow::Assets => select_asset(ui, &type_registry, self.world, self.selection),
            EguiWindow::Inspector => (),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::CADView)
    }
}
