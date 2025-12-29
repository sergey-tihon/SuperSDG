use bevy::ecs::system::SystemId;
use bevy::prelude::*;

use super::AppState;

pub const MENU_ZINDEX: i32 = i32::MAX - 10;

// ============================================================================
// Core Types
// ============================================================================

/// Action triggered when a menu item is selected
#[derive(Clone)]
pub enum MenuAction {
    /// Transition to a new app state
    ChangeState(AppState),
    /// Execute a one-shot system and optionally change state
    RunSystem {
        system: SystemId,
        next_state: Option<AppState>,
    },
    /// Exit the application (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    Exit,
}

/// Single menu item definition
#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
}

/// Menu definition passed to the plugin
pub struct MenuDef {
    pub items: Vec<MenuItem>,
}

/// Configuration for menu appearance
#[derive(Resource, Clone)]
pub struct MenuConfig {
    pub font_path: String,
    pub font_size: f32,
    pub selected_color: Color,
    pub normal_color: Color,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            font_path: "fonts/screen-diags-font.ttf".to_string(),
            font_size: 48.0,
            selected_color: Color::srgb(1.0, 0.4, 0.0),
            normal_color: Color::WHITE,
        }
    }
}

// ============================================================================
// Resources & Components
// ============================================================================

/// Tracks current selection index for the active menu
#[derive(Resource, Default)]
pub struct MenuSelection(pub usize);

/// Marker for menu root entity (for cleanup)
#[derive(Component)]
struct MenuRoot;

/// Marker for the dedicated menu camera
#[derive(Component)]
struct MenuCamera;

/// Component on each menu item storing its index
#[derive(Component)]
struct MenuItemIndex(usize);

/// Stores the menu definition for a specific state
#[derive(Resource, Clone)]
struct ActiveMenuItems(Vec<MenuItem>);

// ============================================================================
// Plugins
// ============================================================================

/// Main menu plugin - initializes shared resources
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuSelection>()
            .init_resource::<MenuConfig>();
    }
}

/// Plugin for the start menu state (AppState::Menu)
pub struct StartMenuPlugin {
    pub menu_def: MenuDef,
}

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        let items = self.menu_def.items.clone();

        app.insert_resource(StartMenuItems(items))
            .add_systems(OnEnter(AppState::Menu), setup_start_menu)
            .add_systems(OnExit(AppState::Menu), cleanup_menu)
            .add_systems(
                Update,
                (
                    navigate_menu,
                    activate_menu,
                    update_visuals,
                    esc_in_start_menu,
                )
                    .run_if(in_state(AppState::Menu)),
            );
    }
}

/// Plugin for the pause menu state (AppState::Paused)
pub struct PauseMenuPlugin {
    pub menu_def: MenuDef,
}

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        let items = self.menu_def.items.clone();

        app.insert_resource(PauseMenuItems(items))
            .add_systems(OnEnter(AppState::Paused), setup_pause_menu)
            .add_systems(OnExit(AppState::Paused), cleanup_menu)
            .add_systems(
                Update,
                (
                    navigate_menu,
                    activate_menu,
                    update_visuals,
                    esc_in_pause_menu,
                )
                    .run_if(in_state(AppState::Paused)),
            );
    }
}

/// Convenience plugin for ESC to pause (add to InGame state)
pub struct EscToPausePlugin;

impl Plugin for EscToPausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, esc_to_pause.run_if(in_state(AppState::InGame)));
    }
}

// ============================================================================
// Menu Item Storage Resources (one per menu state)
// ============================================================================

#[derive(Resource, Clone)]
struct StartMenuItems(Vec<MenuItem>);

#[derive(Resource, Clone)]
struct PauseMenuItems(Vec<MenuItem>);

// ============================================================================
// Systems
// ============================================================================

fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<MenuConfig>,
    items: Res<StartMenuItems>,
    mut selection: ResMut<MenuSelection>,
) {
    selection.0 = 0;
    commands.insert_resource(ActiveMenuItems(items.0.clone()));
    spawn_menu(&mut commands, &asset_server, &config, &items.0);
}

fn setup_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<MenuConfig>,
    items: Res<PauseMenuItems>,
    mut selection: ResMut<MenuSelection>,
) {
    selection.0 = 0;
    commands.insert_resource(ActiveMenuItems(items.0.clone()));
    spawn_menu(&mut commands, &asset_server, &config, &items.0);
}

fn spawn_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    config: &Res<MenuConfig>,
    items: &[MenuItem],
) {
    // Spawn dedicated 2D camera for menu UI
    let camera_entity = commands
        .spawn((
            Camera2d,
            Camera {
                order: 100, // Render after 3D camera
                ..default()
            },
            MenuCamera,
            MenuRoot,
        ))
        .id();

    // Spawn menu UI
    commands
        .spawn((
            UiTargetCamera(camera_entity),
            GlobalZIndex(MENU_ZINDEX),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            MenuRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|col| {
                    for (idx, item) in items.iter().enumerate() {
                        col.spawn((
                            Text::new(&item.label),
                            TextFont {
                                font: asset_server.load(&config.font_path),
                                font_size: config.font_size,
                                ..default()
                            },
                            TextColor(if idx == 0 {
                                config.selected_color
                            } else {
                                config.normal_color
                            }),
                            MenuItemIndex(idx),
                        ));
                    }
                });
        });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn navigate_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<MenuSelection>,
    items: Res<ActiveMenuItems>,
) {
    let count = items.0.len();
    if count == 0 {
        return;
    }

    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        selection.0 = if selection.0 == 0 {
            count - 1
        } else {
            selection.0 - 1
        };
    }

    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        selection.0 = (selection.0 + 1) % count;
    }
}

fn activate_menu(
    keys: Res<ButtonInput<KeyCode>>,
    selection: Res<MenuSelection>,
    items: Res<ActiveMenuItems>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    #[cfg(not(target_arch = "wasm32"))] windows: Query<(Entity, &Window)>,
) {
    if !(keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space)) {
        return;
    }

    let Some(item) = items.0.get(selection.0) else {
        return;
    };

    match &item.action {
        MenuAction::ChangeState(state) => {
            next_state.set(state.clone());
        }
        MenuAction::RunSystem {
            system,
            next_state: state,
        } => {
            commands.run_system(*system);
            if let Some(s) = state {
                next_state.set(s.clone());
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        MenuAction::Exit => {
            for (entity, window) in &windows {
                if window.focused {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn update_visuals(
    selection: Res<MenuSelection>,
    config: Res<MenuConfig>,
    mut query: Query<(&MenuItemIndex, &mut TextColor)>,
) {
    for (item, mut color) in &mut query {
        color.0 = if item.0 == selection.0 {
            config.selected_color
        } else {
            config.normal_color
        };
    }
}

fn esc_in_start_menu(
    keys: Res<ButtonInput<KeyCode>>,
    #[cfg(not(target_arch = "wasm32"))] windows: Query<(Entity, &Window)>,
    #[cfg(not(target_arch = "wasm32"))] mut commands: Commands,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    // ESC in main menu -> exit (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    for (entity, window) in &windows {
        if window.focused {
            commands.entity(entity).despawn();
        }
    }
}

fn esc_in_pause_menu(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::InGame);
    }
}

fn esc_to_pause(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}
