use bevy::prelude::*;
use bevy::{
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::PickableBundle;
use robusta_core::point::Point;
use std::f32::consts::PI;

pub mod arc;
pub mod circle;
pub mod line;
pub mod text;

use crate::rselection::Selection;
use crate::REntity;
use crate::TopZLayer;
