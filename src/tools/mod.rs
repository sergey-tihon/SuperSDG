use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::PluginGroup,
};

pub struct ToolsPlugins;

impl PluginGroup for ToolsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Extra tools

        #[cfg(debug_assertions)]
        {
            // Plugins for debugging and development
            group = group
                .add(FrameTimeDiagnosticsPlugin)
                .add(EntityCountDiagnosticsPlugin);
        }

        group
    }
}
