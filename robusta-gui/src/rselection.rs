use bevy::{
    app::PreUpdate,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query},
    },
};
use bevy_mod_picking::{
    events::Pointer,
    pointer::{Location, PointerId},
    prelude::ListenerInput,
    selection::{Deselect, Select},
};

/// This is a wrapper for bevy_mod_picking selection.
pub struct RSelectionPlugin;
impl bevy::app::Plugin for RSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Selection>()
            .add_systems(PreUpdate, update_selection);
    }
}

#[derive(Debug, Component)]
pub struct Selected {
    pub pointer_id: PointerId,
    pub pointer_location: Location,
}

#[derive(Event, Clone, Debug, PartialEq)]
pub struct Selection(pub Entity, pub PointerId, pub Location, pub bool);

impl From<ListenerInput<Pointer<Select>>> for Selection {
    fn from(event: ListenerInput<Pointer<Select>>) -> Self {
        Selection(
            event.target,
            event.pointer_id,
            event.pointer_location.clone(),
            true,
        )
    }
}

impl From<ListenerInput<Pointer<Deselect>>> for Selection {
    fn from(event: ListenerInput<Pointer<Deselect>>) -> Self {
        Selection(
            event.target,
            event.pointer_id,
            event.pointer_location.clone(),
            false,
        )
    }
}

pub fn update_selection(mut c: Commands, mut evs: EventReader<Selection>) {
    for s in evs.read() {
        if s.3 {
            c.entity(s.0).try_insert(Selected {
                pointer_id: s.1,
                pointer_location: s.2.clone(),
            });
        } else {
            c.entity(s.0).remove::<Selected>();
        }
    }
}

pub fn deselect_all(
    c: &mut Commands,
    es: &Query<(Entity, &Selected), With<Selected>>,
    dsel: &mut EventWriter<Pointer<Deselect>>,
) {
    for e in es.iter() {
        dsel.send(Pointer::new(
            e.1.pointer_id,
            e.1.pointer_location.clone(),
            e.0,
            Deselect,
        ));
        c.entity(e.0).remove::<Selected>();
    }
}
