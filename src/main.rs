use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution, WindowTheme};

mod core;
mod maze;
mod tools;

use core::menu::{EscToPausePlugin, MenuAction, MenuDef, MenuItem, MenuPlugin, MenuScreenPlugin};
use core::{AppState, GameSettings};

fn main() {
    let mut app = App::new();

    // Register systems to get SystemIds
    let restart_system_id = app.register_system(maze::level::regenerate_maze);

    // Build menu definitions
    let start_menu = build_start_menu();
    let pause_menu = build_pause_menu(restart_system_id);

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SuperSDG 3".to_string(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                mode: WindowMode::Windowed,
                position: WindowPosition::At(IVec2::new(0, 0)),
                resolution: WindowResolution::new(1280, 1460),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }),
        maze::MazeGamePlugin,
        core::ui::GameUiPlugin,
        tools::ToolsPlugins,
        // Menu system
        MenuPlugin,
        MenuScreenPlugin {
            state: AppState::Menu,
            menu_def: start_menu,
        },
        MenuScreenPlugin {
            state: AppState::Paused,
            menu_def: pause_menu,
        },
        EscToPausePlugin,
    ))
    .init_resource::<GameSettings>()
    .init_state::<AppState>()
    .run();
}

fn build_start_menu() -> MenuDef {
    #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
    let mut items = vec![MenuItem {
        label: "New Game".to_string(),
        action: MenuAction::ChangeState(AppState::InGame),
    }];

    items.push(MenuItem {
        label: "Size: 30x30".to_string(),
        action: MenuAction::CycleComplexity,
    });

    #[cfg(not(target_arch = "wasm32"))]
    items.push(MenuItem {
        label: "Exit".to_string(),
        action: MenuAction::Exit,
    });

    #[cfg(not(target_arch = "wasm32"))]
    let on_escape = Some(MenuAction::Exit);
    #[cfg(target_arch = "wasm32")]
    let on_escape = None;

    MenuDef { items, on_escape }
}

fn build_pause_menu(restart_system_id: SystemId) -> MenuDef {
    #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
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
            label: "Size: 30x30".to_string(),
            action: MenuAction::CycleComplexity,
        },
    ];

    #[cfg(not(target_arch = "wasm32"))]
    items.push(MenuItem {
        label: "Exit".to_string(),
        action: MenuAction::Exit,
    });

    MenuDef {
        items,
        on_escape: Some(MenuAction::ChangeState(AppState::InGame)),
    }
}
