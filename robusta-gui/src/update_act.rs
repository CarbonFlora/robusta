use self::plugins::{construction::insert, phantom::PhantomAct};

use super::*;

#[allow(clippy::too_many_arguments)]
pub fn update_act(
    mut era: EventReader<Act>,
    qrerpp: Query<&REntity, (With<RPhantomPointer>, Without<bevy_pancam::PanCam>)>,
    es: Query<(Entity, &Selected), With<Selected>>,
    mut uis: ResMut<UiState>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut co: Commands,
    mut ewre: EventWriter<REntity>,
    mut dsel: EventWriter<Pointer<Deselect>>,
    mut ewci: EventWriter<ConstructionInput>,
    mut rmcb: ResMut<ConstructionBuffer>,
    mut db: ResMut<DockBuffer>,
    mut ewm: EventWriter<Menu>,
    mut ewpa: EventWriter<PhantomAct>,
    mut ewrsp: EventWriter<UpdateSnapPoints>,
) {
    for act in era.read() {
        let mut binding = act.clone();
        if let Act::TryAct(string) = act {
            binding = to_act(string);
        }

        uis.push_history(act, &mut db);

        match &binding {
            // Act::EguiFocus(ew) => uis.new_focus(ew),
            Act::DeselectAll => deselect_all(&mut co, &es, &mut dsel),
            // Act::OpenCADTerm => uis.cad_state.cad_term = Some(String::new()),
            Act::Insert(sp) => insert(sp, &mut rmcb, &mut ewre, &mut ewrsp),
            Act::CameraUIMenu(sp) => {
                ewm.send(sp.clone());
            }
            Act::Confirm => index_point(&qrerpp, &mut ewci, &mut ewre),
            Act::Exit => uis.close_all(&mut ewrsp, &mut rmcb, &mut ewm, &mut ewpa),
            Act::QuitWithoutSaving => {
                app_exit_events.send(bevy::app::AppExit);
            }
            _ => (),
        }
    }
}

fn to_act(input: &str) -> Act {
    let mut text_buffer = input.split_whitespace();
    match text_buffer
        .next()
        .unwrap_or_default()
        .trim_start_matches(':')
        .to_lowercase()
        .as_str()
    {
        "deselect" | "dsa" => Act::DeselectAll,
        "inspect" => Act::EguiFocus(EguiWindow::Inspect),
        "fitview" | "fv" => Act::FitView,
        "snap" | "s" => snap_acts(text_buffer),
        "q!" => Act::QuitWithoutSaving,
        _ => Act::None,
    }
}

fn snap_acts(mut text_buffer: SplitWhitespace) -> Act {
    let text = text_buffer.next().unwrap_or_default();
    let divisions = text_buffer
        .next()
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap_or_default();
    match text {
        "endpoint" | "end" => Act::ToggleSnap(SnapType::Endpoint),
        "midpoint" | "mid" => Act::ToggleSnap(SnapType::Midpoint),
        "nthpoint" | "nth" => Act::ToggleSnap(SnapType::Nthpoint(Some(divisions))),
        "intersection" | "int" => Act::ToggleSnap(SnapType::Intersection),
        "perpendicular" | "per" => Act::ToggleSnap(SnapType::Perpendicular),
        "tangent" | "tan" => Act::ToggleSnap(SnapType::Tangent),
        _ => Act::None,
    }
}
