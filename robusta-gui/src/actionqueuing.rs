use super::*;

pub struct ActionQueuingPlugin;
impl bevy::app::Plugin for ActionQueuingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RActionQueue::new())
            .add_event::<RAction>()
            .add_systems(Update, update_queue);
    }
}

#[derive(Debug, Event)]
pub struct RAction(Entity);

/// This is a buffer to store what the next pointer action should do.
#[derive(Debug, Resource, Default)]
pub struct RActionQueue(Vec<Entity>);

impl RActionQueue {
    pub fn new() -> Self {
        self::default()
    }
}

fn update_queue(mut erra: EventReader<RAction>, _t: ResMut<RActionQueue>) {
    for _ra in erra.read() {}
}
