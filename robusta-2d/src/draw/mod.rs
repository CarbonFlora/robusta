use crate::*;

/// This actually draws from ViewportState.
pub fn draw_dxf(viewport_state: Res<ViewportState>, mut gizmos: Gizmos) {
    for point in &viewport_state.points {
        // println!("point drawn: {:?}", point);
        gizmos.circle_2d(
            Vec2::new(point.coordinates.x, point.coordinates.y),
            1.,
            Color::ALICE_BLUE,
        );
    }
}
