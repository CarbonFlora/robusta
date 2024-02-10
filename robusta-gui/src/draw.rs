use std::f32::consts::PI;

use bevy::{
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
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
        draw_arcs(&mut commands, &mut meshes, &mut materials, file.1);
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
                mesh: meshes.add(shape::Circle::new(0.5).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(
                    point.coordinates.x,
                    point.coordinates.y,
                    9.,
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
    let line_width = 0.3f32;
    for line in &wrapper.lines {
        let delta_x = line.definition[1].coordinates.x - line.definition[0].coordinates.x;
        let delta_y = line.definition[1].coordinates.y - line.definition[0].coordinates.y;
        let length = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        let angle_rad = angle_full_circle(delta_x, delta_y);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(line_mesh(line_width, length, angle_rad).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(
                    line.definition[0].coordinates.x,
                    line.definition[0].coordinates.y,
                    8.,
                )),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectionInstance>(),
            On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        ));
    }
}

fn draw_arcs(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    wrapper: &DXFWrapper,
) {
    let line_width = 0.3f32;
    for arc in &wrapper.arcs {
        // let delta_x = line.definition[1].coordinates.x - line.definition[0].coordinates.x;
        // let delta_y = line.definition[1].coordinates.y - line.definition[0].coordinates.y;
        // let length = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        // let angle_rad = angle_full_circle(delta_x, delta_y);

        commands.spawn((
            MaterialMesh2dBundle {
                // mesh: meshes.add(arc_mesh(line_width).into()).into(),
                mesh: meshes.add(shape::Circle::new(1.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(
                    arc.definition[0].coordinates.x,
                    arc.definition[0].coordinates.y,
                    7.,
                )),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectionInstance>(),
            On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        ));
    }
}

fn angle_full_circle(delta_x: f32, delta_y: f32) -> f32 {
    if delta_x == 0. && delta_y == 0. {
        panic!("Zero length line detected.")
    } else if delta_x == 0. {
        match delta_y.is_sign_positive() {
            true => return PI * 0.5,
            false => return PI * 1.5,
        }
    }

    let mut angle_rad = (delta_y / delta_x).atan();
    if angle_rad.is_sign_negative() {
        angle_rad += 2.0 * PI;
    }
    if delta_x.is_sign_negative() {
        angle_rad += PI;
    }
    return angle_rad;
}

fn line_mesh(line_width: f32, length: f32, angle_rad: f32) -> Mesh {
    let lw_half = line_width / 2.0f32;
    // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
    Mesh::new(PrimitiveTopology::TriangleList)
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [-lw_half * angle_rad.sin(), lw_half * angle_rad.cos(), 0.0],
                [lw_half * angle_rad.sin(), -lw_half * angle_rad.cos(), 0.0],
                [
                    length * angle_rad.cos() + lw_half * angle_rad.sin(),
                    length * angle_rad.sin() - lw_half * angle_rad.cos(),
                    0.0,
                ],
                [
                    length * angle_rad.cos() - lw_half * angle_rad.sin(),
                    length * angle_rad.sin() + lw_half * angle_rad.cos(),
                    0.0,
                ],
            ],
        )
        // Assign a UV coordinate to each vertex.
        // .with_inserted_attribute(
        //     Mesh::ATTRIBUTE_UV_0,
        //     vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]],
        // )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_indices(Some(Indices::U32(vec![
            // First triangle
            0, 3, 1, // Second triangle
            1, 3, 2,
        ])))
}

fn arc_mesh(line_width: f32) -> Mesh {
    todo!()
}
fn _create_simple_parallelogram() -> Mesh {
    // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
    // https://github.com/bevyengine/bevy/blob/main/assets/docs/Mesh.png
    Mesh::new(PrimitiveTopology::TriangleList)
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [0.0, 0.0, 0.0],
                [1.0, 2.0, 0.0],
                [2.0, 2.0, 0.0],
                [1.0, 0.0, 0.0],
            ],
        )
        // Assign a UV coordinate to each vertex.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]],
        )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_indices(Some(Indices::U32(vec![
            // First triangle
            0, 3, 1, // Second triangle
            1, 3, 2,
        ])))
}
