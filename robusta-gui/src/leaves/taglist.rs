use std::ops::Index;

use egui::Sense;
use egui_extras::{Column, TableBuilder};

use self::plugins::tag::{Tag, TagListModify};

use super::*;

pub fn view_taglist(
    ui: &mut egui::Ui,
    ewa: &mut EventWriter<Act>,
    tc: &TagCharacteristics,
    db: &DockBuffer,
) {
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("⊞").clicked() {
            let tag = Tag::placeholder(Some(tc.len()));
            ewa.send(Act::ModifyTaglist(TagListModify::Add(tag)));
        }
        if ui.button("⊟").clicked() {
            for a in db.egui_selection.values() {
                ewa.send(Act::ModifyTaglist(TagListModify::Remove(a.clone())));
            }
        }
    });

    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .sense(Sense::click())
        .column(Column::auto())
        .column(Column::remainder());
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
                    ui.label(format!("{:?}", &tc.index(row_index).1));
                });

                if row.response().clicked() {
                    ewa.send(Act::ToggleRowSelection((
                        row_index,
                        tc.index(row_index).0.clone(),
                    )));
                }
            });
        });
}
