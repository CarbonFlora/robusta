use std::str::SplitWhitespace;

use bevy::prelude::*;
use bevy_mod_picking::{events::Pointer, selection::Deselect};

use crate::{
    keystrokes::Act,
    phantom::{spawn_phantom_point, RPhantomPointer},
    rselection::{deselect_all, Selected},
    snap::UpdateSnapPoints,
    uistate::UiState,
    REntity, Snaps, TopZLayer,
};

#[allow(clippy::too_many_arguments)]
pub fn update_act(
    mut act_read: EventReader<Act>,
    mut ewrsp: EventWriter<UpdateSnapPoints>,
    re: Query<&REntity>,
    ewp: Query<Entity, With<RPhantomPointer>>,
    es: Query<(Entity, &Selected), With<Selected>>,
    mut ui_state: ResMut<UiState>,
    mut tzi: ResMut<TopZLayer>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut camera: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
    mut co: Commands,
    mut me: ResMut<Assets<Mesh>>,
    mut ma: ResMut<Assets<ColorMaterial>>,
    mut dsel: EventWriter<Pointer<Deselect>>,
) {
    for act in act_read.read() {
        let mut binding = act.clone();
        if let Act::TryAct(string) = act {
            binding = to_act(string);
        }

        ui_state.push_history(act);

        match &binding {
            Act::MoveCamera((x, y)) => camera_transform(x, y, &mut camera),
            Act::ZoomCamera(z) => camera_zoom(z, &mut camera),
            Act::PullCameraFocus(rect) => camera_movement(rect, &mut camera),
            Act::FitView => camera_movement(&fit_view_rect(&re), &mut camera),
            Act::Inspect => ui_state.inspect(),
            Act::DeselectAll => deselect_all(&mut co, &es, &mut dsel),
            Act::OpenCADTerm => ui_state.cad_state.cad_term = Some(String::new()),
            Act::NewPoint => spawn_phantom_point(&mut co, &mut me, &mut ma, &mut tzi, &mut ewrsp),
            Act::NewLine => (),
            Act::ToggleSnap(a) => ui_state.toggle_snap(a),
            Act::ToggleSnapOff => ui_state.toggle_snap_off(&mut ewrsp),
            Act::Confirm => (),
            Act::Exit => ui_state.close_all(&mut co, &ewp, &mut ewrsp),
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

fn camera_movement(
    entity_position: &Rect,
    camera: &mut Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
) {
    let mut camera = camera.get_single_mut().unwrap();
    let current_3d_pos = camera.1.to_scale_rotation_translation().2;
    let current_2d_pos = Vec2::new(current_3d_pos.x, current_3d_pos.y);
    let entity_position = (entity_position.max - entity_position.min) / 2. + entity_position.min;
    let delta = current_2d_pos - entity_position;
    let proposed_cam_transform = camera.0.translation - delta.extend(0.);

    camera.0.translation = proposed_cam_transform;
}

fn camera_transform(
    x: &f32,
    y: &f32,
    camera: &mut Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
) {
    let mut camera = camera.get_single_mut().unwrap();
    let scale = camera.2.scale;
    let current_3d_pos = camera.1.to_scale_rotation_translation().2;
    let current_2d_pos = Vec2::new(current_3d_pos.x, current_3d_pos.y);
    let movement = Vec2 {
        x: *x * scale * 20.,
        y: *y * scale * 20.,
    };
    let delta = current_2d_pos + movement;
    let proposed_cam_transform = delta.extend(0.);

    camera.0.translation = proposed_cam_transform;
}

fn camera_zoom(
    z: &f32,
    camera: &mut Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
) {
    let mut camera = camera.get_single_mut().unwrap();
    camera.2.scale += z * 0.02;
    if camera.2.scale < 0. {
        camera.2.scale = 0.;
    }
}

fn fit_view_rect(re: &Query<&REntity>) -> Rect {
    let mut a = Vec::new();
    for e in re.iter() {
        match e {
            REntity::Arc(sp) => a.extend(&sp.definition),
            REntity::Circle(sp) => a.extend(&sp.definition),
            REntity::Line(sp) => a.extend(&sp.definition),
            REntity::Point(sp) => a.push(sp),
            REntity::Text(sp) => a.extend(&sp.definition),
        }
    }

    let (mut min_x, mut min_y, mut max_x, mut max_y) = match a.first() {
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
