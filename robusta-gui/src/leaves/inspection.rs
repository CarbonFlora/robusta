use self::plugins::keystroke::{ModalResources, Mode};

use super::*;

type EditingTags = HashSet<Tag>;

#[derive(Debug, Resource, Default, Clone)]
pub struct InspectionBuffer {
    pub selected_list: Vec<(REntity, TagList, EditingTags)>,
    pub temporary_name: String,
}

impl InspectionBuffer {
    pub fn sync(&mut self) {
        //if buffers get too large, it may be cleaner to have a dedicated sync function to update the buffer with what is actually happening in the world.
    }
}

pub fn view_inspection(
    ui: &mut egui::Ui,
    ib: &mut InspectionBuffer,
    ewa: &mut EventWriter<Act>,
    ewdbm: &mut EventWriter<DockBufferModify>,
    ewm: &mut ModalResources,
) {
    ui.separator();
    if ib.selected_list.is_empty() {
        ui.label("No Selected Entities.");
    } else {
        inspection_bundle(ui, ib, ewa, ewdbm, ewm);
    }
}

fn inspection_bundle(
    ui: &mut egui::Ui,
    ib: &mut InspectionBuffer,
    ewa: &mut EventWriter<Act>,
    ewdbm: &mut EventWriter<DockBufferModify>,
    ewm: &mut ModalResources,
) {
    for (i, selected) in ib.selected_list.iter_mut().enumerate() {
        ui.push_id(i, |ui_idd| {
            let mut c: Option<(f32, f32, f32, f32)> = None;

            match &selected.0 {
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
                REntity::PhantomStatic(_) => (),
            }

            tag_bundle(ui_idd, ewa, ewdbm, ewm, selected, &mut ib.temporary_name);
            ui_idd.separator();

            if let Some(c) = c {
                ewa.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
            }
        });
    }
}

fn tag_bundle(
    ui: &mut egui::Ui,
    ewa: &mut EventWriter<Act>,
    ewdbm: &mut EventWriter<DockBufferModify>,
    ewm: &mut ModalResources,
    //Intersection Buffer Parts
    selected: &mut (REntity, TagList, EditingTags),
    string_buffer: &mut String,
) {
    ui.horizontal_wrapped(|ui| {
        ui.menu_button("⛭", |ui| {
            ui.horizontal(|ui_collapse| {
                if ui_collapse.button("⊞").clicked() {
                    let a = Tag::new(format!("Untitled-{}", selected.1.taglist.len() + 1));
                    ewa.send(Act::ModifyTag(
                        selected.0.clone(),
                        TagModify::Add(a.clone()),
                    ));
                    ewdbm.send(DockBufferModify::AddTag(selected.0.clone(), a));
                }
                if ui_collapse.button("⊟").clicked() {
                    ewa.send(Act::ModifyTag(selected.0.clone(), TagModify::RemoveAll));
                    ewdbm.send(DockBufferModify::RemoveAllTags(selected.0.clone()));
                }
            });
        });

        for tag in selected.1.taglist.iter() {
            match selected.2.contains(tag) {
                false => {
                    if ui.small_button(tag.name.to_string()).clicked() {
                        selected.2.insert(tag.clone());
                    }
                }
                true => {
                    let response = ui.text_edit_singleline(string_buffer);
                    if response.lost_focus() {
                        ewm.mode = Mode::Normal;
                        ewa.send_batch([
                            Act::ModifyTag(selected.0.clone(), TagModify::Remove(tag.clone())),
                            Act::ModifyTag(
                                selected.0.clone(),
                                TagModify::Add(Tag::new(string_buffer.to_string())),
                            ),
                        ]);
                        ewdbm.send_batch([
                            DockBufferModify::RemoveTag(selected.0.clone(), tag.clone()),
                            DockBufferModify::AddTag(
                                selected.0.clone(),
                                Tag::new(string_buffer.to_string()),
                            ),
                        ]);

                        string_buffer.clear();
                        selected.2.clear();
                    } else {
                        ewm.mode = Mode::Typing;
                    };
                    response.request_focus();
                }
            }
        }
    });
}
