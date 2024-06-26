use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct ToolsPlugins;

impl PluginGroup for ToolsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Extra tools
        group = group.add(bevy_fps_counter::FpsCounterPlugin);

        #[cfg(debug_assertions)]
        {
            // Plugins for debugging and development
            group = group.add(WorldInspectorPlugin::new());
        }

        group
    }
}
