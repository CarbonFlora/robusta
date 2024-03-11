use self::plugins::RCADPlugins;
use self::plugins::{
    construction::ConstructionBuffer,
    phantom::{despawn_all_phantoms, PhantomSnaps, RPhantomPointer},
    selection::Selected,
    snap::UpdateSnapPoints,
};
use self::plugins::{
    construction::{construct_line, construct_point, ConstructionInput},
    phantom::index_point,
    selection::deselect_all,
};
use ::bevy::render::{
    mesh::{Indices, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::hashbrown::HashMap;
use bevy::window;
use bevy_egui::EguiContext;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::{events::Pointer, selection::Deselect};
use bevy_window::PrimaryWindow;
use dxf::entities::EntityType;
use dxf::Drawing;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use parsedxf::{lwp_to_lines, pl_to_lines};
use primitives::*;
use std::path::PathBuf;
use std::str::SplitWhitespace;

use crate::{keystrokes::Act, uistate::UiState, Snaps};

pub mod app;
pub mod keystrokes;
pub mod leaves;
pub mod parsedxf;
pub mod plugins;
pub mod primitives;
pub mod uistate;
pub mod update_act;
pub mod viewport_ui;

use crate::keystrokes::*;
use crate::leaves::history::view_history;
use crate::leaves::inspection::view_inspection;
use crate::leaves::term::update_terminal_egui;
use crate::uistate::*;
use crate::update_act::*;
use crate::viewport_ui::*;

#[derive(Component, Debug, Clone, PartialEq, Event)]
pub enum REntity {
    Arc(primitives::arc::Arc),
    Circle(primitives::circle::Circle),
    Line(primitives::line::Line),
    Point(primitives::point::Point),
    Text(primitives::text::Text),
    PhantomPoint,
    SnapPoint(primitives::point::Point),
}

impl From<primitives::line::Line> for REntity {
    fn from(value: primitives::line::Line) -> Self {
        REntity::Line(value)
    }
}

impl From<primitives::arc::Arc> for REntity {
    fn from(value: primitives::arc::Arc) -> Self {
        REntity::Arc(value)
    }
}

impl From<primitives::circle::Circle> for REntity {
    fn from(value: primitives::circle::Circle) -> Self {
        REntity::Circle(value)
    }
}

impl From<primitives::text::Text> for REntity {
    fn from(value: primitives::text::Text) -> Self {
        REntity::Text(value)
    }
}

impl REntity {
    pub fn unwrap_point(&self) -> &point::Point {
        match self {
            REntity::Point(sp) => sp,
            _ => panic!("Unwrapped a non-point using custom unwrap."),
        }
    }

    pub fn unwrap_point_mut(&mut self) -> &mut point::Point {
        match self {
            REntity::Point(sp) => sp,
            _ => panic!("Unwrapped a non-point using custom unwrap."),
        }
    }
}

pub fn draw_first(ui_state: Res<UiState>, mut ewre: EventWriter<REntity>) {
    for (_file_name, info) in &ui_state.loaded_files {
        match info {
            InterchangeFormat::DXF(dr) => spawn_from_dxf(dr, &mut ewre),
        }
    }
}

fn spawn_from_dxf(
    //Input
    drawing: &dxf::Drawing,
    //Output
    ewre: &mut EventWriter<REntity>,
) {
    let mut vre: Vec<REntity> = Vec::new();

    for e in drawing.entities() {
        match &e.specific {
            EntityType::Face3D(_) => todo!(),
            EntityType::Solid3D(_) => todo!(),
            EntityType::ProxyEntity(_) => todo!(),
            EntityType::Arc(sp) => vre.push(sp.into()),
            EntityType::ArcAlignedText(_) => todo!(),
            EntityType::AttributeDefinition(_) => todo!(),
            EntityType::Attribute(_) => todo!(),
            EntityType::Body(_) => todo!(),
            EntityType::Circle(sp) => vre.push(sp.into()),
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
            EntityType::Line(sp) => vre.push(sp.into()),
            EntityType::LwPolyline(sp) => vre.extend(lwp_to_lines(sp)),
            EntityType::MLine(_) => todo!(),
            EntityType::MText(sp) => vre.push(sp.into()),
            EntityType::OleFrame(_) => todo!(),
            EntityType::Ole2Frame(_) => todo!(),
            EntityType::ModelPoint(_) => todo!(),
            EntityType::Polyline(sp) => vre.extend(pl_to_lines(sp)),
            EntityType::Ray(_) => todo!(),
            EntityType::Region(_) => todo!(),
            EntityType::RText(_) => todo!(),
            EntityType::Section(_) => todo!(),
            EntityType::Seqend(_) => todo!(),
            EntityType::Shape(_) => todo!(),
            EntityType::Solid(_) => todo!(),
            EntityType::Spline(_) => todo!(),
            EntityType::Text(sp) => vre.push(sp.into()),
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

    ewre.send_batch(vre);
}
