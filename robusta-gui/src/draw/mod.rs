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
    entity_mapping: ResMut<self::entitymapping::EntityMapping>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut entity_package = (&mut commands, &mut meshes, &mut materials);
    let entity_mapping = entity_mapping.into_inner();

    for file in &ui_state.loaded_files {
        draw_points(&mut entity_package, file.1, entity_mapping);
        draw_lines(&mut entity_package, file.1, entity_mapping);
        draw_arcs(&mut entity_package, file.1, entity_mapping);
        draw_circles(&mut entity_package, file.1, entity_mapping);
        draw_texts(&mut entity_package, file.1, entity_mapping);
    }
}
