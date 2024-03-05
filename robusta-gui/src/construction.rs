use super::*;

pub struct ConstructionPlugin;
impl bevy::app::Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConstructionBuffer::new())
            .add_event::<ConstructionInput>()
            .add_systems(Update, update_queue)
            .add_systems(Update, update_construction);
    }
}

#[derive(Debug, Event, Clone)]
pub struct ConstructionInput {
    input_type: InputType,
}

#[derive(Debug, Clone, Default)]
pub enum InputType {
    #[default]
    Cursor,
    Coordinates,
}

/// This is a buffer to store what the next pointer action should do.
#[derive(Debug, Resource, Default)]
pub struct ConstructionBuffer(Vec<ConstructionInput>);

impl ConstructionBuffer {
    pub fn new() -> Self {
        self::default()
    }
}

#[derive(Debug, Component)]
pub struct RConstructionEntity;

fn update_queue(mut erra: EventReader<ConstructionInput>, mut rmcb: ResMut<ConstructionBuffer>) {
    for ci in erra.read() {
        rmcb.0.push(ci.clone());
    }
}

fn update_construction(rcb: Res<ConstructionBuffer>) {
    // let a = rcb.0.first();
    if let Some(ci) = rcb.0.first() {
        match ci.input_type {
            InputType::Cursor => todo!(),
            InputType::Coordinates => todo!(),
        }
    }
}
