use egui::Response;

use self::plugins::keystroke::Mode;

use super::*;

#[derive(Debug, Resource, Default, Clone)]
pub struct InspectionBuffer {
    pub selected_list: Vec<(REntity, TagList)>,
}

pub fn view_inspection(ui: &mut egui::Ui, ib: &mut InspectionBuffer, ewa: &mut EventWriter<Act>) {
    ui.separator();
    if ib.selected_list.is_empty() {
        ui.label("No Selected Entities.");
        return;
    }

    for i in 0..ib.selected_list.len() {
        ui.push_id(i, |ui| {
            r1(ui, (i, ib), ewa);
            r2(ui, (i, ib), ewa);
            r3(ui, (i, ib), ewa);
            ui.separator();
        });
    }
}

fn r1(ui: &mut egui::Ui, (i, ib): (usize, &mut InspectionBuffer), ewa: &mut EventWriter<Act>) {
    let (re, _tl) = &ib.selected_list[i];
    let mut c: Option<(f32, f32, f32, f32)> = None;

    match re {
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

fn r2(ui: &mut egui::Ui, (i, ib): (usize, &mut InspectionBuffer), ewa: &mut EventWriter<Act>) {
    let (i, ib) = (i, ib);
    let (re, tl) = &ib.selected_list[i];
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

fn r3(ui: &mut egui::Ui, (i, ib): (usize, &mut InspectionBuffer), ewa: &mut EventWriter<Act>) {
    let (i, ib) = (i, ib);
    let (re, tl) = &mut ib.selected_list[i];

    for tag in tl.0.iter_mut() {
        let r = ui.text_edit_singleline(&mut tag.name);
        typing_keybind_mode(&r, ewa);
        if r.lost_focus() {
            tag_rename(re, &tag.name, tag, ewa);
        }
    }
}

// fn r2c1(ui: &mut egui::Ui, (i, ib): (usize, &mut InspectionBuffer), ewa: &mut EventWriter<Act>) {
//     let (re, tl, _hs) = &ib.selected_list[i];
//     ui.menu_button("⛭", |ui| {
//         ui.horizontal(|ui_collapse| {
//             if ui_collapse.button("⊞").clicked() {
//                 let a = Tag::new(format!("Untitled-{}", tl.0.len() + 1));
//                 ewa.send(Act::ModifyTag(re.clone(), TagModify::Add(a.clone())));
//             }
//             if ui_collapse.button("⊟").clicked() {
//                 ewa.send(Act::ModifyTag(re.clone(), TagModify::RemoveAll));
//             }
//         });
//     });
// }

// fn r2c2(ui: &mut egui::Ui, (i, ib): (usize, &mut InspectionBuffer), ewa: &mut EventWriter<Act>) {
//     let (re, tl, hs) = &mut ib.selected_list[i];
//     let sb = &mut ib.str_buf;

//     for tag in tl.0.iter() {
//         match *hs == Some(tag.clone()) {
//             //this is spagetti code
//             false => {
//                 if ui.small_button(tag.name.to_string()).clicked() {
//                     *hs = Some(tag.clone());
//                 }
//             }
//             true => {
//                 let r = ui.text_edit_singleline(sb);
//                 typing_keybind_mode(&r, ewa);
//                 if r.lost_focus() {
//                     tag_rename(re, sb, tag, ewa);
//                     soft_reset(hs, sb);
//                 }
//             }
//         }
//     }
// }

pub fn typing_keybind_mode(r: &Response, ewa: &mut EventWriter<Act>) {
    if r.gained_focus() {
        ewa.send(Act::KeyState(Mode::Typing));
    }
    if r.lost_focus() {
        ewa.send(Act::KeyState(Mode::Normal));
    }
}

fn tag_rename(re: &REntity, sb: &String, tag: &Tag, ewa: &mut EventWriter<Act>) {
    let mut ewa_packages = vec![Act::ModifyTag(re.clone(), TagModify::Remove(tag.clone()))];

    if !sb.is_empty() {
        ewa_packages.push(Act::ModifyTag(
            re.clone(),
            TagModify::Add(Tag::new(sb.to_string())),
        ));
    }

    ewa.send_batch(ewa_packages);
}

pub fn refresh_inspection_buffer(
    mut db: ResMut<DockBuffer>,
    qretl: Query<(&REntity, &TagList), With<Selected>>,
) {
    let ib = &mut db.inspection;
    ib.selected_list = qretl
        .iter()
        .map(|x| (x.0.clone(), x.1.clone()))
        .collect::<Vec<(REntity, TagList)>>();
}
