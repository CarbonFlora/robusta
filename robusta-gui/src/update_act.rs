use bevy::prelude::*;
use bevy_mod_picking::{prelude::Pointer, selection::Deselect};

use crate::{keystrokes::Act, uistate::UiState, EntityMapping, Snaps};

#[allow(clippy::too_many_arguments)]
pub fn update_act(
    mut act_read: EventReader<Act>,
    mut ui_state: ResMut<UiState>,
    mut entity_mapping: ResMut<EntityMapping>,
    mut deselections: EventWriter<Pointer<Deselect>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut camera: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for act in act_read.read() {
        let mut binding = act.clone();
        if let Act::TryAct(string) = act {
            binding = to_act(string);
        }

        ui_state.push_history(act, &entity_mapping.hash);

        match &binding {
            Act::MoveCamera((x, y)) => camera_transform(x, y, &mut camera),
            Act::ZoomCamera(z) => camera_zoom(z, &mut camera),
            Act::PullCameraFocus(rect) => camera_movement(rect, &mut camera),
            Act::FitView => camera_movement(&ui_state.all_rect(), &mut camera),
            Act::Inspect => ui_state.inspect(),
            Act::DeselectAll => ui_state.deselect_all(&mut deselections),
            Act::OpenCADTerm => ui_state.cad_state.cad_term = Some(String::new()),
            Act::DebugReMapSelection(entity) => ui_state.remap_selection(entity, &entity_mapping),
            Act::NewPoint => ui_state.new_point(&mut commands, &mut meshes, &mut materials),
            Act::ToggleSnap(a) => ui_state.toggle_snap(a),
            Act::ToggleSnapOff => ui_state.toggle_snap_off(),
            Act::Confirm => ui_state.canonize(&mut commands, &mut entity_mapping),
            Act::Exit => ui_state.close_all(&mut commands),
            Act::QuitWithoutSaving => app_exit_events.send(bevy::app::AppExit),
            _ => (),
        }
    }
}

fn to_act(input: &str) -> Act {
    match input {
        "deselect" | "dsa" => Act::DeselectAll,
        "inspect" | "i" => Act::Inspect,
        "fitview" | "fv" => Act::FitView,
        "point" | "p" => Act::NewPoint,
        "snap endpoint" | "s end" => Act::ToggleSnap(Snaps::Endpoint),
        "snap midpoint" | "s mid" => Act::ToggleSnap(Snaps::Midpoint),
        "snap center" | "s cen" => Act::ToggleSnap(Snaps::Center),
        "snap intersection" | "s int" => Act::ToggleSnap(Snaps::Intersection),
        "snap perpendicular" | "s per" => Act::ToggleSnap(Snaps::Perpendicular),
        "snap tangent" | "s tan" => Act::ToggleSnap(Snaps::Tangent),
        "snap off" | "s off" => Act::ToggleSnapOff,
        "q!" => Act::QuitWithoutSaving,
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
