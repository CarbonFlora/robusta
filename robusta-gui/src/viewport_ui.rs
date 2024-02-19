use robusta_core::{line::Line, point::Point, RobustaEntity};

use super::*;

#[allow(clippy::too_many_arguments)]
pub fn update_viewport_ui(
    act_write: EventWriter<Act>,
    mut ui_state: ResMut<UiState>,
    mut entity_transform: Query<
        &mut Transform,
        (With<PhantomREntity>, Without<bevy_pancam::PanCam>),
    >, //If this gets too clunky, use commands.entity() instead
    egui_context_primary: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut camera: Query<&mut bevy_pancam::PanCam>,
    window: Query<&Window, With<PrimaryWindow>>,
    transform: Query<(&Camera, &Transform, &GlobalTransform), With<bevy_pancam::PanCam>>,
    secondary_window: Query<&mut Window, Without<PrimaryWindow>>,
) {
    match secondary_window.single().focused {
        true => {
            camera.single_mut().enabled = false;
        }
        false => {
            camera.single_mut().enabled = true;
        }
    }

    if ui_state.cad_state.cad_term.is_some() {
        update_terminal_egui(act_write, &mut ui_state, egui_context_primary);
    }

    if let Some((_a, b)) = &mut ui_state.cad_state.construction {
        match b {
            RobustaEntity::Point(b) => {
                update_construction_point(b, &mut entity_transform, window, transform)
            }
            RobustaEntity::Line(b) => place_line(b),
            _ => (),
        }
    }
}

fn update_construction_point(
    target: &mut Point,
    entity_transform: &mut Query<
        &mut Transform,
        (With<PhantomREntity>, Without<bevy_pancam::PanCam>),
    >,
    window: Query<&Window, With<PrimaryWindow>>,
    transform: Query<(&Camera, &Transform, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    let mut a = entity_transform.single_mut();
    let (camera, _transform, global_transform) = transform.single();
    if let Some(cursor_world_pos) = window
        .single()
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(global_transform, cursor_pos))
    {
        target.xyz_mut(cursor_world_pos.x, cursor_world_pos.y, target.coordinates.z);
        println!("target: {:?}", target);
        a.translation = Vec3::new(cursor_world_pos.x, cursor_world_pos.y, a.translation.z)
    }
}

fn place_line(target: &mut Line) {}
