use self::{
    phantom::RPhantomStatic, point::Point, selection::PickableSelectionBundle, snap::SnapBundle,
};

use super::*;

pub struct UnsortedPlugin;
impl bevy::app::Plugin for UnsortedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TopZLayer::new())
            .add_event::<Act>()
            .add_event::<REntity>()
            .add_systems(PostStartup, draw_first)
            .add_systems(Update, update_spawn_rentities)
            .add_systems(PreUpdate, update_act);
    }
}

pub fn update_spawn_rentities(
    //Input
    mut erre: EventReader<REntity>,
    //Util
    mut me: ResMut<Assets<Mesh>>,
    mut ma: ResMut<Assets<ColorMaterial>>,
    mut tz: ResMut<TopZLayer>,
    //Output
    mut co: Commands,
) {
    for re in erre.read() {
        match re {
            REntity::Arc(sp) => {
                co.spawn((
                    REntity::Arc(sp.clone()),
                    sp.mesh(&mut me, &mut ma, &mut tz),
                    PickableSelectionBundle::default(),
                ));
            }
            REntity::Circle(sp) => {
                co.spawn((
                    REntity::Circle(sp.clone()),
                    sp.mesh(&mut me, &mut ma, &mut tz),
                    PickableSelectionBundle::default(),
                ));
            }
            REntity::Line(sp) => {
                co.spawn((
                    REntity::Line(sp.clone()),
                    sp.mesh(&mut me, &mut ma, &mut tz),
                    PickableSelectionBundle::default(),
                ));
            }
            REntity::Point(sp) => {
                co.spawn((
                    REntity::Point(sp.clone()),
                    sp.mesh(&mut me, &mut ma, &mut tz),
                    PickableSelectionBundle::default(),
                ));
            }
            REntity::Text(sp) => {
                co.spawn((
                    REntity::Text(sp.clone()),
                    sp.mesh(&mut me, &mut ma, &mut tz),
                    // sp.text_mesh(&mut tz),
                    PickableSelectionBundle::default(),
                ))
                .with_children(|builder| {
                    builder.spawn(sp.text_mesh(&mut tz));
                });
            }
            REntity::PhantomPoint => {
                co.spawn((
                    REntity::Point(Point::new(0., 0., 0.)),
                    Point::new(0., 0., 0.).mesh(&mut me, &mut ma, &mut tz),
                    RPhantomPointer,
                ));
            }
            REntity::SnapPoint(sp) => {
                co.spawn((
                    REntity::Point(sp.clone()),
                    sp.mesh(&mut me, &mut ma, &mut tz),
                    SnapBundle::default(),
                ));
            }
            REntity::PhantomStatic(sp) => {
                co.spawn((
                    REntity::Point(sp.clone()),
                    sp.mesh(&mut me, &mut ma, &mut tz),
                    RPhantomStatic,
                ));
            }
        };
    }
}
