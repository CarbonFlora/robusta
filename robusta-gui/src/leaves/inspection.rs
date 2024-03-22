use self::plugins::keystroke::{ModalResources, Mode};

use super::*;

type EditingTags = HashSet<Tag>;

#[derive(Debug, Resource, Default, Clone)]
pub struct InspectionBuffer {
    pub selected_list: Vec<(REntity, TagList, EditingTags)>,
    pub temporary_name: String,
}

pub fn view_inspection(
    //Util
    ui: &mut egui::Ui,
    ib: &mut InspectionBuffer,
    ewm: &mut ModalResources, //Change this when typing.
    //Output
    ewa: &mut EventWriter<Act>, //Use this to communicate out.
) {
    ui.separator();
    if ib.selected_list.is_empty() {
        ui.label("No Selected Entities.");
    } else {
        inspection_bundle(ui, ib, ewm, ewa);
    }
}

fn inspection_bundle(
    //Util
    ui: &mut egui::Ui,
    ib: &mut InspectionBuffer,
    ewm: &mut ModalResources,
    //Output
    ewa: &mut EventWriter<Act>,
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

            tag_bundle(ui_idd, ewm, selected, &mut ib.temporary_name, ewa);
            ui_idd.separator();

            if let Some(c) = c {
                ewa.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
            }
        });
    }
}

fn tag_bundle(
    //Util
    ui: &mut egui::Ui,
    ewm: &mut ModalResources,
    //Intersection Buffer Parts
    selected: &mut (REntity, TagList, EditingTags),
    string_buffer: &mut String,
    //Output
    ewa: &mut EventWriter<Act>,
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
                }
                if ui_collapse.button("⊟").clicked() {
                    ewa.send(Act::ModifyTag(selected.0.clone(), TagModify::RemoveAll));
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
                        act_deliver(ewa, &mut selected.0, &mut selected.2, string_buffer, tag);
                    } else {
                        ewm.mode = Mode::Typing;
                    };
                    response.request_focus();
                }
            }
        }
    });
}

fn act_deliver(
    ewa: &mut EventWriter<Act>,
    re: &mut REntity,
    editing_tags: &mut EditingTags,
    string_buffer: &mut String,
    tag: &Tag,
) {
    let mut ewa_packages = vec![Act::ModifyTag(re.clone(), TagModify::Remove(tag.clone()))];

    if !string_buffer.is_empty() {
        ewa_packages.push(Act::ModifyTag(
            re.clone(),
            TagModify::Add(Tag::new(string_buffer.to_string())),
        ));
    }

    ewa.send_batch(ewa_packages);
    string_buffer.clear();
    editing_tags.clear();
}
