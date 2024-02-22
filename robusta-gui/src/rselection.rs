use bevy::ecs::{
    component::Component,
    entity::Entity,
    event::{Event, EventReader},
    query::With,
    system::{Commands, Query},
};
use bevy_mod_picking::{
    events::Pointer,
    prelude::{ListenerInput, On},
    selection::{Deselect, Select},
    PickableBundle,
};

use crate::{PhantomREntity, REntity};

#[derive(Debug, Component)]
pub struct Selected;

#[derive(Event, Clone, Debug, PartialEq)]
pub struct Selection(pub Entity, pub bool);

impl From<ListenerInput<Pointer<Select>>> for Selection {
    fn from(event: ListenerInput<Pointer<Select>>) -> Self {
        Selection(event.target, true)
    }
}

impl From<ListenerInput<Pointer<Deselect>>> for Selection {
    fn from(event: ListenerInput<Pointer<Deselect>>) -> Self {
        Selection(event.target, false)
    }
}

pub fn update_selection(mut c: Commands, mut evs: EventReader<Selection>) {
    for s in evs.read() {
        if s.1 {
            c.entity(s.0).try_insert(Selected);
        } else {
            c.entity(s.0).remove::<Selected>();
        }
    }
}

pub fn normalize(c: &mut Commands, e: Entity) {
    c.entity(e).insert((
        PickableBundle::default(),
        On::<Pointer<Select>>::send_event::<Selection>(),
        On::<Pointer<Deselect>>::send_event::<Selection>(),
    ));
    c.entity(e).remove::<PhantomREntity>();
}

pub fn deselect_all(mut c: Commands, es: Query<Entity, With<Selected>>) {
    for e in es.iter() {
        c.entity(e).remove::<Selected>();
    }
}

pub fn select_all(mut c: Commands, es: Query<Entity, With<REntity>>) {
    for e in es.iter() {
        c.entity(e).add(Selected);
    }
}
