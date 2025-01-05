use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};
use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub struct ToolsPlugins;

impl PluginGroup for ToolsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        if cfg!(debug_assertions) {
            // Plugins for debugging and development
            PluginGroupBuilder::start::<Self>()
                .add(FrameTimeDiagnosticsPlugin)
                .add(EntityCountDiagnosticsPlugin)
                .add(LogDiagnosticsPlugin::default())
        } else {
            PluginGroupBuilder::start::<Self>()
        }
    }
}
