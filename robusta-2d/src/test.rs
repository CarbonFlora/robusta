use bevy::prelude::*;

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
