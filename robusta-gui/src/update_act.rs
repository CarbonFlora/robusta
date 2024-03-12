use self::plugins::construction::construct_text;

use super::*;

#[allow(clippy::too_many_arguments)]
pub fn update_act(
    mut era: EventReader<Act>,
    mut ewrsp: EventWriter<UpdateSnapPoints>,
    qerpp: Query<Entity, With<RPhantomPointer>>,
    qrerpp: Query<&REntity, (With<RPhantomPointer>, Without<bevy_pancam::PanCam>)>,
    es: Query<(Entity, &Selected), With<Selected>>,
    mut ui_state: ResMut<UiState>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut co: Commands,
    mut erre: EventWriter<REntity>,
    mut dsel: EventWriter<Pointer<Deselect>>,
    mut ewci: EventWriter<ConstructionInput>,
    mut rmcb: ResMut<ConstructionBuffer>,
    mut fs: ResMut<PhantomSnaps>,
) {
    for act in era.read() {
        let mut binding = act.clone();
        if let Act::TryAct(string) = act {
            binding = to_act(string);
        }

        ui_state.push_history(act);

        match &binding {
            Act::Inspect => ui_state.inspect(),
            Act::DeselectAll => deselect_all(&mut co, &es, &mut dsel),
            Act::OpenCADTerm => ui_state.cad_state.cad_term = Some(String::new()),
            Act::NewPoint => construct_point(&mut erre, &mut ewrsp, &mut rmcb),
            Act::NewLine => construct_line(&mut erre, &mut ewrsp, &mut rmcb),
            Act::NewText => construct_text(&mut erre, &mut ewrsp, &mut rmcb),
            Act::ToggleSnap(a) => ui_state.toggle_snap(a),
            Act::ToggleSnapOff => ui_state.toggle_snap_off(&mut ewrsp),
            Act::Confirm => index_point(&qrerpp, &mut ewci),
            Act::Exit => ui_state.close_all(&mut co, &qerpp, &mut ewrsp, &mut rmcb, &mut fs),
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
        "inspect" | "i" => Act::Inspect,
        "fitview" | "fv" => Act::FitView,
        "snap" | "s" => snap_acts(text_buffer),
        "point" | "p" => Act::NewPoint,
        "line" | "l" => Act::NewLine,
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
        "endpoint" | "end" => Act::ToggleSnap(Snaps::Endpoint),
        "midpoint" | "mid" => Act::ToggleSnap(Snaps::Midpoint),
        "nthpoint" | "nth" => Act::ToggleSnap(Snaps::Nthpoint(divisions)),
        "intersection" | "int" => Act::ToggleSnap(Snaps::Intersection),
        "perpendicular" | "per" => Act::ToggleSnap(Snaps::Perpendicular),
        "tangent" | "tan" => Act::ToggleSnap(Snaps::Tangent),
        "off" => Act::ToggleSnapOff,
        _ => Act::None,
    }
}
