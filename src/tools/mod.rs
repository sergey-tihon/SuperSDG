use bevy::app::{PluginGroup, PluginGroupBuilder};

mod debug;
mod fps_overlay;

pub struct ToolsPlugins;

impl PluginGroup for ToolsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        if cfg!(feature = "dev") {
            // Plugins for debugging and development
            PluginGroupBuilder::start::<Self>()
                .add(debug::plugin)
                .add(fps_overlay::plugin)
        } else {
            PluginGroupBuilder::start::<Self>()
        }
    }
}
