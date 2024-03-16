use super::*;

pub fn view_inspection(
    ui: &mut egui::Ui,
    selected_entities: &[(REntity, Tags)],
    ewa: &mut EventWriter<Act>,
) {
    ui.separator();
    if selected_entities.is_empty() {
        ui.label("No Selected Entities.");
        return;
    }

    for (i, re) in selected_entities.iter().enumerate() {
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

            tag_bundle(ui_idd, re, ewa);
            ui_idd.separator();

            if let Some(c) = c {
                ewa.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
            }
        });
    }
}

fn tag_bundle(ui: &mut egui::Ui, re: &(REntity, Tags), ewa: &mut EventWriter<Act>) {
    for t in &re.1.taglist {
        let _ = ui.small_button(t.name.to_string());
    }
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
    ui.menu_button("⛭", |ui| {
        ui.horizontal_wrapped(|ui_collapse| {
            if ui_collapse.button("⊞").clicked() {
                ewa.send(Act::ModifyTag(re.0.clone(), TagModify::AddPlaceholder));
            }
            if ui_collapse.button("⊟").clicked() {
                ewa.send(Act::ModifyTag(re.0.clone(), TagModify::RemoveAll));
            }
        });
    });
}
