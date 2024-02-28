use robusta_core::{arc::Arc, circle::Circle, line::Line, point::Point};

use super::*;

pub struct SnapPlugin;
impl bevy::app::Plugin for SnapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateSnapPoints>()
            .add_event::<Snap>()
            .add_systems(Update, update_snap_points);
    }
}

#[derive(Debug, Component)]
pub struct SnapPoint;

#[derive(Event, Clone, Debug, PartialEq)]
pub struct Snap(pub Entity, pub bool);

#[derive(Event, Clone, Debug, PartialEq)]
pub struct UpdateSnapPoints(pub bool);

impl From<ListenerInput<Pointer<Out>>> for Snap {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        Snap(event.target, false)
    }
}

impl From<ListenerInput<Pointer<Over>>> for Snap {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        Snap(event.target, true)
    }
}

impl UiState {
    pub fn toggle_snap(&mut self, snap: &Snaps) {
        let ss = &mut self.cad_state.object_snapping;
        match snap {
            Snaps::Endpoint => flip(&mut ss.endpoint),
            Snaps::Midpoint => flip(&mut ss.midpoint),
            Snaps::Nthpoint(div) => {
                flip(&mut ss.nthpoint.0);
                if div > &0usize {
                    ss.nthpoint.1 = *div;
                }
            }
            Snaps::Intersection => flip(&mut ss.intersection),
            Snaps::Perpendicular => flip(&mut ss.perpendicular),
            Snaps::Tangent => flip(&mut ss.tangent),
        }
    }

    pub fn toggle_snap_off(&mut self, ewrsp: &mut EventWriter<UpdateSnapPoints>) {
        ewrsp.send(UpdateSnapPoints(false));
        self.cad_state.object_snapping = SnapSettings::default();
    }

    pub fn reload_snap_point(
        &self,
        res: &Query<&REntity, With<Selected>>,
        esp: &Query<Entity, With<SnapPoint>>,
        co: &mut Commands,
        me: &mut ResMut<Assets<Mesh>>,
        ma: &mut ResMut<Assets<ColorMaterial>>,
        tzi: &mut ResMut<TopZLayer>,
    ) {
        despawn_all_snap_points(co, esp);
        spawn_all_snap_points(self, res, co, me, ma, tzi);
    }
}

pub fn despawn_all_snap_points(co: &mut Commands, esp: &Query<Entity, With<SnapPoint>>) {
    for e in esp.iter() {
        co.entity(e).despawn();
    }
}

fn spawn_all_snap_points(
    us: &UiState,
    res: &Query<&REntity, With<Selected>>,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    tzi: &mut ResMut<TopZLayer>,
) {
    let ss = &us.cad_state.object_snapping;
    let mut vp = Vec::new();
    for re in res.iter() {
        match re {
            REntity::Arc(sp) => arc_snaps(sp, ss, &mut vp),
            REntity::Circle(sp) => circle_snaps(sp, ss, &mut vp),
            REntity::Line(sp) => line_snaps(sp, ss, &mut vp),
            REntity::Point(sp) => (),
            REntity::Text(sp) => (),
        }
    }
    ssp(vp, co, me, ma, tzi);
}

fn ssp(
    vp: Vec<Point>,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    xi: &mut ResMut<TopZLayer>,
) {
    for po in vp {
        co.spawn((
            MaterialMesh2dBundle {
                mesh: me.add(shape::Circle::new(0.2).into()).into(),
                material: ma.add(ColorMaterial::from(Color::ORANGE)),
                transform: Transform::from_translation(Vec3::new(
                    po.coordinates.x,
                    po.coordinates.y,
                    xi.top() as f32,
                )),
                ..default()
            },
            SnapPoint,
            REntity::Point(point::Point::new(
                po.coordinates.x,
                po.coordinates.y,
                po.coordinates.z,
            )),
            On::<Pointer<Over>>::send_event::<Snap>(),
            On::<Pointer<Out>>::send_event::<Snap>(),
        ));
    }
}

fn arc_snaps(sp: &Arc, ss: &SnapSettings, vp: &mut Vec<Point>) {
    if ss.endpoint {
        vp.extend(sp.endpoints());
    }
    if ss.midpoint {
        vp.extend(sp.midpoints());
        vp.extend(sp.center());
    }
    if ss.nthpoint.0 {
        vp.extend(sp.nthpoints(ss.nthpoint.1));
    }
}

fn circle_snaps(sp: &Circle, ss: &SnapSettings, vp: &mut Vec<Point>) {
    if ss.midpoint {
        vp.extend(sp.center());
    }
}

fn line_snaps(sp: &Line, ss: &SnapSettings, vp: &mut Vec<Point>) {
    if ss.endpoint {
        vp.extend(sp.endpoints());
    }
    if ss.midpoint {
        vp.extend(sp.midpoints());
    }
}

#[allow(clippy::complexity)]
fn update_snap_points(
    mut errsp: EventReader<UpdateSnapPoints>,
    us: Res<UiState>,
    res: Query<&REntity, With<Selected>>,
    esp: Query<Entity, With<SnapPoint>>,
    mut co: Commands,
    mut me: ResMut<Assets<Mesh>>,
    mut ma: ResMut<Assets<ColorMaterial>>,
    mut tzi: ResMut<TopZLayer>,
) {
    if errsp.is_empty() {
        return;
    }
    for temp in errsp.read() {
        match temp.0 {
            true => us.reload_snap_point(&res, &esp, &mut co, &mut me, &mut ma, &mut tzi),
            false => despawn_all_snap_points(&mut co, &esp),
        }
    }
}
