use bevy::{prelude::*, utils::HashMap, window};
use bevy_egui::EguiContext;
use bevy_mod_picking::prelude::*;
use bevy_window::PrimaryWindow;

use egui_dock::{DockArea, DockState, NodeIndex, Style};
use robusta_dxf::wrapper::DXFWrapper;

pub mod app;
pub mod draw;
pub mod keystrokes;
pub mod leaves;
pub mod test;
pub mod uistate;
pub mod viewport_ui;

use crate::draw::*;
use crate::keystrokes::*;
use crate::leaves::inspection::view_inspection;
use crate::leaves::keystrokes::view_pressed_keys;
use crate::leaves::points::view_points;
use crate::leaves::term::open_term_egui;
use crate::uistate::*;
use crate::viewport_ui::*;
