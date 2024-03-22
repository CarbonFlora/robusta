use self::selection::Selected;

use super::*;

pub struct SnapPlugin;
impl bevy::app::Plugin for SnapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SnapSettings::default())
            .add_event::<UpdateSnapPoints>()
            .add_event::<SnapTo>()
            .add_systems(Update, update_snap_points);
    }
}

#[derive(Debug, Component)]
pub struct SnapPoint;

#[derive(Event, Clone, Debug, PartialEq)]
pub struct SnapTo(pub Entity, pub bool);

#[derive(Event, Clone, Debug, PartialEq)]
pub struct UpdateSnapPoints(pub bool);

pub fn toggle_snap(
    ss: &mut ResMut<SnapSettings>,
    ost: &SnapType,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
) {
    match ost {
        SnapType::Endpoint => flip(&mut ss.endpoint),
        SnapType::Midpoint => flip(&mut ss.midpoint),
        SnapType::Nthpoint(div) => ss.flip_nth(div),

        SnapType::Intersection => flip(&mut ss.intersection),
        SnapType::Perpendicular => flip(&mut ss.perpendicular),
        SnapType::Tangent => flip(&mut ss.tangent),
    };

    ewrsp.send(UpdateSnapPoints(true));
}

pub fn reload_snap_point(
    //Util
    ss: &Res<SnapSettings>,
    res: &Query<&REntity, With<Selected>>,
    esp: &Query<Entity, With<SnapPoint>>,
    //Output
    ewre: &mut EventWriter<REntity>,
    co: &mut Commands,
) {
    despawn_all_snap_points(co, esp);
    spawn_simple_snap_points(ss, res, ewre);
    spawn_shared_snap_points(ss, res, ewre);
}

pub fn despawn_all_snap_points(co: &mut Commands, esp: &Query<Entity, With<SnapPoint>>) {
    for e in esp.iter() {
        co.entity(e).despawn();
    }
}

fn spawn_shared_snap_points(
    ss: &Res<SnapSettings>,
    res: &Query<&REntity, With<Selected>>,
    ewre: &mut EventWriter<REntity>,
) {
    //Implement intersection functions
    // point x line
    // line x line
    // line x arc
    // arc x arc
    // arc x point
    // circle x everything
    // Text is ignored.
}

fn spawn_simple_snap_points(
    ss: &Res<SnapSettings>,
    res: &Query<&REntity, With<Selected>>,
    ewre: &mut EventWriter<REntity>,
) {
    let mut vp = Vec::new();
    for re in res.iter() {
        match re {
            REntity::Arc(sp) => arc_snaps(sp, ss, &mut vp),
            REntity::Circle(sp) => circle_snaps(sp, ss, &mut vp),
            REntity::Line(sp) => line_snaps(sp, ss, &mut vp),
            REntity::Point(sp) => point_snap(sp, ss, &mut vp),
            REntity::Text(_) => (),
            REntity::SnapPoint(sp) => vp.push(sp.clone()),
            REntity::PhantomPoint => (),
            REntity::PhantomStatic(_) => (),
        }
    }
    ewre.send_batch(vp.iter().map(|p| REntity::SnapPoint(p.as_snap())));
}

fn arc_snaps(sp: &arc::Arc, ss: &SnapSettings, vp: &mut Vec<point::Point>) {
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

fn circle_snaps(sp: &circle::Circle, ss: &SnapSettings, vp: &mut Vec<point::Point>) {
    if ss.midpoint {
        vp.extend(sp.center());
    }
    if ss.nthpoint.0 {
        vp.extend(sp.nthpoints(ss.nthpoint.1));
    }
}

fn line_snaps(sp: &line::Line, ss: &SnapSettings, vp: &mut Vec<point::Point>) {
    if ss.endpoint {
        vp.extend(sp.endpoints());
    }
    if ss.midpoint {
        vp.extend(sp.midpoints());
    }
    if ss.nthpoint.0 {
        vp.extend(sp.nthpoints(ss.nthpoint.1));
    }
}

fn point_snap(sp: &point::Point, ss: &SnapSettings, vp: &mut Vec<point::Point>) {
    if ss.endpoint || ss.midpoint {
        vp.push(sp.clone());
    }
}

#[allow(clippy::complexity)]
fn update_snap_points(
    mut errsp: EventReader<UpdateSnapPoints>,
    ss: Res<SnapSettings>,
    res: Query<&REntity, With<Selected>>,
    esp: Query<Entity, With<SnapPoint>>,
    mut ewre: EventWriter<REntity>,
    mut co: Commands,
) {
    for temp in errsp.read() {
        match temp.0 {
            true => reload_snap_point(&ss, &res, &esp, &mut ewre, &mut co),
            // true => us.reload_snap_point(&res, &esp, &mut co, &mut me, &mut ma, &mut tzi),
            false => despawn_all_snap_points(&mut co, &esp),
        }
    }
}

#[derive(Bundle)]
pub struct SnapBundle {
    a: SnapPoint,
    b: On<Pointer<Over>>,
    c: On<Pointer<Out>>,
}

impl Default for SnapBundle {
    fn default() -> Self {
        SnapBundle {
            a: SnapPoint,
            b: On::<Pointer<Over>>::send_event::<SnapTo>(),
            c: On::<Pointer<Out>>::send_event::<SnapTo>(),
        }
    }
}

impl std::fmt::Display for SnapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            SnapType::Endpoint => "Endpoint",
            SnapType::Midpoint => "Midpoint",
            SnapType::Nthpoint(_) => "Nthpoint",
            SnapType::Intersection => "Intersection",
            SnapType::Perpendicular => "Perpendicular",
            SnapType::Tangent => "Tangent",
        };
        f.write_fmt(format_args!("{}", a))
    }
}

impl From<ListenerInput<Pointer<Out>>> for SnapTo {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        SnapTo(event.target, false)
    }
}

impl From<ListenerInput<Pointer<Over>>> for SnapTo {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        SnapTo(event.target, true)
    }
}
