use bevy::utils::HashMap;
use robusta_dxf::wrapper::DXFWrapper;

// use crate::uistate::*;
// use crate::*;

pub fn view_points(
    ui: &mut egui::Ui,
    loaded_files: &HashMap<Option<String>, DXFWrapper>,
    // type_registry: &TypeRegistry,
    // world: &World,
    // selection: &mut InspectorSelection,
) {
    let mut text = String::new();
    for file in loaded_files {
        for point in &file.1.points {
            text += format!("{}\n", point).as_str();
        }
    }
    ui.label(text);
    // let mut assets: Vec<_> = type_registry
    //     .iter()
    //     .filter_map(|registration| {
    //         let reflect_asset = registration.data::<ReflectAsset>()?;
    //         Some((
    //             registration.type_info().type_path_table().short_path(),
    //             registration.type_id(),
    //             reflect_asset,
    //         ))
    //     })
    //     .collect();
    // assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

    // for (asset_name, asset_type_id, reflect_asset) in assets {
    //     let handles: Vec<_> = reflect_asset.ids(world).collect();

    //     ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
    //         for handle in handles {
    //             let selected = match *selection {
    //                 InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
    //                 _ => false,
    //             };

    //             if ui
    //                 .selectable_label(selected, format!("{:?}", handle))
    //                 .clicked()
    //             {
    //                 *selection =
    //                     InspectorSelection::Asset(asset_type_id, asset_name.to_string(), handle);
    //             }
    //         }
    //     });
    // }
}