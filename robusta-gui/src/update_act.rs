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
    mut ewm: EventWriter<Menu>,
) {
    for act in era.read() {
        let mut binding = act.clone();
        if let Act::TryAct(string) = act {
            binding = to_act(string);
        }

        uis.push_history(act, &mut db);

        match &binding {
            Act::EguiFocus(ew) => uis.new_focus(ew),
            Act::DeselectAll => deselect_all(&mut co, &es, &mut dsel),
            // Act::OpenCADTerm => uis.cad_state.cad_term = Some(String::new()),
            Act::Insert(sp) => insert(sp, &mut rmcb, &mut erre, &mut ewrsp),
            Act::ToggleSnap(a) => toggle_snap(&mut ss, a, &mut ewrsp),
            Act::ClearSnaps => {
                ss.reset();
                ewrsp.send(UpdateSnapPoints(true));
            }
            Act::CameraUIMenu(sp) => {
                ewm.send(sp.clone());
            }
            Act::Confirm => index_point(&qrerpp, &mut ewci),
            Act::Exit => uis.close_all(&mut co, &qerpp, &mut ewrsp, &mut rmcb, &mut fs, &mut ewm),
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

fn insert(
    oct: &ConstructType,
    rmcb: &mut ResMut<ConstructionBuffer>,
    erre: &mut EventWriter<REntity>,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
) {
    rmcb.build = Some(*oct);
    ewrsp.send(UpdateSnapPoints(true));
    erre.send(REntity::PhantomPoint);
}
