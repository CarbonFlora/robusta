use self::point::Point;

use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    pub bud_position: crate::point::Point,
    pub body: String,
    pub rotation: f32,
    pub height: f32,
    pub leader: Option<Leader>,
    pub horizontal_tj: HorizontalTJ,
    pub vertical_tj: VerticalTJ,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, PartialOrd)]
pub enum HorizontalTJ {
    #[default]
    Left,
    Center,
    Right,
    Aligned,
    Middle,
    Fit,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, PartialOrd)]
pub enum VerticalTJ {
    #[default]
    Baseline,
    Bottom,
    Middle,
    Top,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextSpec {
    pub corners: [crate::point::Point; 4], //follows quadrant standard. NE=0, NW=1, SW=2, SE=3
    pub midpoints: [crate::point::Point; 4], //follows NWSE
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Leader {
    pub seed_pos: [crate::point::Point; 1],
    pub length: f32,
    pub rotation: f32,
}

impl Text {
    pub fn new(origin: crate::point::Point) -> Self {
        Text {
            bud_position: origin,
            height: 1.,
            body: "".to_string(),
            rotation: 0.,
            leader: None,
            horizontal_tj: HorizontalTJ::default(),
            vertical_tj: VerticalTJ::default(),
        }
    }

    // pub fn specifications(&self) -> TextSpec {
    //     let corners = [];
    //     let midpoints = [];

    //     TextSpec { corners, midpoints }
    // }

    pub fn min_max(&self) -> (f32, f32, f32, f32) {
        // This is temp as text is not implimented.
        let x1 = self.bud_position.coordinates.x - 1.0;
        let y1 = self.bud_position.coordinates.y - 1.0;
        let x2 = self.bud_position.coordinates.x + 1.0;
        let y2 = self.bud_position.coordinates.y + 1.0;

        crate::min_max(&[Point::new(x1, y1, 0.), Point::new(x2, y2, 0.)])
    }

    pub fn mesh(
        &self,
        tf: &TagFlags,
        me: &mut ResMut<Assets<Mesh>>,
        ma: &mut ResMut<Assets<ColorMaterial>>,
        tz: &mut TopZLayer,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: me
                .add(bevy::math::primitives::Rectangle::new(
                    tf.thickness_or_default(),
                    tf.thickness_or_default(),
                ))
                .into(),
            material: ma.add(ColorMaterial::from(tf.color_or_default())),
            transform: Transform::from_translation(Vec3::new(
                self.bud_position.coordinates.x,
                self.bud_position.coordinates.y,
                tz.top() as f32,
            )),
            ..default()
        }
    }

    pub fn text_mesh(&self, tz: &mut TopZLayer) -> Text2dBundle {
        let text_body = bevy::text::Text::from_section(
            self.body.clone(),
            TextStyle {
                font_size: self.height * 50., //This magic # should be connected to the zoom level of the viewport camera(pancam).
                ..default()
            },
        );

        Text2dBundle {
            text: text_body,
            text_anchor: bevy::sprite::Anchor::BottomLeft,
            transform: Transform::from_translation(Vec3::new(0., 0., tz.top() as f32))
                .with_scale(Vec3::new(self.height / 50., self.height / 50., 1.))
                .with_rotation(Quat::from_rotation_z(self.rotation)),
            // .with_scale(Vec3::new(self.height / 5., self.height / 5., 1.)),
            // text_layout_info: bevy::text::TextLayoutInfo::default(),
            ..default()
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Origin: {}", self.bud_position))
    }
}
