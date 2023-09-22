use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{camera::MazeCameraPlugin, light::MazeLightPlugin, render::MazeRenderPlugin};

mod camera;
mod light;
mod render;

pub struct MazePlugins;

impl PluginGroup for MazePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MazeCameraPlugin)
            .add(MazeLightPlugin)
            .add(MazeRenderPlugin)
    }
}
