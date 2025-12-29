use bevy::{
    app::PluginGroupBuilder,
    prelude::{Component, PluginGroup, SystemSet},
};

use self::{
    camera::MazeCameraPlugin, level::MazeLevelPlugin, light::MazeLightPlugin,
    mini_map::MiniMapPlugin, player::PlayerPlugin, render::MazeRenderPlugin,
};

mod camera;
mod fps_overlay;
mod help_overlay;
mod light;
mod mini_map;
mod render;

pub mod level;
pub mod player;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CameraSwawned;

/// SystemSet that marks the overlay camera as spawned
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OverlayCameraSpawned;

/// Shared 2D camera for all overlays (FPS, help, etc.)
#[derive(Component)]
pub struct OverlayCamera;

pub use level::MazeLevel;

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
            .add(help_overlay::HelpOverlayPlugin::default())
            .add(MiniMapPlugin)
            .add(MazeLightPlugin)
            .add(MazeRenderPlugin);

        group
    }
}
