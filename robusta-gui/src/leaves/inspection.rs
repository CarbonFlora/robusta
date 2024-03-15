use super::*;

pub fn view_inspection(
    ui: &mut egui::Ui,
    selected_entities: &Vec<(REntity, Tags)>,
    act_write: &mut EventWriter<Act>,
) {
    ui.separator();
    if selected_entities.is_empty() {
        ui.label("No Selected Entities.");
        return;
    }

    for re in selected_entities {
        let mut c: Option<(f32, f32, f32, f32)> = None;

        match &re.0 {
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
        }

        for t in &re.1.taglist {
            let _ = ui.small_button(t.name.to_string());
        }
        ui.separator();

        if let Some(c) = c {
            act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
        }
    }
}
