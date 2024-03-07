use robusta_core::{line::Line, point::Point};

use self::{
    parse::dxf::line::spawn_line_mesh,
    phantom::{despawn_all_phantoms, RPhantomPointer},
    rselection::Selection,
};

use super::*;

pub struct ConstructionPlugin;
impl bevy::app::Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConstructionBuffer::new())
            .add_event::<ConstructionInput>()
            .add_systems(Update, update_queue);
        // .add_systems(Update, update_construction);
    }
}

#[derive(Debug, Event, Clone, PartialEq)]
pub struct ConstructionInput {
    pub coords: Vec3,
}

#[derive(Debug, Clone, Default)]
pub enum InputType {
    #[default]
    Cursor,
    Coordinates,
}

/// This is a buffer to store what the next pointer action should do.
#[derive(Debug, Resource, Default)]
pub struct ConstructionBuffer {
    buf: Vec<ConstructionInput>,
    build: Option<ConstructType>,
}

impl ConstructionBuffer {
    pub fn new() -> Self {
        self::default()
    }

    pub fn reset(&mut self) {
        self.buf = Vec::new();
        self.build = None;
    }
}

#[derive(Debug, Component)]
pub struct RConstructionEntity;

#[derive(Debug)]
pub enum ConstructType {
    Arc,
    Circle,
    Line,
    Point,
    Text,
}

#[allow(clippy::too_many_arguments)]
fn update_queue(
    mut erra: EventReader<ConstructionInput>,
    mut rmcb: ResMut<ConstructionBuffer>,
    mut co: Commands,
    mut me: ResMut<Assets<Mesh>>,
    mut ma: ResMut<Assets<ColorMaterial>>,
    mut tzi: ResMut<TopZLayer>,
    mut ewrsp: EventWriter<UpdateSnapPoints>,
    ewp: Query<Entity, With<RPhantomPointer>>,
) {
    if erra.is_empty() {
        return;
    }
    for ci in erra.read() {
        rmcb.buf.push(ci.clone());
        rmcb.buf.dedup();
    }
    erra.clear();

    match rmcb
        .build
        .as_ref()
        .expect("Encountered construction with no build criteria.")
    {
        ConstructType::Arc => todo!(),
        ConstructType::Circle => todo!(),
        ConstructType::Line => {
            if rmcb.buf.len() == 2 {
                let pt1 = rmcb.buf[0].coords;
                let pt2 = rmcb.buf[1].coords;
                let sp = Line::new([
                    Point::new(pt1.x, pt1.y, pt1.z),
                    Point::new(pt2.x, pt2.y, pt2.z),
                ]);
                canonize_line(sp, &mut co, &mut me, &mut ma, &mut tzi);
                ewrsp.send(UpdateSnapPoints(false));
                despawn_all_phantoms(&mut co, &ewp);
                rmcb.into_inner().reset();
            }
        }
        ConstructType::Point => {
            if rmcb.buf.len() == 1 {
                let pt1 = rmcb.buf[0].coords;
                let sp = Point::new(pt1.x, pt1.y, pt1.z);
                canonize_point(sp, &mut co, &mut me, &mut ma, &mut tzi);
                ewrsp.send(UpdateSnapPoints(false));
                despawn_all_phantoms(&mut co, &ewp);
                rmcb.into_inner().reset();
            }
        }
        ConstructType::Text => todo!(),
    }
}

fn canonize_point(
    sp: Point,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    tzi: &mut TopZLayer,
) {
    co.spawn((
        MaterialMesh2dBundle {
            mesh: me.add(bevy::math::primitives::Circle::new(0.5)).into(),
            material: ma.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                sp.coordinates.x,
                sp.coordinates.y,
                tzi.top() as f32,
            )),
            ..default()
        },
        REntity::Point(sp),
        PickableBundle::default(),
        On::<Pointer<Select>>::send_event::<Selection>(),
        On::<Pointer<Deselect>>::send_event::<Selection>(),
    ));
}

fn canonize_line(
    sp: Line,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    tzi: &mut TopZLayer,
) {
    let id = spawn_line_mesh(sp, co, me, ma, tzi);
    co.entity(id).insert((
        PickableBundle::default(),
        On::<Pointer<Select>>::send_event::<Selection>(),
        On::<Pointer<Deselect>>::send_event::<Selection>(),
    ));
}

pub fn construct_point(
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    tzi: &mut TopZLayer,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
    rmcb: &mut ResMut<ConstructionBuffer>,
) {
    rmcb.build = Some(ConstructType::Point);
    ewrsp.send(UpdateSnapPoints(true));
    spawn_phantom_point(co, me, ma, tzi);
}

pub fn construct_line(
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    tzi: &mut TopZLayer,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
    rmcb: &mut ResMut<ConstructionBuffer>,
) {
    rmcb.build = Some(ConstructType::Line);
    ewrsp.send(UpdateSnapPoints(true));
    spawn_phantom_point(co, me, ma, tzi);
}
