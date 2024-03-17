use bevy::utils::hashbrown::HashSet;

use super::*;

#[derive(Debug, Resource, Default, Clone)]
pub struct InspectionBuffer {
    pub selected: Vec<(REntity, Tags)>,
    pub editing_tag: HashSet<Tag>,
    pub temporary_name: String,
}

pub fn view_inspection(ui: &mut egui::Ui, ib: &mut InspectionBuffer, ewa: &mut EventWriter<Act>) {
    ui.separator();
    if ib.selected.is_empty() {
        ui.label("No Selected Entities.");
    } else {
        inspection_bundle(ui, ib, ewa);
    }
}

fn inspection_bundle(ui: &mut egui::Ui, ib: &mut InspectionBuffer, ewa: &mut EventWriter<Act>) {
    for (i, re) in ib.selected.iter_mut().enumerate() {
        ui.push_id(i, |ui_idd| {
            let mut c: Option<(f32, f32, f32, f32)> = None;

            match &re.0 {
                REntity::Arc(sp) => {
                    if ui_idd.selectable_label(false, format!("{sp}")).clicked() {
                        c = Some(sp.min_max());
                    }
                }
                REntity::Circle(sp) => {
                    if ui_idd.selectable_label(false, format!("{sp}")).clicked() {
                        c = Some(sp.min_max());
                    }
                }
                REntity::Line(sp) => {
                    if ui_idd.selectable_label(false, format!("{sp}")).clicked() {
                        c = Some(sp.min_max());
                    }
                }
                REntity::Point(sp) => {
                    if ui_idd.selectable_label(false, format!("{sp}")).clicked() {
                        c = Some(sp.min_max());
                    }
                }
                REntity::Text(sp) => {
                    if ui_idd.selectable_label(false, format!("{sp}")).clicked() {
                        c = Some(sp.min_max());
                    }
                }
                REntity::SnapPoint(_) => (),
                REntity::PhantomPoint => (),
            }

            tag_bundle(ui_idd, re, ewa, &mut ib.editing_tag, &mut ib.temporary_name);
            ui_idd.separator();

            if let Some(c) = c {
                ewa.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
            }
        });
    }
}

fn tag_bundle(
    ui: &mut egui::Ui,
    re: &mut (REntity, Tags),
    ewa: &mut EventWriter<Act>,
    hst: &mut HashSet<Tag>,
    temporary_name: &mut String,
) {
    ui.horizontal_wrapped(|ui| {
        ui.menu_button("⛭", |ui| {
            ui.horizontal(|ui_collapse| {
                if ui_collapse.button("⊞").clicked() {
                    let a = Tag::new(format!("Untitled-{}", re.1.ordered_taglist.len() + 1));
                    ewa.send(Act::ModifyTag(re.0.clone(), TagModify::Add(a)));
                }
                if ui_collapse.button("⊟").clicked() {
                    ewa.send(Act::ModifyTag(re.0.clone(), TagModify::RemoveAll));
                }
            });
        });

        for (i, t) in re.1.ordered_taglist.clone().iter().enumerate() {
            match hst.contains(t) {
                true => {
                    if ui.text_edit_singleline(temporary_name).lost_focus() {
                        temporary_name.clear();
                        re.1.ordered_taglist.remove(i);
                        re.1.ordered_taglist
                            .insert(i, Tag::new(temporary_name.to_string()));
                        hst.remove(t);
                    };
                }
                false => {
                    if ui.small_button(t.name.to_string()).clicked() {
                        hst.insert(t.clone());
                    };
                }
            };
        }
    });
    // ui.collapsing("⛭", |ui| {
    //     ui.horizontal_wrapped(|ui_collapse| {
    //         if ui_collapse.button("⊞").clicked() {
    //             ewa.send(Act::ModifyTag(re.0.clone(), TagModify::AddPlaceholder));
    //         }
    //         if ui_collapse.button("⊟").clicked() {
    //             ewa.send(Act::ModifyTag(re.0.clone(), TagModify::RemoveAll));
    //         }
    //     });
    // });
}
