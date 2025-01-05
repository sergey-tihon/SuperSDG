use bevy::app::{PluginGroup, PluginGroupBuilder};

#[cfg(feature = "dev")]
mod debug;
#[cfg(feature = "dev")]
mod fps_overlay;

pub struct ToolsPlugins;

impl PluginGroup for ToolsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        #[cfg(feature = "dev")]
        {
            // Plugins for debugging and development
            PluginGroupBuilder::start::<Self>()
                .add(debug::plugin)
                .add(fps_overlay::plugin)
        }
        #[cfg(not(feature = "dev"))]
        {
            PluginGroupBuilder::start::<Self>()
        }
    }
}
