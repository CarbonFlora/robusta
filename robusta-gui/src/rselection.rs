use bevy::{
    app::PreUpdate,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        system::{Commands, Query},
    },
};
use bevy_mod_picking::{
    events::Pointer,
    prelude::ListenerInput,
    selection::{Deselect, Select},
};

use crate::REntity;

/// This is a wrapper for bevy_mod_picking selection.
pub struct RSelectionPlugin;
impl bevy::app::Plugin for RSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Selection>()
            .add_systems(PreUpdate, update_selection);
    }
}

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

pub fn deselect_all(c: &mut Commands, es: &Query<Entity, With<Selected>>) {
    for e in es.iter() {
        c.entity(e).remove::<Selected>();
    }
}

pub fn select_all(mut c: Commands, es: Query<Entity, With<REntity>>) {
    for e in es.iter() {
        c.entity(e).insert(Selected);
    }
}
