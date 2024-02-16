use bevy::prelude::*;
use bevy_mod_picking::{prelude::Pointer, selection::Deselect};

use crate::{
    keystrokes::Act,
    uistate::{CADPanel, UiState},
    EntityMapping,
};

pub fn update_act(
    mut act_read: EventReader<Act>,
    mut ui_state: ResMut<UiState>,
    entity_mapping: Res<EntityMapping>,
    mut deselections: EventWriter<Pointer<Deselect>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    // for focus
    primary_window: Query<&Window, With<bevy_window::PrimaryWindow>>,
    mut query: Query<(
        &bevy_pancam::PanCam,
        &mut Transform,
        &OrthographicProjection,
    )>,
    // mut camera: Query<(&mut Camera, &GlobalTransform), With<bevy_window::PrimaryWindow>>,
    mut camera: Query<(&mut Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
    // mut camera: Query<(&mut Camera, &GlobalTransform), Without<CADPanel>>,
    // mut camera: Query<&mut Camera, With<bevy_window::PrimaryWindow>>,
    // mut gt: Query<&GlobalTransform>,
) {
    for act in act_read.read() {
        if let Act::TryAct(a) = act {
            run_act(
                &to_act(a),
                &mut ui_state,
                &entity_mapping,
                &mut deselections,
                &mut app_exit_events,
                &primary_window,
                &mut query,
                &mut camera,
            );
        } else {
            run_act(
                act,
                &mut ui_state,
                &entity_mapping,
                &mut deselections,
                &mut app_exit_events,
                &primary_window,
                &mut query,
                &mut camera,
            )
        }
    }
}

fn run_act(
    act: &Act,
    ui_state: &mut ResMut<UiState>,
    entity_mapping: &Res<EntityMapping>,
    deselections: &mut EventWriter<Pointer<Deselect>>,
    app_exit_events: &mut ResMut<Events<bevy::app::AppExit>>,
    primary_window: &Query<&Window, With<bevy_window::PrimaryWindow>>,
    query: &mut Query<(
        &bevy_pancam::PanCam,
        &mut Transform,
        &OrthographicProjection,
    )>,
    // camera: &mut Query<(&mut Camera, &GlobalTransform), Without<CADPanel>>,
    // camera: &mut Query<(&mut Camera, &GlobalTransform), With<bevy_window::PrimaryWindow>>,
    camera: &mut Query<(&mut Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
    // camera: &mut Query<&mut Camera, With<bevy_window::PrimaryWindow>>,
    // gt: &mut Query<&GlobalTransform>,
    // camera: &mut Query<&mut Camera, With<bevy_window::PrimaryWindow>>,
    // gt: &mut Query<&GlobalTransform>,
) {
    match act {
        // Act::NewPoint => new_point(
        //     &mut commands,
        //     &mut meshes,
        //     &mut materials,
        //     ui_state.cad_state.construction,
        // ),
        // Act::DeselectAll => deselect_all(&ui_state, deselections),
        Act::PullCameraFocus(rect) => camera_movement(rect, primary_window, query, camera),
        Act::Inspect => ui_state.inspect(),
        Act::DeselectAll => ui_state.deselect_all(deselections),
        Act::OpenCADTerm => ui_state.cad_state.cad_term = Some(String::new()),
        Act::DebugReMapSelection(entity) => ui_state.remap_selection(entity, entity_mapping),
        Act::Exit => ui_state.cad_state.close_all(),
        Act::QuitWithoutSaving => app_exit_events.send(bevy::app::AppExit),
        _ => (),
    }
}

fn to_act(input: &String) -> Act {
    return match input.as_str() {
        "deselect" | "dsa" => Act::DeselectAll,
        "inspect" | "i" => Act::Inspect,
        "point" | "p" => Act::NewPoint,
        "q!" => Act::QuitWithoutSaving,
        _ => Act::None,
    };
}

// This is untested
fn camera_movement(
    rect: &Rect,
    primary_window: &Query<&Window, With<bevy_window::PrimaryWindow>>,
    query: &mut Query<(
        &bevy_pancam::PanCam,
        &mut Transform,
        &OrthographicProjection,
    )>,
    camera: &mut Query<(&mut Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
    // camera: &mut Query<(&mut Camera, &GlobalTransform), Without<CADPanel>>,
) {
    let window = primary_window.single();
    let window_size = Vec2::new(window.width(), window.height());

    let (a, b) = camera.get_single_mut().unwrap();
    let current_3d_pos = b.to_scale_rotation_translation().2;
    let current_2d_pos = Vec2::new(current_3d_pos.x, current_3d_pos.y);

    let last_pos = (rect.max - rect.min) / 2.;

    let delta_device_pixels = current_2d_pos - last_pos;

    for (cam, mut transform, projection) in query {
        // if cam.enabled {
        let proj_size = projection.area.size();

        let world_units_per_device_pixel = proj_size / window_size;

        // The proposed new camera position
        let delta_world = delta_device_pixels * world_units_per_device_pixel;
        let mut proposed_cam_transform = transform.translation - delta_world.extend(0.);

        // Check whether the proposed camera movement would be within the provided boundaries, override it if we
        // need to do so to stay within bounds.
        if let Some(min_x_boundary) = cam.min_x {
            let min_safe_cam_x = min_x_boundary + proj_size.x / 2.;
            proposed_cam_transform.x = proposed_cam_transform.x.max(min_safe_cam_x);
        }
        if let Some(max_x_boundary) = cam.max_x {
            let max_safe_cam_x = max_x_boundary - proj_size.x / 2.;
            proposed_cam_transform.x = proposed_cam_transform.x.min(max_safe_cam_x);
        }
        if let Some(min_y_boundary) = cam.min_y {
            let min_safe_cam_y = min_y_boundary + proj_size.y / 2.;
            proposed_cam_transform.y = proposed_cam_transform.y.max(min_safe_cam_y);
        }
        if let Some(max_y_boundary) = cam.max_y {
            let max_safe_cam_y = max_y_boundary - proj_size.y / 2.;
            proposed_cam_transform.y = proposed_cam_transform.y.min(max_safe_cam_y);
        }

        transform.translation = proposed_cam_transform;
        // }
    }
}
