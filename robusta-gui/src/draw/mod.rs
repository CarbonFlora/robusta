pub mod arcs;
pub mod circles;
pub mod lines;
pub mod points;
pub mod texts;

use crate::arcs::draw_arcs;
use crate::circles::draw_circles;
use crate::lines::draw_lines;
use crate::points::draw_points;
use crate::texts::draw_texts;
use crate::*;

pub fn draw_first(
    ui_state: Res<self::uistate::UiState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for file in &ui_state.loaded_files {
        draw_points(&mut commands, &mut meshes, &mut materials, file.1);
        draw_lines(&mut commands, &mut meshes, &mut materials, file.1);
        draw_arcs(&mut commands, &mut meshes, &mut materials, file.1);
        draw_circles(&mut commands, &mut meshes, &mut materials, file.1);
        draw_texts(&mut commands, file.1);
    }
}
