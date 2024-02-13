use bevy::{
    prelude::{Commands, Entity, Mesh, ResMut},
    sprite::ColorMaterial,
};
use bevy_asset::Assets;

pub fn new_point(
    _commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<ColorMaterial>>,
    _held_point: Option<Entity>,
) {
    // ui_state.cad_state.held_point
}
