use bevy::prelude::*;

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

pub struct MazeGamePlugin;

impl Plugin for MazeGamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MazeSystems::Input,
                MazeSystems::Movement,
                MazeSystems::Camera,
                MazeSystems::Lighting,
            )
                .chain(),
        )
        .add_plugins((
            MazeLevelPlugin,
            PlayerPlugin,
            MazeCameraPlugin,
            fps_overlay::FpsOverlayPlugin,
            help_overlay::HelpOverlayPlugin,
            MiniMapPlugin,
            MazeLightPlugin,
            MazeRenderPlugin,
        ));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MazeSystems {
    Input,
    Movement,
    Camera,
    Lighting,
}
