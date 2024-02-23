use robusta_core::{arc::Arc, point::Point};

use super::*;

pub struct SnapPlugin;
impl bevy::app::Plugin for SnapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_snap);
    }
}

#[derive(Debug, Component)]
pub struct SnapPoint;

pub fn spawn_snaps(
    res: Query<&REntity, With<SnapPoint>>,
    us: Res<UiState>,
    mut co: Commands,
    mut me: ResMut<Assets<Mesh>>,
    mut ma: ResMut<Assets<ColorMaterial>>,
) {
    let ss = &us.cad_state.object_snapping;
    for re in res.iter() {
        match re {
            REntity::Arc(sp) => arc_snaps(sp, ss, &co),
            REntity::Circle(sp) => (),
            REntity::Line(sp) => (),
            REntity::Point(sp) => (),
            REntity::Text(sp) => (),
        }
    }
}

fn spawn_snap_points(p: Vec<Point>, co: &Commands) {
    todo!()
}

fn arc_snaps(sp: &Arc, ss: &SnapSettings, co: &Commands) {
    if ss.endpoint {
        spawn_snap_points(sp.endpoints(), co);
    }
    if ss.midpoint {
        spawn_snap_points(sp.midpoints(), co);
    }
    if ss.center {
        spawn_snap_points(sp.center(), co);
    }
}

fn update_snap(res: Query<&REntity, With<Selected>>, us: Res<UiState>) {
    let ss = &us.cad_state.object_snapping;
    for re in res.iter() {}
}
