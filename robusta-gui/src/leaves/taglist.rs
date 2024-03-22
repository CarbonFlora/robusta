use std::ops::{Index, IndexMut};

use egui::Sense;
use egui_extras::{Column, TableBuilder};

use self::plugins::{
    keystroke::ModalResources,
    tag::{Tag, TagFlags, TagListModify},
};

use super::*;

#[derive(Debug, Resource, Clone)]
pub struct TaglistBuffer {
    pub ordered_tag_flags: Vec<(Tag, TagFlags)>,
    pub egui_selection: HashMap<usize, Tag>,
    pub is_selection_mode: bool,
}

impl Default for TaglistBuffer {
    fn default() -> Self {
        let ordered_tag_flags = vec![(Tag::new("Default".to_string()), TagFlags::default())];
        Self {
            ordered_tag_flags,
            egui_selection: HashMap::new(),
            is_selection_mode: false,
        }
    }
}

pub fn view_taglist(
    //Util
    ui: &mut egui::Ui,
    tb: &mut TaglistBuffer,
    ewm: &mut ModalResources,
    //Output
    ewa: &mut EventWriter<Act>,
) {
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("⊞").clicked() {
            let tag = Tag::placeholder(Some(tb.ordered_tag_flags.len()));
            ewa.send(Act::ModifyTaglist(TagListModify::Add(tag)));
        }
        if tb.is_selection_mode && ui.button("⊟").clicked() {
            for a in tb.egui_selection.values() {
                ewa.send(Act::ModifyTaglist(TagListModify::Remove(a.clone())));
            }
        }
        ui.checkbox(&mut tb.is_selection_mode, "Selection Mode");
    });

    let table = match tb.is_selection_mode {
        true => TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(Sense::click())
            .column(Column::auto())
            .column(Column::remainder()),
        false => TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .column(Column::auto())
            .column(Column::remainder()),
    };

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Tag");
            });
            header.col(|ui| {
                ui.strong("Characteristics");
            });
        })
        .body(|body| {
            body.rows(20.0, tb.ordered_tag_flags.len(), |mut row| {
                let row_index = row.index();
                row.set_selected(tb.egui_selection.contains_key(&row_index));

                row.col(|ui| {
                    ui.label(&tb.ordered_tag_flags.index(row_index).0.name);
                });
                row.col(|ui| {
                    tag_flag_egui(ui, tb, ewm, row_index, ewa);
                });

                if row.response().clicked() {
                    match tb.egui_selection.contains_key(&row_index) {
                        true => tb.egui_selection.remove(&row_index),
                        false => tb
                            .egui_selection
                            .insert(row_index, tb.ordered_tag_flags.index(row_index).0.clone()),
                    };
                }
            });
        });
}

fn tag_flag_egui(
    //Util
    ui: &mut egui::Ui,
    tb: &mut TaglistBuffer,
    ewm: &mut ModalResources,
    row_index: usize,
    //Output
    ewa: &mut EventWriter<Act>,
) {
    let a = tb.ordered_tag_flags.index_mut(row_index);
    ui.horizontal_wrapped(|ui| {
        if let Some(color) = &mut a.1.color {
            if ui.color_edit_button_srgba(color).lost_focus() {
                ewa.send(Act::ModifyTaglist(TagListModify::NewColor(
                    a.0.clone(),
                    Some(*color),
                )));
            };
        }
    });
}
