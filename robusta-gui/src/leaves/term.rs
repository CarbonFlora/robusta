use bevy_egui::EguiContexts;
use bevy_text_popup::{TextPopupButton, TextPopupEvent, TextPopupLocation};

use crate::*;

pub fn open_term(mut text_popup_events: EventWriter<TextPopupEvent>) {
    let event = TextPopupEvent {
        content: "Close this popup and generate a new one at the bottom?".to_string(),
        confirm_button: Some(TextPopupButton {
            text: "OK".to_string(),
            action: |commands, root_entity| {
                // Fire event to spawn a new popup when user clicks 'OK'.
                commands.add(|world: &mut World| {
                    world.send_event(TextPopupEvent {
                        content: "New Popup Generated".to_string(),
                        location: TextPopupLocation::Bottom,
                        ..Default::default()
                    });
                });
                // Despawn the original popup.
                commands.entity(root_entity).despawn_recursive();
            },
            ..Default::default()
        }),
        dismiss_button: Some(TextPopupButton {
            text: "Cancel".to_string(),
            background_color: Color::RED,
            ..Default::default()
        }),
        ..default()
    };
    text_popup_events.send(event);
}

pub fn open_term_egui(context: Query<&mut EguiContext, With<PrimaryWindow>>) {
    if let Ok(w) = context.get_single() {
        egui::Window::new("Hello").show(w.get(), |ui| {
            ui.label("world");
        });
    } else {
        return;
    }
    // let Ok(egui_context) = world
    //     .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    //     .get_single(world)
    // else {
    //     return;
    // };
    // let mut egui_context = egui_context.clone();

    // world.resource_scope::<UiState, _>(|world, mut ui_state| {
    //     ui_state.ui(world, egui_context.get_mut())
    // });
}
