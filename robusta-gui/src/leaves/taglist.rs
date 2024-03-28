use std::ops::{Index, IndexMut};

use egui_extras::{Column, TableBuilder};

use self::plugins::tag::{Tag, TagCharacteristics, TagFlags, TagListModify};

use super::*;

type Selected = bool;

#[derive(Debug, Resource, Clone)]
pub struct TaglistBuffer {
    pub ordered_tag_flags: Vec<(Tag, TagFlags, Selected)>,
}

impl Default for TaglistBuffer {
    fn default() -> Self {
        let ordered_tag_flags = vec![(Tag::new("Default".to_string()), TagFlags::default(), false)];
        Self { ordered_tag_flags }
    }
}

pub fn view_taglist(ui: &mut egui::Ui, tb: &mut TaglistBuffer, ewa: &mut EventWriter<Act>) {
    ui.separator();
    r1(ui, tb, ewa);
    r2(ui, tb, ewa);
}

fn r1(ui: &mut egui::Ui, tb: &mut TaglistBuffer, ewa: &mut EventWriter<Act>) {
    ui.horizontal(|ui| {
        if ui.button("⊞").clicked() {
            let tag = Tag::placeholder(Some(tb.ordered_tag_flags.len()));
            ewa.send(Act::Taglist(TagListModify::Add(tag)));
        }
        if ui.button("⊟").clicked() {
            for a in tb.ordered_tag_flags.iter().filter(|x| x.2) {
                ewa.send(Act::Taglist(TagListModify::Remove(a.0.clone())));
            }
        }
    });
}

fn r2(ui: &mut egui::Ui, tb: &mut TaglistBuffer, ewa: &mut EventWriter<Act>) {
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
                let i = row.index();
                row.set_selected(tb.ordered_tag_flags[i].2);

                row.col(|ui| {
                    ui.checkbox(&mut tb.ordered_tag_flags[i].2, "");
                });
                row.col(|ui| {
                    ui.label(&tb.ordered_tag_flags.index(i).0.name);
                });
                row.col(|ui| {
                    tag_flag_egui(ui, (i, tb), ewa);
                });
            });
        });
}

fn tag_flag_egui(
    //Util
    ui: &mut egui::Ui,
    (i, tb): (usize, &mut TaglistBuffer),
    //Output
    ewa: &mut EventWriter<Act>,
) {
    let (t, tf, _selected) = tb.ordered_tag_flags.index_mut(i);

    ui.horizontal_wrapped(|ui| {
        ui.menu_button("⛭", |ui| {
            ui.horizontal(|ui_collapse| {
                if ui_collapse.button("🎨").clicked() {
                    tf.toggle_color();
                }
                if ui_collapse.button("🇦").clicked() {
                    tf.toggle_thickness();
                }
            });
        });

        if let Some(color) = &mut tf.color {
            if ui.color_edit_button_srgba(color).changed() {
                ewa.send(Act::Taglist(TagListModify::NewColor(
                    t.clone(),
                    Some(*color),
                )));
            };
        }
        if let Some(thickness) = &mut tf.thickness {
            ui.label(format!("Thickness: {}", thickness));
        }
    });
}

pub fn refresh_taglist_buffer(mut db: ResMut<DockBuffer>, rtc: Res<TagCharacteristics>) {
    if rtc.is_changed() {
        let tb = &mut db.taglist;
        tb.ordered_tag_flags = rtc
            .tag_flags
            .iter()
            .map(|x| (x.0.clone(), *x.1, false))
            .collect::<Vec<(Tag, TagFlags, Selected)>>();
    }
}
