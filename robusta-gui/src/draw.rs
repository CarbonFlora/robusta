use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::{prelude::*, PickableBundle};

use crate::*;

use self::uistate::{SelectionInstance, UiState};

pub fn draw_first(
    ui_state: Res<UiState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for file in &ui_state.loaded_files {
        draw_points(&mut commands, &mut meshes, &mut materials, file.1);
        draw_lines(&mut commands, &mut meshes, &mut materials, file.1);
    }
}

fn draw_points(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    wrapper: &DXFWrapper,
) {
    for point in &wrapper.points {
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
            On::<Pointer<Select>>::send_event::<SelectionInstance>(),
            On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        ));
    }
}

fn draw_lines(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    wrapper: &DXFWrapper,
) {
    for line in &wrapper.lines {
        // Create my own mesh for lines.
        // commands.spawn((
        //     MaterialMesh2dBundle {
        //         mesh: meshes
        //             .add(shape::Quad::new(Vec2 { x: 5., y: 15. }).into())
        //             .into(),
        //         material: materials.add(ColorMaterial::from(Color::WHITE)),
        //         transform: Transform::from_translation(Vec3::new(
        //             line.definition[0].coordinates.x,
        //             line.definition[0].coordinates.y,
        //             0.,
        //         )),
        //         ..default()
        //     },
        //     PickableBundle::default(),
        //     On::<Pointer<Select>>::send_event::<SelectionInstance>(),
        //     On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        // ));
    }
}
