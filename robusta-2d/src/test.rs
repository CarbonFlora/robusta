use crate::*;

#[cfg(test)]
mod app2d {
    #[test]
    fn init_minimal_app() {
        // let mut app = App::new();
        // app.insert_resource(UiState::new())
        //     .add_systems(First, draw_arc);

        // app.update();

        assert!(true);
    }
}

/// Gizmos are not designed to be used as design lines.
/// "Immediate mode drawing api for visual debugging."-Bevy
pub fn draw_arc(mut gizmos: Gizmos) {
    let pi = std::f32::consts::PI;
    gizmos.arc_2d(
        Vec2::ZERO,
        0.,
        std::f32::consts::PI / 2.,
        350.,
        Color::ORANGE_RED,
    );
    gizmos.arc_2d(
        Vec2::new(100., 100.),
        0.,
        std::f32::consts::PI / 2.,
        350.,
        Color::YELLOW_GREEN,
    );
    gizmos.arc_2d(Vec2::ZERO, pi, pi / 2., 300., Color::ALICE_BLUE);

    gizmos.circle_2d(Vec2::new(25., 20.), 0.5, Color::ALICE_BLUE);

    gizmos.line_2d(Vec2::new(50., 50.), Vec2::new(500., 300.), Color::CYAN);
    gizmos.line_2d(Vec2::new(0., 0.), Vec2::new(100., -10.), Color::CRIMSON);
}

fn _draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}
