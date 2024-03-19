use super::*;

type LoadedFiles = HashMap<Option<String>, InterchangeFormat>;
/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct UiState {
    pub loaded_files: LoadedFiles,
    pub dock_state: DockState<EguiWindow>,
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

impl UiState {
    pub fn new(path: &Option<String>) -> Self {
        Self {
            loaded_files: load_files(path),
            dock_state: ribbon_cadpanel(),
        }
    }

    pub fn ui(
        &mut self,
        ctx: &mut egui::Context,
        act_write: EventWriter<Act>,
        dock_buffer: &mut DockBuffer,
        ss: &SnapSettings,
        tc: &mut TagCharacteristics,
    ) {
        let mut tab_viewer = TabViewer {
            act_write,
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
        ewm: &mut EventWriter<Menu>,
    ) {
        ewrsp.send(UpdateSnapPoints(false)); //snap plugin
        rmcb.as_mut().reset(); //construction plugin
        ewm.send(Menu::NoMenu); //cameraui plugin
        despawn_all_phantoms(co, ewp, fs); //phantom plugin
    }

    pub fn push_history(&mut self, act: &Act, db: &mut ResMut<DockBuffer>) {
        let hb = &mut db.history;
        let mut meta_data = String::new();

        if act == &hb.latest_act {
            return;
        }

        hb.all_history.push_str(match act {
            Act::None => return,
            Act::Exit => "Cleaning up.",
            Act::QuitWithoutSaving => "Quit without saving.",
            Act::DeselectAll => "Deselecting everything.",
            Act::Confirm => "Action confirmed.",
            // Act::OpenCADTerm => "Terminal opened.",
            Act::TryAct(a) => {
                meta_data = format!("{a:?}");
                "Terminal submitted: "
            }
            Act::ToggleSnap(a) => {
                meta_data = format!("{a}");
                "Snap configuration changed: "
            }
            Act::EguiFocus(a) => {
                meta_data = format!("{a:?}");
                "Focusing tab: "
            }

            Act::Insert(a) => {
                meta_data = format!("{a}");
                "Insert: "
            }
            Act::PullCameraFocus(_) => "Camera moved.",
            Act::FitView => "Fit view to all entities.",
            Act::MoveCamera(_) => return,
            Act::ZoomCamera(_) => return,
            Act::ModifyTag(_a, b) => {
                meta_data = format!("{b}");
                "Tag modification: "
            }
            Act::ModifyTaglist(a) => {
                meta_data = format!("{a}");
                "Tag list modification: "
            }
            Act::CameraUIMenu(m) => {
                meta_data = format!("{m}");
                "Menu opened: "
            }
            Act::ClearSnaps => todo!(),
        });
        hb.all_history.push_str(&meta_data);
        hb.all_history.push('\n');

        db.history.latest_act = act.clone();
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

/// This is a [`egui_dock`] implimentation. This also directly shows all the available tabs.
struct TabViewer<'a> {
    act_write: EventWriter<'a, Act>,
    ss: &'a SnapSettings,
    db: &'a mut DockBuffer,
    tc: &'a mut TagCharacteristics,
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
            EguiWindow::Inspect => {
                view_inspection(ui, &mut self.db.inspection, &mut self.act_write)
            }
            EguiWindow::StateRibbon => view_stateribbon(ui, self.ss),
            EguiWindow::Taglist => view_taglist(self.tc, ui, &mut self.act_write, self.db),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, _window: &Self::Tab) -> bool {
        true
    }
}

fn view_stateribbon(ui: &mut egui::Ui, ss: &SnapSettings) {
    ui.label(format!("{:?}", ss));
}
