use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{
    camera::MazeCameraPlugin, level::MazeLevelPlugin, light::MazeLightPlugin, player::PlayerPlugin,
    render::MazeRenderPlugin,
};

mod camera;
mod level;
mod light;
mod player;
mod render;

pub struct MazeGamePlugins;

impl PluginGroup for MazeGamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Core Game Plugins
        group = group
            .add(MazeLevelPlugin)
            .add(PlayerPlugin)
            .add(MazeCameraPlugin)
            .add(MazeLightPlugin)
            .add(MazeRenderPlugin);

        group
    }
}
