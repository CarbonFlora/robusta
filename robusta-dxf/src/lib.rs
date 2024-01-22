use anyhow::Result;
use dxf::{entities::*, Drawing};
// use std::{io::Write, path::PathBuf};
use std::path::PathBuf;

use robusta_core::point::Point;

pub mod open;
pub mod parse;
pub mod wrapper;
