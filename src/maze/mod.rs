use bevy::{
    app::PluginGroupBuilder,
    prelude::{PluginGroup, SystemSet},
};

use self::{
    camera::MazeCameraPlugin, level::MazeLevelPlugin, light::MazeLightPlugin,
    mini_map::MiniMapPlugin, player::PlayerPlugin, render::MazeRenderPlugin,
};

mod camera;
mod fps_overlay;
mod level;
mod light;
mod mini_map;
mod player;
mod render;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CameraSwawned;

pub use camera::MainCamera;

pub struct MazeGamePlugins;

impl PluginGroup for MazeGamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Core Game Plugins
        group = group
            .add(MazeLevelPlugin)
            .add(PlayerPlugin)
            .add(MazeCameraPlugin)
            .add(fps_overlay::FpsOverlayPlugin::default())
            .add(MiniMapPlugin)
            .add(MazeLightPlugin)
            .add(MazeRenderPlugin);

        group
    }
}
