use bevy::prelude::*;

pub mod menu;

/// Application states for the game
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Menu, // Start menu (initial state)
    InGame, // Playing the game
    Paused, // In-game pause menu
}
