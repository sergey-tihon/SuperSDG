use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

//mod fps;

pub struct ToolsPlugins;

impl PluginGroup for ToolsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Extra tools
        //group = group.add(fps::FpsPlugin);

        #[cfg(debug_assertions)]
        {
            // Plugins for debugging and development
            //group = group.add(WorldInspectorPlugin::new());
        }

        group
    }
}
