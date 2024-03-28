use super::*;

#[derive(Debug, Resource, Default, Clone, PartialEq)]
pub struct HistoryBuffer {
    pub latest_act: Act,
    pub all_history: String,
}

pub fn view_history(ui: &mut egui::Ui, hb: &HistoryBuffer) {
    // ui.label(format!("Latest Action: {:?}", history.0));
    // ui.separator();
    ui.label(hb.all_history.to_string());
}

pub fn refresh_history_buffer(mut era: EventReader<Act>, mut rmdb: ResMut<DockBuffer>) {
    let hb = &mut rmdb.history;
    let mut meta_data = String::new();

    for a in era.read() {
        if a == &hb.latest_act {
            continue;
        }

        hb.all_history.push_str(match a {
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
            Act::Taglist(a) => {
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
        hb.latest_act = a.clone();
    }
}
