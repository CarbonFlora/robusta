use core::panic;
use std::f32::consts::PI;

use bevy::{
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::{prelude::*, PickableBundle};
use nalgebra::Matrix3;
use robusta_core::point::Point;

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
        draw_circles(&mut commands, &mut meshes, &mut materials, file.1);
        draw_text(&mut commands, file.1);
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
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(arc_mesh(line_width, arc.definition).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 7.)),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectionInstance>(),
            On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        ));
    }
}

fn draw_circles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    wrapper: &DXFWrapper,
) {
    let line_width = 0.3f32;
    for circle in &wrapper.circles {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(circle_mesh(line_width, circle.definition).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 7.)),
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
    return angle_rad % (2. * PI);
}

fn line_mesh(line_width: f32, length: f32, angle_rad: f32) -> Mesh {
    let lw_half = line_width / 2.0f32;
    Mesh::new(PrimitiveTopology::TriangleList)
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
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
        )
        .with_indices(Some(Indices::U32(vec![0, 3, 1, 1, 3, 2])))
}

fn arc_mesh(line_width: f32, definition: [Point; 3]) -> Mesh {
    let lw_half = line_width / 2.0f32;
    let num_segments = 30u32;
    let vertexes: Vec<[f32; 3]> = arc_vertexes(num_segments, definition, lw_half);
    let triangle_indexes: Vec<u32> = arc_indexes(num_segments);

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; vertexes.len()])
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertexes)
        .with_indices(Some(Indices::U32(triangle_indexes)))
}

fn circle_mesh(line_width: f32, definition: [Point; 2]) -> Mesh {
    let lw_half = line_width / 2.0f32;
    let num_segments = 30u32;
    let vertexes: Vec<[f32; 3]> = circle_vertexes(num_segments, definition, lw_half);
    let triangle_indexes: Vec<u32> = arc_indexes(num_segments);

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; vertexes.len()])
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertexes)
        .with_indices(Some(Indices::U32(triangle_indexes)))
}

fn arc_vertexes(num_segments: u32, definition: [Point; 3], lw_half: f32) -> Vec<[f32; 3]> {
    let mut vertexes = Vec::new();
    let (radius, center) = circle_specs(definition);
    let start_angle_rad = angle_full_circle(
        definition[0].coordinates.x - center[0],
        definition[0].coordinates.y - center[1],
    );
    let end_angle_rad = angle_full_circle(
        definition[1].coordinates.x - center[0],
        definition[1].coordinates.y - center[1],
    );

    let mut angle = (end_angle_rad - start_angle_rad).abs();
    if end_angle_rad < start_angle_rad {
        angle = (2. * PI) - angle;
    }
    let angle_increment = angle / num_segments as f32;

    for i in 0..=num_segments {
        let angle_offset = start_angle_rad + angle_increment * i as f32;

        let x_outer = center[0] + (radius + lw_half) * (angle_offset).cos();
        let y_outer = center[1] + (radius + lw_half) * (angle_offset).sin();
        let x_inner = center[0] + (radius - lw_half) * (angle_offset).cos();
        let y_inner = center[1] + (radius - lw_half) * (angle_offset).sin();

        vertexes.push([x_outer, y_outer, 0.]);
        vertexes.push([x_inner, y_inner, 0.]);
    }

    return vertexes;
}

fn circle_vertexes(num_segments: u32, definition: [Point; 2], lw_half: f32) -> Vec<[f32; 3]> {
    let mut vertexes = Vec::new();
    let center = [definition[1].coordinates.x, definition[1].coordinates.y];
    let radius = (definition[0].coordinates.x - definition[1].coordinates.x).abs();
    let angle_increment = (2. * PI) / num_segments as f32;

    for i in 0..=num_segments {
        let angle_offset = angle_increment * i as f32;

        let x_outer = center[0] + (radius + lw_half) * (angle_offset).cos();
        let y_outer = center[1] + (radius + lw_half) * (angle_offset).sin();
        let x_inner = center[0] + (radius - lw_half) * (angle_offset).cos();
        let y_inner = center[1] + (radius - lw_half) * (angle_offset).sin();

        vertexes.push([x_outer, y_outer, 0.]);
        vertexes.push([x_inner, y_inner, 0.]);
    }

    return vertexes;
}

