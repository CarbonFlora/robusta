use super::*;

#[allow(clippy::too_many_arguments)]
pub fn update_act(
    mut era: EventReader<Act>,
    mut ewrsp: EventWriter<UpdateSnapPoints>,
    qerpp: Query<Entity, With<RPhantomPointer>>,
    qrerpp: Query<&REntity, (With<RPhantomPointer>, Without<bevy_pancam::PanCam>)>,
    es: Query<(Entity, &Selected), With<Selected>>,
    mut uis: ResMut<UiState>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut co: Commands,
    mut erre: EventWriter<REntity>,
    mut dsel: EventWriter<Pointer<Deselect>>,
    mut ewci: EventWriter<ConstructionInput>,
    mut rmcb: ResMut<ConstructionBuffer>,
    mut fs: ResMut<PhantomSnaps>,
    mut ss: ResMut<SnapSettings>,
    mut db: ResMut<DockBuffer>,
) {
    for act in era.read() {
        let mut binding = act.clone();
        if let Act::TryAct(string) = act {
            binding = to_act(string);
        }

        uis.push_history(act, &mut db);

        match &binding {
            Act::Inspect => uis.inspect(),
            Act::DeselectAll => deselect_all(&mut co, &es, &mut dsel),
            Act::OpenCADTerm => uis.cad_state.cad_term = Some(String::new()),
            Act::Insert(sp) => insert(sp, &mut uis, &mut rmcb, &mut erre, &mut ewrsp),
            Act::ToggleSnap(a) => toggle_snap(&mut ss, a, &mut uis, &mut ewrsp),
            Act::Confirm => index_point(&qrerpp, &mut ewci),
            Act::Exit => uis.close_all(&mut co, &qerpp, &mut ewrsp, &mut rmcb, &mut fs),
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
        "endpoint" | "end" => Act::ToggleSnap(Some(SnapType::Endpoint)),
        "midpoint" | "mid" => Act::ToggleSnap(Some(SnapType::Midpoint)),
        "nthpoint" | "nth" => Act::ToggleSnap(Some(SnapType::Nthpoint(Some(divisions)))),
        "intersection" | "int" => Act::ToggleSnap(Some(SnapType::Intersection)),
        "perpendicular" | "per" => Act::ToggleSnap(Some(SnapType::Perpendicular)),
        "tangent" | "tan" => Act::ToggleSnap(Some(SnapType::Tangent)),
        _ => Act::None,
    }
}

fn insert(
    oct: &Option<ConstructType>,
    uis: &mut UiState,
    rmcb: &mut ResMut<ConstructionBuffer>,
    erre: &mut EventWriter<REntity>,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
) {
    match oct {
        None => {
            uis.cad_state.insert_menu = Some(*oct);
            uis.cad_state.mode = Mode::Insert;
        }
        Some(ct) => {
            uis.cad_state.insert_menu = None;
            uis.cad_state.mode = Mode::Normal;
            rmcb.build = Some(*ct);
            ewrsp.send(UpdateSnapPoints(true));
            erre.send(REntity::PhantomPoint);
        }
    };
}
