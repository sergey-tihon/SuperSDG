use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution, WindowTheme};

mod core;
mod maze;
mod tools;

use core::AppState;
use core::menu::{
    EscToPausePlugin, MenuAction, MenuDef, MenuItem, MenuPlugin, PauseMenuPlugin, StartMenuPlugin,
};

fn main() {
    let mut app = App::new();

    // Register restart system to get SystemId
    let restart_system_id = app.register_system(restart_game);

    // Build menu definitions
    let start_menu = build_start_menu();
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
    .init_state::<AppState>()
    .run();
}

fn build_start_menu() -> MenuDef {
    let mut items = vec![MenuItem {
        label: "New Game".to_string(),
        action: MenuAction::ChangeState(AppState::InGame),
    }];

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
    ];

    #[cfg(not(target_arch = "wasm32"))]
    items.push(MenuItem {
        label: "Exit".to_string(),
        action: MenuAction::Exit,
    });

    MenuDef { items }
}

fn restart_game(mut level: ResMut<maze::MazeLevel>) {
    level.regenerate();
}
