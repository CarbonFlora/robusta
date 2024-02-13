use bevy::{prelude::*, utils::HashMap, window};
use bevy::{
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use bevy_egui::EguiContext;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_window::PrimaryWindow;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use robusta_dxf::wrapper::DXFWrapper;
use std::f32::consts::PI;

pub mod app;
pub mod draw;
pub mod keystrokes;
pub mod leaves;
pub mod new_point;
pub mod test;
pub mod uistate;
pub mod update_act;
pub mod viewport_ui;

use crate::draw::*;
use crate::keystrokes::*;
use crate::leaves::inspection::view_inspection;
// use crate::leaves::keystrokes::view_pressed_keys;
use crate::leaves::points::view_points;
use crate::leaves::term::open_term_egui;
// use crate::new_point::*;
use crate::uistate::*;
use crate::update_act::*;
use crate::viewport_ui::*;
