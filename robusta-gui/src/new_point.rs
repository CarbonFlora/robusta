use bevy::{
    prelude::{Commands, Entity, EventReader, Mesh, ResMut},
    sprite::ColorMaterial,
};
use bevy_asset::Assets;
use bevy_window::CursorMoved;

use crate::uistate::UiState;

pub fn new_point(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    held_point: Option<Entity>,
) {
    // ui_state.cad_state.held_point
}
