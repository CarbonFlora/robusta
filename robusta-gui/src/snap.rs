use super::*;

pub struct SnapPlugin;
impl bevy::app::Plugin for SnapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_snap);
    }
}

#[derive(Debug, Component)]
pub struct SnapPoint;

pub fn spawn_snaps(res: Query<&REntity, With<SnapPoint>>, us: Res<UiState>) {
    let ss = &us.cad_state.object_snapping;
    for re in res.iter() {
        match re {
            REntity::Arc(sp) => (),
            REntity::Circle(sp) => (),
            REntity::Line(sp) => (),
            REntity::Point(sp) => (),
            REntity::Text(sp) => (),
        }
    }
}

fn update_snap(res: Query<&REntity, With<Selected>>, us: Res<UiState>) {
    let ss = &us.cad_state.object_snapping;
    for re in res.iter() {}
}
