use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
    pub bud_pos: [crate::point::Point; 1],
    pub body: String,
    pub rotation: f32,
    pub height: f32,
    pub leader: Option<Leader>,
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
            bud_pos: [origin],
            body: String::new(),
            rotation: 0.,
            height: 10.,
            leader: None,
        }
    }

    pub fn min_max(&self) -> (f32, f32, f32, f32) {
        // This is temp as text is not implimented.
        crate::min_max(&self.bud_pos.clone())
    }

    pub fn mesh(
        &self,
        me: &mut ResMut<Assets<Mesh>>,
        ma: &mut ResMut<Assets<ColorMaterial>>,
        tz: &mut TopZLayer,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        todo!()
    }

    pub fn text_mesh(&self, tz: &mut TopZLayer) -> Text2dBundle {
        let text_body = bevy::text::Text::from_section(self.body.clone(), TextStyle::default());
        let origin = self.bud_pos[0].xyz();

        Text2dBundle {
            text: text_body,
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_translation(Vec3::new(
                origin[0],
                origin[1],
                tz.top() as f32,
            ))
            .with_rotation(Quat::from_rotation_z(self.rotation))
            .with_scale(Vec3::new(self.height / 5., self.height / 5., 1.)),
            text_layout_info: bevy::text::TextLayoutInfo::default(),
            ..default()
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Origin: {}", self.bud_pos[0]))
    }
}
