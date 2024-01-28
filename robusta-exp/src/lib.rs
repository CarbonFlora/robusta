use bevy::utils::{hashbrown::Equivalent, HashMap};
use std::any::TypeId;

use bevy::{prelude::*, render::camera::Viewport};
use bevy_asset::{ReflectAsset, UntypedAssetId};
use bevy_egui::EguiContext;
// use bevy_mod_picking::backends::egui::EguiPointer;
// use bevy_mod_picking::prelude::*;
use bevy_reflect::{TypeRegistry, Uuid};
use bevy_window::PrimaryWindow;
use egui_dock::{dock_state, DockArea, DockState, Node, NodeIndex, Style};
use robusta_gui::uistate::{EguiWindow, UiState, ViewportCamera};

// Spawn a camera. Two cameras should not be assigned to the same viewport.
pub fn camera_startup(
    mut ui_state: ResMut<UiState>,
    cameras: Query<(&Camera, &ViewportCamera)>,
    mut commands: Commands,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
) {
    for node in ui_state.state.iter_all_nodes() {
        for tab in node.1.iter_tabs() {
            match tab {
                EguiWindow::CADView(viewport_state) => {
                    assign_camera(&mut commands, viewport_state.viewport_id);
                    // assign_viewport_rectangles(
                    //     &mut ui_state,
                    //     viewport_state.viewport_id,
                    //     node.1.rect(),
                    //     primary_window,
                    //     egui_settings,
                    // );
                }
                _ => (),
            };
        }
    }

    // ui_state.viewport_rectangles = map_viewport_rectangles(ui_state, cameras);
}

/// Spawns a camera for each existing viewport and assigns the viewport UUID to it.
fn assign_camera(commands: &mut Commands, viewport_id: Uuid) {
    commands.spawn(Camera2dBundle::default()).insert((
        //instead of default, set the camera viewport to what's found in dock.
        bevy_pancam::PanCam {
            zoom_to_cursor: false,
            ..default()
        },
        ViewportCamera::new(viewport_id),
    ));
}

// Assign the UUID to UiState viewport, which will keep track of the viewport.
fn assign_viewport_rectangles(
    ui_state: &mut ResMut<UiState>,
    viewport_id: Uuid,
    rect: Option<egui::emath::Rect>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
) {
    let scale_factor = scale_factor(primary_window, egui_settings);
    let rect = rect.expect("CADView tab exists but does not have a display.");
    let viewport_position = rect.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = rect.size() * scale_factor as f32;

    ui_state.viewport_rectangles.insert(
        viewport_id,
        Viewport {
            physical_position: UVec2::new(viewport_position.x as u32, viewport_position.y as u32),
            physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
            depth: 0.0..1.0,
        },
    );
}

// make camera only render to view not obstructed by UI
// todo!() This function should only run after viewport
pub fn update_camera_viewport(
    ui_state: Res<UiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<ViewportCamera>>,
) {
    let scale_factor = scale_factor(primary_window, egui_settings);
    // ui_state
    //I need to assign viewports to the cameras.

    // cam.viewport = match ui_state.get_viewport_rectangles() {
    //     None => None,
    //     Some(l) => {
    //         Some(Viewport { physical_position: , physical_size: , depth: 0.0..1.0 })
    //     }
    // };

    // let viewport_pos = ui_state.viewport_rectangles.left_top().to_vec2() * scale_factor as f32;
    // let viewport_size = ui_state.viewport_rectangles.size() * scale_factor as f32;

    // cam.viewport = Some(Viewport {
    //     physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
    //     physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
    //     depth: 0.0..1.0,
    // });
}

fn scale_factor(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
) -> f32 {
    let window = primary_window.get_single().expect(&format!(
        "There's not exactly 1 primary window. Recieved: {}",
        primary_window.iter().count(),
    ));

    return (window.scale_factor() * egui_settings.scale_factor) as f32;
}