fn arc_indexes(num_segments: u32) -> Vec<u32> {
    let mut a = Vec::new();

    for i in 0..(num_segments * 2) {
        a.extend(vec![i, i + 1, i + 2]);
    }

    return a;
}

fn circle_specs(definition: [Point; 3]) -> (f32, [f32; 3]) {
    let i_11 = definition[0].coordinates.x.powi(2) + definition[0].coordinates.y.powi(2);
    let i_21 = definition[1].coordinates.x.powi(2) + definition[1].coordinates.y.powi(2);
    let i_31 = definition[2].coordinates.x.powi(2) + definition[2].coordinates.y.powi(2);

    let m_14 = Matrix3::new(
        i_11,
        definition[0].coordinates.x,
        definition[0].coordinates.y,
        i_21,
        definition[1].coordinates.x,
        definition[1].coordinates.y,
        i_31,
        definition[2].coordinates.x,
        definition[2].coordinates.y,
    )
    .determinant();
    let m_13 = Matrix3::new(
        i_11,
        definition[0].coordinates.x,
        1.,
        i_21,
        definition[1].coordinates.x,
        1.,
        i_31,
        definition[2].coordinates.x,
        1.,
    )
    .determinant();
    let m_12 = Matrix3::new(
        i_11,
        definition[0].coordinates.y,
        1.,
        i_21,
        definition[1].coordinates.y,
        1.,
        i_31,
        definition[2].coordinates.y,
        1.,
    )
    .determinant();
    let m_11 = Matrix3::new(
        definition[0].coordinates.x,
        definition[0].coordinates.y,
        1.,
        definition[1].coordinates.x,
        definition[1].coordinates.y,
        1.,
        definition[2].coordinates.x,
        definition[2].coordinates.y,
        1.,
    )
    .determinant();

    if m_11 == 0. {
        panic!("not a circle.");
    }

    let x_center = 1. / 2. * m_12 / m_11;
    let y_center = -1. / 2. * m_13 / m_11;
    let radius = (x_center.powi(2) + y_center.powi(2) + m_14 / m_11).sqrt();
    return (radius, [x_center, y_center, 0.]);
}

#[test]
fn arc_v_test() {
    let p1 = Point::new(0., 0., 0.);
    let p2 = Point::new(4., 0., 0.);
    let p3 = Point::new(2., 2., 0.);
    let definition = [p1, p2, p3];
    let (radius, center) = circle_specs(definition);
    assert_eq!(radius, 2.);
    assert_eq!(center, [2., 0., 0.]);
    // let a = arc_vertexes(0.5, 5, b);
    let start_angle_rad = angle_full_circle(
        definition[0].coordinates.x - center[0],
        definition[0].coordinates.y - center[1],
    ) % (2. * PI);
    let end_angle_rad = angle_full_circle(
        definition[1].coordinates.x - center[0],
        definition[1].coordinates.y - center[1],
    ) % (2. * PI);
    assert_eq!((start_angle_rad * 1000.).trunc(), (PI * 1000.).trunc());
    assert_eq!(end_angle_rad, 0.);
}

#[test]
fn radius_eq_2() {
    let p1 = Point::new(1., 1., 0.);
    let p2 = Point::new(2., 4., 0.);
    let p3 = Point::new(5., 3., 0.);
    let a = [p1, p2, p3];
    assert_eq!(circle_specs(a).0, (5.0f32).sqrt());
    assert_eq!(circle_specs(a).1, [3., 2., 0.]);
}

fn draw_text(commands: &mut Commands, wrapper: &DXFWrapper) {
    for text in &wrapper.text {
        let text_body = Text::from_section(text.body.clone(), TextStyle::default());
        let origin = text.coordinates.xyz();

        commands.spawn((
            Text2dBundle {
                text: text_body,
                text_anchor: bevy::sprite::Anchor::Center,
                transform: Transform::from_translation(Vec3::new(origin[0], origin[1], origin[2])),
                text_layout_info: bevy::text::TextLayoutInfo::default(),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectionInstance>(),
            On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        ));
    }
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
