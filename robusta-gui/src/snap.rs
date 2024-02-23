use robusta_core::{arc::Arc, point::Point};

use super::*;

pub struct SnapPlugin;
impl bevy::app::Plugin for SnapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Snap>().add_systems(Update, update_snap);
    }
}

#[derive(Debug, Component)]
pub struct SnapPoint;

#[derive(Event, Clone, Debug, PartialEq)]
pub struct Snap(Entity);

impl From<ListenerInput<Pointer<Select>>> for Snap {
    fn from(event: ListenerInput<Pointer<Select>>) -> Self {
        Snap(event.target)
    }
}

impl From<ListenerInput<Pointer<Over>>> for Snap {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        Snap(event.target)
    }
}

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
            REntity::Arc(sp) => arc_snaps(sp, ss, &mut co, &mut me, &mut ma),
            REntity::Circle(sp) => (),
            REntity::Line(sp) => (),
            REntity::Point(sp) => (),
            REntity::Text(sp) => (),
        }
    }
}

fn spawn_snap_points(
    vp: Vec<Point>,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
) {
    for po in vp {
        co.spawn((
            MaterialMesh2dBundle {
                mesh: me.add(shape::Circle::new(0.7).into()).into(),
                material: ma.add(ColorMaterial::from(Color::ORANGE)),
                transform: Transform::from_translation(Vec3::new(
                    po.coordinates.x,
                    po.coordinates.y,
                    999990.,
                )),
                ..default()
            },
            SnapPoint,
            PickableBundle::default(),
            On::<Pointer<Over>>::send_event::<Snap>(),
            On::<Pointer<Select>>::send_event::<Snap>(),
        ));
    }
}

fn arc_snaps(
    sp: &Arc,
    ss: &SnapSettings,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
) {
    if ss.endpoint {
        spawn_snap_points(sp.endpoints(), co, me, ma);
    }
    if ss.midpoint {
        spawn_snap_points(sp.midpoints(), co, me, ma);
    }
    if ss.center {
        spawn_snap_points(sp.center(), co, me, ma);
    }
}

fn update_snap(res: Query<&REntity, With<Selected>>, us: Res<UiState>) {
    let ss = &us.cad_state.object_snapping;
    for re in res.iter() {}
}
