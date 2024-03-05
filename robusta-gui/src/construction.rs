use robusta_core::point::Point;

use self::{
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

#[derive(Debug, Event, Clone)]
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
    build: Option<REntity>,
}

impl ConstructionBuffer {
    pub fn new() -> Self {
        self::default()
    }
}

#[derive(Debug, Component)]
pub struct RConstructionEntity;

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
    }
    let mut rmcbi = rmcb.buf.iter();
    match rmcb
        .build
        .as_ref()
        .expect("Encountered construction with no build criteria.")
    {
        REntity::Arc(_) => todo!(),
        REntity::Circle(_) => todo!(),
        REntity::Line(_) => todo!(),
        REntity::Point(_) => {
            if rmcb.buf.len() == 1 {
                let pt1 = rmcbi.next().unwrap().coords;
                let sp = Point::new(pt1.x, pt1.y, pt1.z);
                canonize_point(sp, &mut co, &mut me, &mut ma, &mut tzi);
                ewrsp.send(UpdateSnapPoints(false));
                despawn_all_phantoms(&mut co, &ewp);
            }
        }
        REntity::Text(_) => todo!(),
    }
}

// &mut self,
//         co: &mut Commands,
//         ewp: &Query<Entity, With<RPhantomPointer>>,
//         ewrsp: &mut EventWriter<UpdateSnapPoints>,
//     ) {

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
            material: ma.add(ColorMaterial::from(Color::CYAN)),
            transform: Transform::from_translation(Vec3::new(
                sp.coordinates.x,
                sp.coordinates.x,
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

// fn update_construction(rcb: Res<ConstructionBuffer>) {
//     if let Some(ci) = rcb.0.first() {
//         match ci.input_type {
//             InputType::Cursor => todo!(),
//             InputType::Coordinates => todo!(),
//         }
//     }
// }

pub fn construct_point(
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    tzi: &mut TopZLayer,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
    rmcb: &mut ResMut<ConstructionBuffer>,
) {
    rmcb.build = Some(REntity::Point(Point::new(0., 0., 0.)));
    ewrsp.send(UpdateSnapPoints(true));
    spawn_phantom_point(co, me, ma, tzi);
}
