use std::ops::{Index, IndexMut};

use egui_extras::{Column, TableBuilder};

use self::plugins::{
    keystroke::ModalResources,
    tag::{Tag, TagFlags, TagListModify},
};

use super::*;

#[derive(Debug, Resource, Clone)]
pub struct TaglistBuffer {
    pub ordered_tag_flags: Vec<(Tag, TagFlags, Selected)>,
}

type Selected = bool;

impl Default for TaglistBuffer {
    fn default() -> Self {
        let ordered_tag_flags = vec![(Tag::new("Default".to_string()), TagFlags::default(), false)];
        Self { ordered_tag_flags }
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
        if ui.button("⊟").clicked() {
            for a in tb.ordered_tag_flags.iter().filter(|x| x.2) {
                ewa.send(Act::ModifyTaglist(TagListModify::Remove(a.0.clone())));
            }
        }
    });

    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .column(Column::exact(20.))
        .column(Column::initial(100.))
        .column(Column::remainder());

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Sel");
            });
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
                row.set_selected(tb.ordered_tag_flags[row_index].2);

                row.col(|ui| {
                    ui.checkbox(&mut tb.ordered_tag_flags[row_index].2, "");
                });
                row.col(|ui| {
                    ui.label(&tb.ordered_tag_flags.index(row_index).0.name);
                });
                row.col(|ui| {
                    tag_flag_egui(ui, tb, ewm, row_index, ewa);
                });
            });
        });
}

fn tag_flag_egui(
    //Util
    ui: &mut egui::Ui,
    tb: &mut TaglistBuffer,
    _ewm: &mut ModalResources,
    row_index: usize,
    //Output
    ewa: &mut EventWriter<Act>,
) {
    let a = tb.ordered_tag_flags.index_mut(row_index);

    ui.horizontal_wrapped(|ui| {
        ui.menu_button("⛭", |ui| {
            ui.horizontal(|ui_collapse| {
                if ui_collapse.button("🎨").clicked() {
                    a.1.toggle_color();
                }
                if ui_collapse.button("🇦").clicked() {
                    a.1.toggle_thickness();
                }
            });
        });

        if let Some(color) = &mut a.1.color {
            if ui.color_edit_button_srgba(color).changed() {
                ewa.send(Act::ModifyTaglist(TagListModify::NewColor(
                    a.0.clone(),
                    Some(*color),
                )));
            };
        }
        if let Some(thickness) = &mut a.1.thickness {
            ui.label(format!("Thickness: {}", thickness));
            // if ui.color_edit_button_srgba(color).changed() {
            //     ewa.send(Act::ModifyTaglist(TagListModify::NewColor(
            //         a.0.clone(),
            //         Some(*color),
            //     )));
            // };
        }
    });
}
