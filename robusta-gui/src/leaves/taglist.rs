use std::ops::Index;

use egui::Sense;
use egui_extras::{Column, TableBuilder};

use super::*;

pub fn view_taglist(ui: &mut egui::Ui, ewa: &mut EventWriter<Act>, tc: &TagCharacteristics) {
    ui.separator();
    if tc.is_empty() {
        ui.label("No Selected Entities.");
        return;
    }

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
        .body(|mut body| {
            body.rows(20.0, tc.len(), |mut row| {
                let row_index = row.index();

                row.col(|ui| {
                    ui.label(&tc.index(row_index).0.name);
                });
                row.col(|ui| {
                    ui.label(format!("{:?}", &tc.index(row_index).1));
                });
            });
        });
}
