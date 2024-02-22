pub mod arcs;
pub mod circles;
pub mod lines;
pub mod points;
pub mod texts;

use robusta_core::*;
use robusta_dxf::parse::line::{to_points, to_rentity};

use crate::arcs::draw_arcs;
use crate::circles::draw_circles;
use crate::lines::draw_lines;
use crate::points::draw_points;
use crate::texts::draw_texts;
use crate::*;

use self::rselection::Selection;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum REntity {
    Arc(arc::Arc),
    Circle(circle::Circle),
    Line(line::Line),
    Point(point::Point),
    Text(text::Text),
}

pub fn draw_first(
    ui_state: Res<self::uistate::UiState>,
    entity_mapping: ResMut<self::entitymapping::EntityMapping>,
    mut co: Commands,
    mut me: ResMut<Assets<Mesh>>,
    mut ma: ResMut<Assets<ColorMaterial>>,
) {
    let entity_mapping = entity_mapping.into_inner();

    for (_file_name, info) in &ui_state.loaded_files {
        match info {
            robusta_dxf::open::InterchangeFormat::DXF(drawing) => {
                spawn_from_dxf(&mut co, &mut me, &mut ma, drawing)
            }
        }
        // for (index, entity) in file.entities.iter().enumerate() {
        //     match entity {
        //         robusta_core::RobustaEntity::Arc(specific) => {
        // draw_arcs(&mut entity_package, specific, entity_mapping, index)
        //         }
        //         robusta_core::RobustaEntity::Circle(specific) => {
        //             draw_circles(&mut entity_package, specific, entity_mapping, index)
        //         }
        //         robusta_core::RobustaEntity::Line(specific) => {
        // draw_lines(&mut entity_package, specific, entity_mapping, index)
        //         }
        //         robusta_core::RobustaEntity::Point(specific) => {
        // draw_points(&mut entity_package, specific, entity_mapping, index)
        //         }
        //         robusta_core::RobustaEntity::Text(specific) => {
        //             draw_texts(&mut entity_package, specific, entity_mapping, index)
        //         }
        //     }
        // }
    }
}

fn spawn_from_dxf(
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    drawing: &dxf::Drawing,
) {
    for (ix, e) in drawing.entities().enumerate() {
        match &e.specific {
            dxf::entities::EntityType::Face3D(_) => todo!(),
            dxf::entities::EntityType::Solid3D(_) => todo!(),
            dxf::entities::EntityType::ProxyEntity(_) => todo!(),
            dxf::entities::EntityType::Arc(_) => todo!(),
            dxf::entities::EntityType::ArcAlignedText(_) => todo!(),
            dxf::entities::EntityType::AttributeDefinition(_) => todo!(),
            dxf::entities::EntityType::Attribute(_) => todo!(),
            dxf::entities::EntityType::Body(_) => todo!(),
            dxf::entities::EntityType::Circle(_) => todo!(),
            dxf::entities::EntityType::RotatedDimension(_) => todo!(),
            dxf::entities::EntityType::RadialDimension(_) => todo!(),
            dxf::entities::EntityType::DiameterDimension(_) => todo!(),
            dxf::entities::EntityType::AngularThreePointDimension(_) => todo!(),
            dxf::entities::EntityType::OrdinateDimension(_) => todo!(),
            dxf::entities::EntityType::Ellipse(_) => todo!(),
            dxf::entities::EntityType::Helix(_) => todo!(),
            dxf::entities::EntityType::Image(_) => todo!(),
            dxf::entities::EntityType::Insert(_) => todo!(),
            dxf::entities::EntityType::Leader(_) => todo!(),
            dxf::entities::EntityType::Light(_) => todo!(),
            dxf::entities::EntityType::Line(sp) => spawn_line(sp, co, me, ma, ix),
            dxf::entities::EntityType::LwPolyline(_) => todo!(),
            dxf::entities::EntityType::MLine(_) => todo!(),
            dxf::entities::EntityType::MText(_) => todo!(),
            dxf::entities::EntityType::OleFrame(_) => todo!(),
            dxf::entities::EntityType::Ole2Frame(_) => todo!(),
            dxf::entities::EntityType::ModelPoint(_) => todo!(),
            dxf::entities::EntityType::Polyline(_) => todo!(),
            dxf::entities::EntityType::Ray(_) => todo!(),
            dxf::entities::EntityType::Region(_) => todo!(),
            dxf::entities::EntityType::RText(_) => todo!(),
            dxf::entities::EntityType::Section(_) => todo!(),
            dxf::entities::EntityType::Seqend(_) => todo!(),
            dxf::entities::EntityType::Shape(_) => todo!(),
            dxf::entities::EntityType::Solid(_) => todo!(),
            dxf::entities::EntityType::Spline(_) => todo!(),
            dxf::entities::EntityType::Text(_) => todo!(),
            dxf::entities::EntityType::Tolerance(_) => todo!(),
            dxf::entities::EntityType::Trace(_) => todo!(),
            dxf::entities::EntityType::DgnUnderlay(_) => todo!(),
            dxf::entities::EntityType::DwfUnderlay(_) => todo!(),
            dxf::entities::EntityType::PdfUnderlay(_) => todo!(),
            dxf::entities::EntityType::Vertex(_) => todo!(),
            dxf::entities::EntityType::Wipeout(_) => todo!(),
            dxf::entities::EntityType::XLine(_) => todo!(),
        }
    }
}

fn spawn_line(
    sp: &dxf::entities::Line,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    ix: usize,
) {
    let sp = to_rentity(sp);
    co.spawn((
        MaterialMesh2dBundle {
            mesh: me.add(shape::Circle::new(0.5).into()).into(),
            material: ma.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                sp.definition[0].coordinates.x,
                sp.definition[0].coordinates.y,
                ix as f32,
            )),
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Select>>::send_event::<Selection>(),
        On::<Pointer<Deselect>>::send_event::<Selection>(),
    ));
}
