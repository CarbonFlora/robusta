use self::rselection::PhantomPoint;

use super::*;

pub struct RCADPlugins;

impl bevy::app::PluginGroup for RCADPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut builder = bevy::app::PluginGroupBuilder::start::<Self>();

        builder = builder.add(RCADCorePlugin);
        builder = builder.add(RSelectionPlugin);
        builder = builder.add(SnapPlugin);

        builder
    }
}

pub struct RCADCorePlugin;
impl bevy::app::Plugin for RCADCorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Act>()
            .add_systems(Startup, camera_startup)
            .add_systems(Startup, spawn_window)
            .add_systems(PostStartup, draw_first)
            .add_systems(PreUpdate, capture_keystrokes)
            .add_systems(Update, update_viewport_ui)
            .add_systems(Update, update_dock)
            .add_systems(Update, update_phantom_point)
            .add_systems(PostUpdate, update_act);
    }
}

/// This is a wrapper for bevy_mod_picking selection.
pub struct RSelectionPlugin;
impl bevy::app::Plugin for RSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<rselection::Selection>()
            .add_systems(PreUpdate, update_selection);
    }
}

/// Spawn a new window with reasonable defaults.
fn spawn_window(mut co: Commands, mut pw: Query<&mut Window, With<PrimaryWindow>>) {
    co.spawn((
        window::Window {
            title: String::from("CADPanel"),
            focused: false,
            ..Default::default()
        },
        CADPanel::default(),
    ));
    pw.single_mut().title = String::from("Viewport");
}

fn camera_startup(
    mut co: Commands,
    mut ss: ResMut<selection::SelectionSettings>,
    dp: ResMut<State<bevy_mod_picking::debug::DebugPickingMode>>,
) {
    ss.click_nothing_deselect_all = false;
    *dp.into_inner() = State::new(bevy_mod_picking::debug::DebugPickingMode::Disabled);

    co.spawn(Camera2dBundle::default())
        .insert((bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Middle, MouseButton::Right],
            zoom_to_cursor: false,
            ..default()
        },));
}

#[allow(clippy::type_complexity)]
fn update_phantom_point(
    mut ewp: Query<
        (&mut Transform, &mut REntity),
        (With<PhantomPoint>, Without<bevy_pancam::PanCam>),
    >,
    window: Query<&Window, With<PrimaryWindow>>,
    transform: Query<(&Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    let (ca, gt) = transform.single();
    if let Ok((mut tr, re)) = ewp.get_single_mut() {
        if let Some(cursor_world_pos) = window
            .single()
            .cursor_position()
            .and_then(|cursor_pos| ca.viewport_to_world_2d(gt, cursor_pos))
        {
            match &mut re.into_inner() {
                REntity::Point(sp) => {
                    sp.xyz_mut(cursor_world_pos.x, cursor_world_pos.y, sp.coordinates.z);
                }
                _ => panic!("Non-point is a phantom entity."),
            };

            tr.translation = Vec3::new(cursor_world_pos.x, cursor_world_pos.y, tr.translation.z);
        }
    }
}
