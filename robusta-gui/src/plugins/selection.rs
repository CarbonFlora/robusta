use bevy::{
    app::{First, PreUpdate},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{With, Without},
        system::{Commands, Query, ResMut},
    },
};
use bevy_mod_picking::{
    events::Pointer,
    pointer::{Location, PointerId},
    prelude::{ListenerInput, On},
    selection::{self, Deselect, Select},
    PickableBundle,
};
use bevy_window::{PrimaryWindow, Window};

use super::tag::Tags;

/// This is a wrapper for bevy_mod_picking selection.
pub struct RSelectionPlugin;
impl bevy::app::Plugin for RSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Selection>()
            .add_systems(First, maintain_selection)
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

fn maintain_selection(
    sw: Query<&mut Window, Without<PrimaryWindow>>,
    mut ss: ResMut<selection::SelectionPluginSettings>,
) {
    ss.is_enabled = !sw.single().focused;
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

// pub fn selection_bundle(id: Entity, co: &mut Commands) {
//     co.entity(id).insert((
//         PickableBundle::default(),
//         On::<Pointer<Select>>::send_event::<Selection>(),
//         On::<Pointer<Deselect>>::send_event::<Selection>(),
//     ));
// }

#[derive(Bundle)]
pub struct PickableSelectionBundle {
    a: PickableBundle,
    b: On<Pointer<Select>>,
    c: On<Pointer<Deselect>>,
    d: Tags,
}

impl Default for PickableSelectionBundle {
    fn default() -> Self {
        PickableSelectionBundle {
            a: PickableBundle::default(),
            b: On::<Pointer<Select>>::send_event::<Selection>(),
            c: On::<Pointer<Deselect>>::send_event::<Selection>(),
            d: Tags::default(),
        }
    }
}
