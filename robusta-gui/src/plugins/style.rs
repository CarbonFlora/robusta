use bevy::sprite::Mesh2dHandle;

use self::tag::TagCharacteristics;

use super::*;

pub struct StylePlugin;
impl bevy::app::Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RefreshStyle>()
            .add_systems(Update, update_rentity_color);
    }
}

#[derive(Debug, Event, PartialEq, Eq, Hash, Clone)]
pub enum RefreshStyle {
    Color,
    Thickness,
}

//This is a bulky and heavy function.
pub fn update_rentity_color(
    //Input
    mut errs: EventReader<RefreshStyle>,
    //Util
    mut rtc: ResMut<TagCharacteristics>,
    mut colorm_assets: ResMut<Assets<ColorMaterial>>,
    //Output
    mut qare: Query<(&mut Handle<ColorMaterial>, &TagList), With<REntity>>,
) {
    for _ in errs.read().filter(|x| x == &&RefreshStyle::Color) {
        for (hcm, tl) in qare.iter_mut() {
            let colorm = match colorm_assets.get_mut(hcm.id()) {
                Some(a) => a,
                None => return,
            };
            if let Some(first_match) = tl.taglist.iter().find(|x| rtc.get(x).color.is_some()) {
                let color = rtc
                    .get(first_match)
                    .color
                    .unwrap()
                    .to_normalized_gamma_f32();
                colorm.color = Color::rgba(color[0], color[1], color[2], color[3]);
            }
        }
    }
}

//This is a bulky and heavy function.
pub fn update_rentity_thickness(
    //Input
    mut errs: EventReader<RefreshStyle>,
    //Util
    mut rtc: ResMut<TagCharacteristics>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    //Output
    mut qare: Query<(&mut Mesh2dHandle, &TagList, &REntity), With<REntity>>,
) {
    for _ in errs.read().filter(|x| x == &&RefreshStyle::Thickness) {
        for (hcm, tl, re) in qare.iter_mut() {
            if let Some(first_match) = tl.taglist.iter().find(|x| rtc.get(x).thickness.is_some()) {
                let thickness = rtc.get(first_match).thickness.unwrap();
                let mesh = match re {
                    REntity::Arc(sp) => sp.arc_mesh(thickness),
                    REntity::Circle(sp) => todo!(),
                    REntity::Line(_) => todo!(),
                    REntity::Point(_) => todo!(),
                    REntity::Text(_) => todo!(),
                    REntity::PhantomPoint => todo!(),
                    REntity::PhantomStatic(_) => todo!(),
                    REntity::SnapPoint(_) => todo!(),
                };
                mesh_assets.remove(&hcm.0);
                mesh_assets.insert(&hcm.0, mesh);
            }
            // m2dh.insert_attribute(Mesh::ATTRIBUTE_POSITION, );
        }
    }
}
// m2dh.compute_aabb()

// #[allow(clippy::type_complexity)]
// pub fn update_entity_with_tags(
//     //Input
//     mut errs: EventReader<RefreshStyle>,
//     //Util
//     rtc: Res<TagCharacteristics>,
//     mut mesh_assets: ResMut<Assets<Mesh>>,
//     mut colorm_assets: ResMut<Assets<ColorMaterial>>,
//     //Output
//     mut qare: Query<
//         (
//             &mut Mesh2dHandle,
//             // &mut Handle<Mesh>,
//             &mut Handle<ColorMaterial>,
//             &mut REntity,
//             &TagList,
//         ),
//         With<REntity>,
//     >,
// ) {
//     let hm = &rtc.tag_flags;

//     for _ in errs.read() {
//         for (m2dh, hcm, mut re, tl) in qare.iter_mut() {
//             let a = tl.taglist.iter().rev();
//             //turn these into events.
//             //todo!()
//             //1. instead of refreshing every rentity, only apply to entities that have the tag.
//             //2. send events to update the specific entities.
//             let mut mesh = mesh_assets.get_mut(m2dh.0.id());
//             let mut colorm = colorm_assets.get_mut(hcm.id());
//         }
//     }

//     // if rtc.is_changed() {
//     //     println!("tc has changed.");
//     // }
// }
