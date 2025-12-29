use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution, WindowTheme};

mod core;
mod maze;
mod tools;

use core::menu::{
    EscToPausePlugin, MenuAction, MenuDef, MenuItem, MenuPlugin, PauseMenuPlugin, StartMenuPlugin,
};
use core::{AppState, GameSettings};

/// Flag to indicate a new game was requested (vs resuming from pause)
#[derive(Resource, Default)]
struct NewGameRequested(bool);

fn main() {
    let mut app = App::new();

    // Register systems to get SystemIds
    let new_game_system_id = app.register_system(new_game);
    let restart_system_id = app.register_system(restart_game);

    // Build menu definitions
    let start_menu = build_start_menu(new_game_system_id);
    let pause_menu = build_pause_menu(restart_system_id);

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SuperSDG 3".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    mode: WindowMode::Windowed,
                    position: WindowPosition::At(IVec2 { x: 0, y: 0 }),
                    resolution: WindowResolution::new(1280, 1460),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin { ..default() }),
        maze::MazeGamePlugins,
        tools::ToolsPlugins,
        // Menu system
        MenuPlugin,
        StartMenuPlugin {
            menu_def: start_menu,
        },
        PauseMenuPlugin {
            menu_def: pause_menu,
        },
        EscToPausePlugin,
    ))
    .init_resource::<GameSettings>()
    .init_resource::<NewGameRequested>()
    .init_state::<AppState>()
    .add_systems(OnEnter(AppState::InGame), apply_complexity)
    .run();
}

fn build_start_menu(new_game_system_id: SystemId) -> MenuDef {
    let mut items = vec![MenuItem {
        label: "New Game".to_string(),
        action: MenuAction::RunSystem {
            system: new_game_system_id,
            next_state: Some(AppState::InGame),
        },
    }];

    items.push(MenuItem {
        label: "Size: 30x30".to_string(), // Label updated dynamically
        action: MenuAction::CycleComplexity,
    });

    #[cfg(not(target_arch = "wasm32"))]
    items.push(MenuItem {
        label: "Exit".to_string(),
        action: MenuAction::Exit,
    });

    MenuDef { items }
}

fn build_pause_menu(restart_system_id: SystemId) -> MenuDef {
    let mut items = vec![
        MenuItem {
            label: "Resume".to_string(),
            action: MenuAction::ChangeState(AppState::InGame),
        },
        MenuItem {
            label: "Restart".to_string(),
            action: MenuAction::RunSystem {
                system: restart_system_id,
                next_state: Some(AppState::InGame),
            },
        },
        MenuItem {
            label: "Size: 30x30".to_string(), // Label updated dynamically
            action: MenuAction::CycleComplexity,
        },
    ];

    #[cfg(not(target_arch = "wasm32"))]
    items.push(MenuItem {
        label: "Exit".to_string(),
        action: MenuAction::Exit,
    });

    MenuDef { items }
}

fn new_game(mut new_game_requested: ResMut<NewGameRequested>) {
    new_game_requested.0 = true;
}

fn restart_game(mut level: ResMut<maze::MazeLevel>, settings: Res<GameSettings>) {
    let (x, y) = settings.complexity.maze_size();
    level.regenerate_with_size(x, y);
}

fn apply_complexity(
    mut level: ResMut<maze::MazeLevel>,
    settings: Res<GameSettings>,
    mut new_game_requested: ResMut<NewGameRequested>,
) {
    if !new_game_requested.0 {
        return;
    }
    new_game_requested.0 = false;

    let (x, y) = settings.complexity.maze_size();
    if level.dimensions() != (x, y) {
        level.regenerate_with_size(x, y);
    }
}
