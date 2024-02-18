use bevy_mod_picking::pointer::Location;
use robusta_core::RobustaEntity;

use super::*;

type LoadedFiles = HashMap<Option<String>, RFile>;
/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct UiState {
    pub cad_state: CADState,
    pub loaded_files: LoadedFiles,
    pub dock_state: DockState<EguiWindow>,
    pub selected_entities: Vec<(SelectionInstance, Option<RobustaEntity>)>,
}

/// This is all available tabs to be accessed.
#[derive(Debug, PartialEq, Eq)]
pub enum EguiWindow {
    CADView,
    Hierarchy,
    Debug,
    Points,
    Inspect,
    History,
}

#[derive(Event, Clone, Debug, PartialEq)]
pub struct SelectionInstance(pub Entity, pub PointerId, pub Location, SelectionAddRemove);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionAddRemove {
    Add,
    Remove,
}

impl From<ListenerInput<Pointer<Select>>> for SelectionInstance {
    fn from(event: ListenerInput<Pointer<Select>>) -> Self {
        SelectionInstance(
            event.target,
            event.pointer_id,
            event.pointer_location.clone(),
            SelectionAddRemove::Add,
        )
    }
}

impl From<ListenerInput<Pointer<Deselect>>> for SelectionInstance {
    fn from(event: ListenerInput<Pointer<Deselect>>) -> Self {
        SelectionInstance(
            event.target,
            event.pointer_id,
            event.pointer_location.clone(),
            SelectionAddRemove::Remove,
        )
    }
}

#[derive(Debug, Default)]
pub struct CADState {
    pub construction: Option<(Entity, RobustaEntity)>,
    pub mode: Mode,
    pub cad_term: Option<String>,
}

impl CADState {
    fn new() -> Self {
        CADState::default()
    }

    pub fn close_all(&mut self) {
        // if let Some((a, b)) = self.construction {

        // }
        self.construction = None;
        self.mode = Mode::Normal;
        self.cad_term = None;
    }
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Typing,
}

// #[derive(Event, Clone, Debug, PartialEq)]
// pub enum ReMapSelections {
//     All,
//     One(Entity),
// }

impl UiState {
    /// This is currently the default gui layout.
    /// Future Features:
    /// - custom default layout in config.toml
    // pub fn new(cameras: Query<&mut Camera, With<ViewportCamera>>) -> Self {
    pub fn new(path: &Option<String>) -> Self {
        Self {
            cad_state: CADState::new(),
            loaded_files: load_files(path),
            dock_state: default_cadpanel(),
            selected_entities: Vec::new(),
        }
    }

    pub fn ui(&mut self, ctx: &mut egui::Context, act_write: EventWriter<Act>) {
        let mut tab_viewer = TabViewer {
            loaded_files: &mut self.loaded_files,
            act_write,
            selected_entities: &mut self.selected_entities,
        };
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }

    pub fn deselect_all(&self, deselections: &mut EventWriter<Pointer<Deselect>>) {
        for i in &self.selected_entities {
            deselections.send(Pointer::new(i.0 .1, i.0 .2.clone(), i.0 .0, Deselect))
        }
    }

    pub fn remap_selection(&mut self, entity: &Entity, entity_mapping: &EntityMapping) {
        let a = entity_mapping
            .get(entity)
            .expect("entity_mapping and entity miscommunication.");
        for i in &mut self.selected_entities {
            if i.0 .0 == *entity {
                i.1 = Some(a.clone());
            }
        }
    }

    pub fn inspect(&mut self) {
        if let Some(b) = self.dock_state.find_tab(&EguiWindow::Inspect) {
            self.dock_state.set_active_tab(b);
        } else {
            self.dock_state.add_window(vec![EguiWindow::Inspect]);
        }
    }

