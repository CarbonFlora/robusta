use self::{leaves::taglist::view_taglist, plugins::phantom::PhantomAct};

use super::*;

type LoadedFiles = HashMap<Option<String>, InterchangeFormat>;
/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct UiState {
    pub loaded_files: LoadedFiles,
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

#[derive(Debug, Default, Resource, Clone)]
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
        }
    }

    // #[allow(clippy::too_many_arguments)]
    // pub fn ui(
    //     &mut self,
    //     ctx: &mut egui::Context,
    //     act_write: EventWriter<Act>,
    //     ewm: &mut ModalResources,
    //     dock_buffer: &mut DockBuffer,
    //     ss: &SnapSettings,
    // ) {
    //     let mut tab_viewer = TabViewer {
    //         act_write,
    //         ewm,
    //         db: dock_buffer,
    //         ss,
    //     };
    //     DockArea::new(&mut self.dock_state)
    //         .style(Style::from_egui(ctx.style().as_ref()))
    //         .show(ctx, &mut tab_viewer);
    // }

    pub fn close_all(
        &mut self,
        ewrsp: &mut EventWriter<UpdateSnapPoints>,
        rmcb: &mut ResMut<ConstructionBuffer>,
        ewm: &mut EventWriter<Menu>,
        ewpa: &mut EventWriter<PhantomAct>,
    ) {
        ewrsp.send(UpdateSnapPoints(false)); //snap plugin
        rmcb.as_mut().reset(); //construction plugin
        ewm.send(Menu::NoMenu); //cameraui plugin
        ewpa.send(PhantomAct::DespawnAll); //phantom plugin
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
            Act::KeyState(sp) => {
                meta_data = format!("{sp:?}");
                "Keystate: "
            }
        });
        hb.all_history.push_str(&meta_data);
        hb.all_history.push('\n');

        db.history.latest_act = act.clone();
    }
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
