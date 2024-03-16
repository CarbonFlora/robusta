use self::{
    leaves::taglist::view_taglist,
    plugins::{
        construction::ConstructType,
        tag::{Tag, TagFlags, Tags},
    },
};

use super::*;

type LoadedFiles = HashMap<Option<String>, InterchangeFormat>;
/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct UiState {
    pub cad_state: CADState,
    pub loaded_files: LoadedFiles,
    pub dock_state: DockState<EguiWindow>,
}

#[derive(Debug, Default, Resource)]
pub struct DockBuffer {
    history: (Act, String),
    pub selected: Vec<(REntity, Tags)>,
    pub nth_n: String,
}

impl DockBuffer {
    pub fn new() -> Self {
        DockBuffer {
            history: (Act::None, String::new()),
            selected: Vec::new(),
            nth_n: String::new(),
        }
    }
}

/// This is all available tabs to be accessed.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EguiWindow {
    Empty,
    Points,
    Inspect,
    History,
    StateRibbon,
    Taglist,
}

#[derive(Debug, Default)]
pub struct CADState {
    // pub object_snapping: SnapSettings,
    pub mode: Mode,
    pub cad_term: Option<String>,
    pub insert_menu: Option<Option<ConstructType>>,
    pub snap_menu: Option<Option<SnapType>>,
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

#[derive(Debug, Default, Resource)]
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

    pub fn flip_nth(&mut self, div: &Option<usize>) {
        flip(&mut self.nthpoint.0);
        if let Some(a) = div {
            if a > &0usize {
                self.nthpoint.1 = *a;
                if !self.nthpoint.0 {
                    flip(&mut self.nthpoint.0);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.endpoint = false;
        self.midpoint = false;
        self.nthpoint = (false, self.nthpoint.1);
        self.intersection = false;
        self.perpendicular = false;
        self.tangent = false;
    }
}

pub fn flip(boolean: &mut bool) {
    *boolean = !(*boolean);
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SnapType {
    Endpoint,
    Midpoint,
    Nthpoint(Option<usize>),
    Intersection,
    Perpendicular,
    Tangent,
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Typing,
    Insert,
    Snap,
}

impl UiState {
    pub fn new(path: &Option<String>) -> Self {
        Self {
            cad_state: CADState::new(),
            loaded_files: load_files(path),
            dock_state: ribbon_cadpanel(),
        }
    }

    pub fn ui(
        &mut self,
        ctx: &mut egui::Context,
        act_write: EventWriter<Act>,
        dock_buffer: &DockBuffer,
        ss: &SnapSettings,
        tc: &TagCharacteristics,
    ) {
        let mut tab_viewer = TabViewer {
            act_write,
            cad_state: &self.cad_state,
            db: dock_buffer,
            ss,
            tc,
        };
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }

    pub fn new_focus(&mut self, ew: &EguiWindow) {
        if let Some(b) = self.dock_state.find_tab(ew) {
            self.dock_state.set_active_tab(b);
        } else {
            self.dock_state.add_window(vec![ew.clone()]);
        }
    }

    pub fn close_all(
        &mut self,
        co: &mut Commands,
        ewp: &Query<Entity, With<RPhantomPointer>>,
        ewrsp: &mut EventWriter<UpdateSnapPoints>,
        rmcb: &mut ResMut<ConstructionBuffer>,
        fs: &mut ResMut<PhantomSnaps>,
    ) {
        ewrsp.send(UpdateSnapPoints(false));
        rmcb.as_mut().reset();
        self.cad_state.cad_term = None;
        self.cad_state.insert_menu = None;
        self.cad_state.snap_menu = None;
        self.cad_state.mode = Mode::Normal;
        despawn_all_phantoms(co, ewp, fs);
    }

    pub fn push_history(&mut self, act: &Act, db: &mut ResMut<DockBuffer>) {
        let (latest, history) = &mut db.history;
        let mut meta_data = String::new();

        if act == latest {
            return;
        }

        history.push_str(match act {
            Act::None => return,
            Act::Exit => "Cleaning up.",
            Act::QuitWithoutSaving => "Quit without saving.",
            Act::DeselectAll => "Deselecting everything.",
            Act::Confirm => "Action confirmed.",
            Act::OpenCADTerm => "Terminal opened.",
            Act::TryAct(a) => {
                meta_data = format!("{a:?}");
                "Terminal submitted: "
            }
            Act::ToggleSnap(a) => match a {
                Some(b) => {
                    meta_data = format!("{b}");
                    "Snap configuration changed: "
                }
                None => "Turned off all snaps.",
            },
            Act::EguiFocus(a) => {
                meta_data = format!("{a:?}");
                "Focusing tab: "
            }

            Act::Insert(a) => match a {
                Some(b) => {
                    meta_data = format!("{b}");
                    "Insert: "
                }
                None => "Opened insert menu.",
            },
            Act::PullCameraFocus(_) => "Camera moved.",
            Act::FitView => "Fit view to all entities.",
            Act::MoveCamera(_) => return,
            Act::ZoomCamera(_) => return,
            Act::ModifyTag(_a, b) => {
                meta_data = format!("{b}");
                "Tag modification: "
            }
        });
        history.push_str(&meta_data);
        history.push('\n');

        db.history.0 = act.clone();
    }
}

fn ribbon_cadpanel() -> DockState<EguiWindow> {
    let mut state = DockState::new(vec![EguiWindow::History, EguiWindow::Taglist]);
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
    ss: Res<SnapSettings>,
    qec: Query<&mut EguiContext, With<CADPanel>>,
    qre: Query<(&REntity, &Tags), With<Selected>>,
    mut db: ResMut<DockBuffer>,
    tc: Res<TagCharacteristics>,
) {
    let mut binding = Vec::new();
    for a in qre.iter() {
        binding.push((a.0.clone(), a.1.clone()));
    }
    db.selected = binding;

    if let Ok(mut w) = qec.get_single().cloned() {
        ui_state.ui(w.get_mut(), act_write, &db, &ss, &tc);
    }
}

/// This is a [`egui_dock`] implimentation. This also directly shows all the available tabs.
struct TabViewer<'a> {
    act_write: EventWriter<'a, Act>,
    cad_state: &'a CADState,
    ss: &'a SnapSettings,
    db: &'a DockBuffer,
    tc: &'a TagCharacteristics,
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
            EguiWindow::Inspect => view_inspection(ui, &self.db.selected, &mut self.act_write),
            EguiWindow::StateRibbon => view_stateribbon(ui, self.cad_state, self.ss),
            EguiWindow::Taglist => view_taglist(ui, &mut self.act_write, self.tc),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, _window: &Self::Tab) -> bool {
        true
    }
}

fn view_stateribbon(ui: &mut egui::Ui, cad_state: &CADState, ss: &SnapSettings) {
    ui.label(format!("{:?}", cad_state.mode));
    ui.label(format!("{:?}", ss));
}
