use std::any::TypeId;

use bevy::prelude::*;
use bevy_asset::{ReflectAsset, UntypedAssetId};
use bevy_egui::EguiContext;
// use bevy_mod_picking::backends::egui::EguiPointer;
// use bevy_mod_picking::prelude::*;
use bevy_reflect::TypeRegistry;
use bevy_window::PrimaryWindow;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

pub mod cad_term;
pub mod leaves;
pub mod uistate;
