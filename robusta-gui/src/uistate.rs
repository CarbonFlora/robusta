use self::plugins::phantom::PhantomAct;

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
