use self::rselection::{update_selection, Selected};
use self::snap::SnapPlugin;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, window};
use bevy_egui::EguiContext;
use bevy_mod_picking::prelude::*;
use bevy_window::PrimaryWindow;
use dxf::entities::EntityType;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use parse::dxf::arc::spawn_arc;
use parse::dxf::circle::spawn_circle;
use parse::dxf::line::spawn_line;
use parse::dxf::text::spawn_text;
use robusta_core::*;

pub mod app;
pub mod entitymapping;
pub mod keystrokes;
pub mod leaves;
pub mod parse;
pub mod rcadplugin;
pub mod rselection;
pub mod snap;
pub mod uistate;
pub mod update_act;
pub mod viewport_ui;

// use crate::entitymapping::*;
use crate::keystrokes::*;
use crate::leaves::inspection::view_inspection;
use crate::leaves::term::update_terminal_egui;
use crate::uistate::*;
use crate::update_act::*;
use crate::viewport_ui::*;

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
    mut co: Commands,
    mut me: ResMut<Assets<Mesh>>,
    mut ma: ResMut<Assets<ColorMaterial>>,
) {
    for (_file_name, info) in &ui_state.loaded_files {
        match info {
            InterchangeFormat::DXF(drawing) => spawn_from_dxf(&mut co, &mut me, &mut ma, drawing),
        }
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
            EntityType::Face3D(_) => todo!(),
            EntityType::Solid3D(_) => todo!(),
            EntityType::ProxyEntity(_) => todo!(),
            EntityType::Arc(sp) => spawn_arc(sp, co, me, ma, ix),
            EntityType::ArcAlignedText(_) => todo!(),
            EntityType::AttributeDefinition(_) => todo!(),
            EntityType::Attribute(_) => todo!(),
            EntityType::Body(_) => todo!(),
            EntityType::Circle(sp) => spawn_circle(sp, co, me, ma, ix),
            EntityType::RotatedDimension(_) => todo!(),
            EntityType::RadialDimension(_) => todo!(),
            EntityType::DiameterDimension(_) => todo!(),
            EntityType::AngularThreePointDimension(_) => todo!(),
            EntityType::OrdinateDimension(_) => todo!(),
            EntityType::Ellipse(_) => todo!(),
            EntityType::Helix(_) => todo!(),
            EntityType::Image(_) => todo!(),
            EntityType::Insert(_) => todo!(),
            EntityType::Leader(_) => todo!(),
            EntityType::Light(_) => todo!(),
            EntityType::Line(sp) => spawn_line(sp, co, me, ma, ix),
            EntityType::LwPolyline(_) => todo!(),
            EntityType::MLine(_) => todo!(),
            EntityType::MText(_) => todo!(),
            EntityType::OleFrame(_) => todo!(),
            EntityType::Ole2Frame(_) => todo!(),
            EntityType::ModelPoint(_) => todo!(),
            EntityType::Polyline(_) => todo!(),
            EntityType::Ray(_) => todo!(),
            EntityType::Region(_) => todo!(),
            EntityType::RText(_) => todo!(),
            EntityType::Section(_) => todo!(),
            EntityType::Seqend(_) => todo!(),
            EntityType::Shape(_) => todo!(),
            EntityType::Solid(_) => todo!(),
            EntityType::Spline(_) => todo!(),
            EntityType::Text(sp) => spawn_text(sp, co, me, ma, ix),
            EntityType::Tolerance(_) => todo!(),
            EntityType::Trace(_) => todo!(),
            EntityType::DgnUnderlay(_) => todo!(),
            EntityType::DwfUnderlay(_) => todo!(),
            EntityType::PdfUnderlay(_) => todo!(),
            EntityType::Vertex(_) => todo!(),
            EntityType::Wipeout(_) => todo!(),
            EntityType::XLine(_) => todo!(),
        }
    }
}
