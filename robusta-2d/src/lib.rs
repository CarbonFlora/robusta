use crate::viewportstate::ViewportState;
use bevy::prelude::*;
// use bevy_mod_picking::prelude::*;
use robusta_core::point::Point;
use robusta_dxf::open::open_from_path;
use robusta_dxf::wrapper::DXFWrapper;
use robusta_gui::uistate::{
    set_camera_viewport, show_ui_system, unfreeze_camera_viewport, UiState,
};

use crate::draw::draw_dxf;

pub mod app;
pub mod draw;
pub mod test;
pub mod viewportstate;
