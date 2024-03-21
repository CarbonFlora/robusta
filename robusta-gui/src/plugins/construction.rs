use self::{line::Line, phantom::PhantomAct, point::Point, snap::UpdateSnapPoints};

use super::*;

pub struct ConstructionPlugin;
impl bevy::app::Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConstructionBuffer::new())
            .add_event::<ConstructionInput>()
            .add_systems(Update, update_queue);
        // .add_systems(Update, update_construction);
    }
}

#[derive(Debug, Event, Clone, PartialEq)]
pub struct ConstructionInput {
    pub coords: Vec3,
}

#[derive(Debug, Clone, Default)]
pub enum InputType {
    #[default]
    Cursor,
    Coordinates,
}

/// This is a buffer to store what the next pointer action should do.
#[derive(Debug, Resource, Default)]
pub struct ConstructionBuffer {
    buf: Vec<ConstructionInput>,
    pub build: Option<ConstructType>,
}

impl ConstructionBuffer {
    pub fn new() -> Self {
        self::default()
    }

    pub fn reset(&mut self) {
        self.buf = Vec::new();
        self.build = None;
    }
}

#[derive(Debug, Component)]
pub struct RConstructionEntity;

#[derive(Debug, Event, Clone, Copy, PartialEq)]
pub enum ConstructType {
    Arc,
    Circle,
    LineBy2Click,
    PointBy1Click,
    Text,
}

#[allow(clippy::too_many_arguments)]
fn update_queue(
    //Input
    mut erra: EventReader<ConstructionInput>,
    //Util
    mut rmcb: ResMut<ConstructionBuffer>,
    //Output
    mut ewrsp: EventWriter<UpdateSnapPoints>,
    mut ewre: EventWriter<REntity>,
    mut ewpa: EventWriter<PhantomAct>,
) {
    if erra.is_empty() {
        return;
    }

    if let Some(w) = erra.read().next() {
        rmcb.buf.push(w.clone());
        rmcb.buf.dedup();
    }

    match rmcb
        .build
        .as_ref()
        .expect("Encountered construction with no build criteria.")
    {
        ConstructType::Arc => todo!(),
        ConstructType::Circle => todo!(),
        ConstructType::LineBy2Click => {
            if rmcb.buf.len() == 2 {
                let pt1 = rmcb.buf[0].coords;
                let pt2 = rmcb.buf[1].coords;
                ewre.send(REntity::Line(Line::new([
                    Point::new(pt1.x, pt1.y, pt1.z),
                    Point::new(pt2.x, pt2.y, pt2.z),
                ])));
                ewrsp.send(UpdateSnapPoints(false));
                ewpa.send(PhantomAct::DespawnAll);
                rmcb.into_inner().reset();
            }
        }
        ConstructType::PointBy1Click => {
            if rmcb.buf.len() == 1 {
                let pt1 = rmcb.buf[0].coords;
                ewre.send(REntity::Point(Point::new(pt1.x, pt1.y, pt1.z)));
                ewrsp.send(UpdateSnapPoints(false));
                ewpa.send(PhantomAct::DespawnAll);
                rmcb.into_inner().reset();
            }
        }
        ConstructType::Text => todo!(),
    }
}

impl std::fmt::Display for ConstructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            ConstructType::Arc => "Arc",
            ConstructType::Circle => "Circle",
            ConstructType::LineBy2Click => "Line",
            ConstructType::PointBy1Click => "Point",
            ConstructType::Text => "Text",
        };
        f.write_fmt(format_args!("{}", a))
    }
}
