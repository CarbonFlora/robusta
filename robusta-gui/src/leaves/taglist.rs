use std::ops::{Index, IndexMut};

use egui::Sense;
use egui_extras::{Column, TableBuilder};

use self::plugins::tag::{Tag, TagListModify};

use super::*;

pub fn view_taglist(
    //Util
    tc: &mut TagCharacteristics,
    ui: &mut egui::Ui,
    //Output
    ewa: &mut EventWriter<Act>,
    db: &mut DockBuffer,
) {
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("⊞").clicked() {
            let tag = Tag::placeholder(Some(tc.len()));
            ewa.send(Act::ModifyTaglist(TagListModify::Add(tag)));
        }
        if db.is_selection_mode && ui.button("⊟").clicked() {
            for a in db.egui_selection.values() {
                ewa.send(Act::ModifyTaglist(TagListModify::Remove(a.clone())));
            }
        }
        ui.checkbox(&mut db.is_selection_mode, "Selection Mode");
    });

    let table = match db.is_selection_mode {
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
            body.rows(20.0, tc.len(), |mut row| {
                let row_index = row.index();
                row.set_selected(db.egui_selection.contains_key(&row_index));

                row.col(|ui| {
                    ui.label(&tc.index(row_index).0.name);
                });
                row.col(|ui| {
                    tag_flag_egui(ui, tc, db, row_index);
                    // ui.label(format!("{:?}", &tc.index(row_index).1));
                });

                if row.response().clicked() {
                    match db.egui_selection.contains_key(&row_index) {
                        true => db.egui_selection.remove(&row_index),
                        false => db
                            .egui_selection
                            .insert(row_index, tc.index(row_index).0.clone()),
                    };
                }
            });
        });
}

fn tag_flag_egui(
    ui: &mut egui::Ui,
    tc: &mut TagCharacteristics,
    db: &mut DockBuffer,
    row_index: usize,
) {
    let a = tc.index_mut(row_index);
    ui.horizontal_wrapped(|ui| {
        if let Some(color) = &mut a.1.color {
            ui.color_edit_button_srgba(color);
        }
    });
}
