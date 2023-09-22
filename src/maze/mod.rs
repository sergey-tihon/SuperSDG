use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{
    camera::MazeCameraPlugin, level::MazeLevelPlugin, light::MazeLightPlugin,
    render::MazeRenderPlugin,
};

mod camera;
mod level;
mod light;
mod render;

pub struct MazePlugins;

impl PluginGroup for MazePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MazeLevelPlugin)
            .add(MazeCameraPlugin)
            .add(MazeLightPlugin)
            .add(MazeRenderPlugin)
    }
}
