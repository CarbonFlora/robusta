use super::*;

pub struct RCameraPlugin;
impl bevy::app::Plugin for RCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_startup)
            .add_systems(Update, update_camera);
    }
}

fn camera_startup(mut co: Commands, dp: ResMut<bevy_mod_picking::debug::DebugPickingMode>) {
    *dp.into_inner() = bevy_mod_picking::debug::DebugPickingMode::Disabled;

    co.spawn(Camera2dBundle::default())
        .insert((bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Middle, MouseButton::Right],
            // zoom_to_cursor: false,
            ..default()
        },));
}

fn update_camera(
    //Input
    mut era: EventReader<Act>,
    //Util
    qre: Query<&REntity>,
    mut qpc: Query<&mut bevy_pancam::PanCam>,
    qw: Query<&Window, With<CADPanel>>,
    //Output
    mut camera: Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
) {
    let a = qw.iter().any(|x| x.focused);
    qpc.iter_mut().for_each(|mut x| x.enabled = !a);

    for a in era.read() {
        match a {
            Act::MoveCamera((x, y)) => camera_transform(x, y, &mut camera),
            Act::ZoomCamera(z) => camera_zoom(z, &mut camera),
            Act::PullCameraFocus(rect) => camera_movement(rect, &mut camera),
            Act::FitView => camera_movement(&fit_view_rect(&qre), &mut camera),
            _ => (),
        }
    }
}

fn camera_movement(
    entity_position: &Rect,
    camera: &mut Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
) {
    let mut camera = camera.get_single_mut().unwrap();
    let current_3d_pos = camera.1.to_scale_rotation_translation().2;
    let current_2d_pos = Vec2::new(current_3d_pos.x, current_3d_pos.y);
    let entity_position = (entity_position.max - entity_position.min) / 2. + entity_position.min;
    let delta = current_2d_pos - entity_position;
    let proposed_cam_transform = camera.0.translation - delta.extend(0.);

    camera.0.translation = proposed_cam_transform;
}

fn camera_transform(
    x: &f32,
    y: &f32,
    camera: &mut Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
) {
    let mut camera = camera.get_single_mut().unwrap();
    let scale = camera.2.scale;
    let current_3d_pos = camera.1.to_scale_rotation_translation().2;
    let current_2d_pos = Vec2::new(current_3d_pos.x, current_3d_pos.y);
    let movement = Vec2 {
        x: *x * scale * 20.,
        y: *y * scale * 20.,
    };
    let delta = current_2d_pos + movement;
    let proposed_cam_transform = delta.extend(0.);

    camera.0.translation = proposed_cam_transform;
}

fn camera_zoom(
    z: &f32,
    camera: &mut Query<
        (
            &mut Transform,
            &GlobalTransform,
            &mut OrthographicProjection,
        ),
        With<bevy_pancam::PanCam>,
    >,
) {
    let mut camera = camera.get_single_mut().unwrap();
    camera.2.scale *= 1. + (z * 0.03);
    if camera.2.scale < 0. {
        camera.2.scale = 0.;
    }
}

fn fit_view_rect(re: &Query<&REntity>) -> Rect {
    let mut a = Vec::new();
    for re in re.iter() {
        match re {
            REntity::Arc(sp) => a.extend(&sp.definition),
            REntity::Circle(sp) => a.extend(&sp.definition),
            REntity::Line(sp) => a.extend(&sp.definition),
            REntity::Point(sp) => a.push(sp),
            REntity::Text(sp) => a.extend(&sp.bud_position),
            REntity::SnapPoint(_) => (),
            REntity::PhantomPoint => (),
        }
    }

    let (mut min_x, mut min_y, mut max_x, mut max_y) = match a.first() {
        None => (0., 0., 0., 0.),
        Some(point) => (
            point.coordinates.x,
            point.coordinates.y,
            point.coordinates.x,
            point.coordinates.y,
        ),
    };

    for point in a {
        if point.coordinates.x < min_x {
            min_x = point.coordinates.x;
        }
        if point.coordinates.x > max_x {
            max_x = point.coordinates.x;
        }
        if point.coordinates.y < min_y {
            min_y = point.coordinates.y;
        }
        if point.coordinates.y > max_y {
            max_y = point.coordinates.y;
        }
    }

    Rect::new(min_x, min_y, max_x, max_y)
}
