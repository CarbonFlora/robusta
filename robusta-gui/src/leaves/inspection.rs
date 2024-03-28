use egui::{Response, Ui};

use self::plugins::keystroke::{KeyState, Mode};

use super::*;

type EditingTags = HashSet<Tag>;

#[derive(Debug, Resource, Default, Clone)]
pub struct InspectionBuffer {
    pub selected_list: Vec<(REntity, TagList, EditingTags)>,
    pub str_buf: String,
}

impl InspectionBuffer {
    pub fn soft_reset(&mut self) {
        self.selected_list.iter_mut().for_each(|x| x.2.clear());
        self.str_buf.clear();
    }
}

pub fn view_inspection(
    //Util
    ui: &mut egui::Ui,
    ib: &mut InspectionBuffer,
    //Output
    ewa: &EventWriter<Act>, //Use this to communicate out.
) {
    ui.separator();
    if ib.selected_list.is_empty() {
        ui.label("No Selected Entities.");
        return;
    }

    for (i, (re, tl, hs)) in ib.selected_list.iter_mut().enumerate() {
        ui.push_id(i, |ui| {
            r1(ui, re, &ewa);
            r2(ui, re, tl, hs, ib, &ewa);
            ui.separator();
        });
    }
}

fn r1(ui: &mut Ui, re: &REntity, mut ewa: &EventWriter<Act>) {
    let mut c: Option<(f32, f32, f32, f32)> = None;

    match &re {
        REntity::Arc(sp) => {
            if ui.selectable_label(false, format!("{sp}")).clicked() {
                c = Some(sp.min_max());
            }
        }
        REntity::Circle(sp) => {
            if ui.selectable_label(false, format!("{sp}")).clicked() {
                c = Some(sp.min_max());
            }
        }
        REntity::Line(sp) => {
            if ui.selectable_label(false, format!("{sp}")).clicked() {
                c = Some(sp.min_max());
            }
        }
        REntity::Point(sp) => {
            if ui.selectable_label(false, format!("{sp}")).clicked() {
                c = Some(sp.min_max());
            }
        }
        REntity::Text(sp) => {
            if ui.selectable_label(false, format!("{sp}")).clicked() {
                c = Some(sp.min_max());
            }
        }
        REntity::SnapPoint(_) => (),
        REntity::PhantomPoint => (),
        REntity::PhantomStatic(_) => (),
    }

    if let Some(c) = c {
        ewa.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
    }
}

fn r2(
    ui: &mut egui::Ui,
    re: &REntity,
    tl: &TagList,
    hs: &HashSet<Tag>,
    ib: &mut InspectionBuffer,
    ewa: &EventWriter<Act>,
) {
    ui.horizontal_wrapped(|ui| {
        r2c1(ui, re, tl, &ewa);
        r2c2(ui, re, tl, hs, ib, &ewa);
    });
}

fn r2c1(ui: &mut Ui, re: &REntity, tl: &TagList, mut ewa: &EventWriter<Act>) {
    ui.menu_button("⛭", |ui| {
        ui.horizontal(|ui_collapse| {
            if ui_collapse.button("⊞").clicked() {
                let a = Tag::new(format!("Untitled-{}", tl.0.len() + 1));
                ewa.send(Act::ModifyTag(re.clone(), TagModify::Add(a.clone())));
            }
            if ui_collapse.button("⊟").clicked() {
                ewa.send(Act::ModifyTag(re.clone(), TagModify::RemoveAll));
            }
        });
    });
}

fn r2c2(
    ui: &mut Ui,
    re: &REntity,
    tl: &TagList,
    hs: &HashSet<Tag>,
    ib: &mut InspectionBuffer,
    ewa: &EventWriter<Act>,
) {
    for tag in tl.0.iter() {
        match hs.contains(tag) {
            false => {
                if ui.small_button(tag.name.to_string()).clicked() {
                    hs.insert(tag.clone());
                }
            }
            true => {
                let r = ui.text_edit_singleline(&mut ib.str_buf);
                typing_keybind_mode(&r, &ewa);
                if r.lost_focus() {
                    tag_rename(&re, &ib.str_buf, tag, &ewa);
                    ib.soft_reset();
                }
            }
        }
    }
}

pub fn typing_keybind_mode(r: &Response, mut ewa: &EventWriter<Act>) {
    if r.gained_focus() {
        ewa.send(Act::KeyState(Mode::Typing));
    }
    if r.lost_focus() {
        ewa.send(Act::KeyState(Mode::Normal));
    }
}

fn tag_rename(re: &REntity, string_buffer: &String, tag: &Tag, mut ewa: &EventWriter<Act>) {
    let mut ewa_packages = vec![Act::ModifyTag(re.clone(), TagModify::Remove(tag.clone()))];

    if !string_buffer.is_empty() {
        ewa_packages.push(Act::ModifyTag(
            re.clone(),
            TagModify::Add(Tag::new(string_buffer.to_string())),
        ));
    }

    ewa.send_batch(ewa_packages);
}