    pub fn all_rect(&self) -> Rect {
        let mut a = self.loaded_files.iter().flat_map(|x| x.1.iter_points());

        let (mut min_x, mut min_y, mut max_x, mut max_y) = match a.next() {
            None => (0., 0., 0., 0.),
            Some(point) => (
                point.coordinates.x,
                point.coordinates.y,
                point.coordinates.x,
                point.coordinates.y,
            ),
        };

        for point in a {
            if point.coordinates.x < min_x {
                min_x = point.coordinates.x;
            }
            if point.coordinates.x > max_x {
                max_x = point.coordinates.x;
            }
            if point.coordinates.y < min_y {
                min_y = point.coordinates.y;
            }
            if point.coordinates.y > max_y {
                max_y = point.coordinates.y;
            }
        }

        Rect::new(min_x, min_y, max_x, max_y)
    }

    pub fn new_point(
        &mut self,
        entity_mapping: &mut ResMut<EntityMapping>,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let entity_package = (commands, meshes, materials);
        let id = entity_package
            .0
            .spawn((
                MaterialMesh2dBundle {
                    mesh: entity_package.1.add(shape::Circle::new(0.5).into()).into(),
                    material: entity_package.2.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(
                        0.,
                        0.,
                        entity_mapping.z_layer_add(),
                    )),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Select>>::send_event::<SelectionInstance>(),
                On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
            ))
            .id();

        self.cad_state.construction = Some((
            id,
            RobustaEntity::Point(robusta_core::point::Point::new(0., 0., 0.)),
        ));
    }
}

fn default_cadpanel() -> DockState<EguiWindow> {
    let mut state = DockState::new(vec![EguiWindow::CADView]);
    let tree = state.main_surface_mut();
    let [game, _inspector] = tree.split_right(NodeIndex::root(), 0.75, vec![EguiWindow::Inspect]);
    let [game, _points] = tree.split_left(game, 0.2, vec![EguiWindow::Points]);
    let [_game, _bottom] = tree.split_below(game, 0.8, vec![EguiWindow::Debug]);

    state
}

fn load_files(path: &Option<String>) -> LoadedFiles {
    let loaded_file = robusta_dxf::open::parse_dxf(path);
    let mut loaded_files = HashMap::new();
    loaded_files.insert(path.clone(), loaded_file);

    loaded_files
}

#[derive(Component, Default)]
pub struct CADPanel {}

pub fn update_dock(
    mut act_write: EventWriter<Act>,
    mut ui_state: ResMut<UiState>,
    egui_context_cadpanel: Query<&mut EguiContext, With<CADPanel>>,
    mut greetings: EventReader<SelectionInstance>,
) {
    let buf = greetings
        .read()
        .cloned()
        .collect::<Vec<SelectionInstance>>();

    for i in buf {
        match i.3 {
            SelectionAddRemove::Add => {
                ui_state.selected_entities.push((i.clone(), None));
                act_write.send(Act::DebugReMapSelection(i.0));
            }
            SelectionAddRemove::Remove => ui_state.selected_entities.retain(|x| x.0 .0 != i.0),
        };
    }
    if let Ok(mut w) = egui_context_cadpanel.get_single().cloned() {
        ui_state.ui(w.get_mut(), act_write);
    }
}

/// This is a [`egui_dock`] implimentation. This also directly shows all the available tabs.
struct TabViewer<'a> {
    loaded_files: &'a LoadedFiles,
    act_write: EventWriter<'a, Act>,
    selected_entities: &'a mut Vec<(SelectionInstance, Option<RobustaEntity>)>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        // let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        // let type_registry = type_registry.read();

        match window {
            EguiWindow::CADView => (),
            EguiWindow::Hierarchy => (),
            // EguiWindow::Debug => view_pressed_keys(ui, self.pressed_keys, self.acts),
            EguiWindow::Debug => (),
            // EguiWindow::History => view_history(ui, self.acts),
            EguiWindow::History => (),
            EguiWindow::Points => view_points(ui, self.loaded_files),
            EguiWindow::Inspect => view_inspection(ui, self.selected_entities, &mut self.act_write),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, _window: &Self::Tab) -> bool {
        true
    }
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
