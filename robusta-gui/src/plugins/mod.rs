pub mod camera;
pub mod construction;
pub mod diagnostic;
pub mod phantom;
pub mod selection;
pub mod snap;
pub mod unsorted;

use self::{
    camera::RCameraPlugin, construction::ConstructionPlugin, diagnostic::DiagnosticPlugin,
    phantom::PhantomPlugin, selection::RSelectionPlugin, snap::SnapPlugin,
    unsorted::UnsortedPlugin,
};

use super::*;

pub struct RCADPlugins;

impl bevy::app::PluginGroup for RCADPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut builder = bevy::app::PluginGroupBuilder::start::<Self>();

        builder = builder.add(UnsortedPlugin);
        builder = builder.add(RCameraPlugin);
        builder = builder.add(RSelectionPlugin);
        builder = builder.add(SnapPlugin);
        builder = builder.add(PhantomPlugin);
        builder = builder.add(ConstructionPlugin);
        builder = builder.add(DiagnosticPlugin);

        builder
    }
}
