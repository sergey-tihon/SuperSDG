use bevy::app::{PluginGroup, PluginGroupBuilder};

#[cfg(feature = "dev")]
mod debug;

pub struct ToolsPlugins;

impl PluginGroup for ToolsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        #[allow(unused_mut)]
        let mut group = PluginGroupBuilder::start::<Self>();
        #[cfg(feature = "dev")]
        {
            // Plugins for debugging and development
            group = group.add(debug::plugin)
        }

        group
    }
}
