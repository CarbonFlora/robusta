use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::{prelude::*, PickableBundle};
use robusta_gui::uistate::DoSomethingComplex;

use crate::*;

pub fn draw_first(
    ui_state: Res<UiState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for file in &ui_state.loaded_files {
        for point in &file.1.points {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(1.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(
                        point.coordinates.x,
                        point.coordinates.y,
                        0.,
                    )),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Select>>::send_event::<DoSomethingComplex>(),
                On::<Pointer<Deselect>>::send_event::<DoSomethingComplex>(),
            ));
        }
    }
}
