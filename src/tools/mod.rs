use bevy::app::{PluginGroup, PluginGroupBuilder};

#[cfg(feature = "dev")]
mod debug;
mod fps_overlay;

pub struct ToolsPlugins;

impl PluginGroup for ToolsPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(fps_overlay::FpsOverlayPlugin::default());
        #[cfg(feature = "dev")]
        {
            // Plugins for debugging and development
            group = group.add(debug::plugin)
        }

        group
    }
}
