use self::{arc::Arc, line::Line, phantom::PhantomAct, snap::UpdateSnapPoints};

use super::*;

pub struct ConstructionPlugin;
impl bevy::app::Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConstructionBuffer::new())
            .add_event::<ConstructionInput>()
            .add_systems(Update, update_construction);
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
    ArcByEndEndMid,
    Circle,
    LineBy2Click,
    PointBy1Click,
    Text,
}

pub fn insert(
    oct: &ConstructType,
    rmcb: &mut ResMut<ConstructionBuffer>,
    ewre: &mut EventWriter<REntity>,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
) {
    rmcb.build = Some(*oct);
    ewrsp.send(UpdateSnapPoints(true));
    ewre.send(REntity::PhantomPoint);
}

#[allow(clippy::too_many_arguments)]
fn update_construction(
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

    let ore: Option<REntity> = match rmcb
        .build
        .as_ref()
        .expect("Encountered construction with no build criteria.")
    {
        ConstructType::ArcByEndEndMid => {
            if rmcb.buf.len() == 3 {
                Some(REntity::Arc(Arc::new([
                    rmcb.buf[0].coords.into(),
                    rmcb.buf[1].coords.into(),
                    rmcb.buf[2].coords.into(),
                ])))
            } else {
                None
            }
        }
        ConstructType::Circle => todo!(),
        ConstructType::LineBy2Click => {
            if rmcb.buf.len() == 2 {
                Some(REntity::Line(Line::new([
                    rmcb.buf[0].coords.into(),
                    rmcb.buf[0].coords.into(),
                ])))
            } else {
                None
            }
        }
        ConstructType::PointBy1Click => {
            if rmcb.buf.len() == 1 {
                Some(REntity::Point(rmcb.buf[0].coords.into()))
            } else {
                None
            }
        }
        ConstructType::Text => todo!(),
    };
    if let Some(re) = ore {
        ewre.send(re);
        ewrsp.send(UpdateSnapPoints(false));
        ewpa.send(PhantomAct::DespawnAll);
        rmcb.into_inner().reset();
    }
}

impl std::fmt::Display for ConstructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            ConstructType::ArcByEndEndMid => "Arc",
            ConstructType::Circle => "Circle",
            ConstructType::LineBy2Click => "Line by 2 points",
            ConstructType::PointBy1Click => "Point by 1 click.",
            ConstructType::Text => "Text",
        };
        f.write_fmt(format_args!("{}", a))
    }
}
