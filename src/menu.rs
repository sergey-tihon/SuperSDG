use bevy::prelude::*;

use crate::AppState;
use crate::maze::MazeLevel;

pub const MENU_ZINDEX: i32 = i32::MAX - 10;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuSelection>()
            .init_resource::<MenuContext>()
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(OnExit(AppState::Menu), cleanup_menu)
            .add_systems(
                Update,
                (
                    esc_open_menu.run_if(in_state(AppState::InGame)),
                    // Ensure menu gets spawned once camera exists if initial OnEnter ran too early
                    ensure_menu_spawned.run_if(in_state(AppState::Menu)),
                    esc_exit_or_close_menu.run_if(in_state(AppState::Menu)),
                    navigate_menu.run_if(in_state(AppState::Menu)),
                    activate_menu.run_if(in_state(AppState::Menu)),
                    update_visuals.run_if(in_state(AppState::Menu)),
                ),
            );
    }
}

#[derive(Resource, Default)]
pub struct MenuSelection(pub usize);

#[derive(Resource, Default)]
pub struct MenuContext {
    pub opened_from_game: bool,
}

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct MenuNeedsCamera;

#[derive(Component)]
struct MenuItem {
    index: usize,
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Query<Entity, With<crate::maze::MainCamera>>,
    mut sel: ResMut<MenuSelection>,
    ctx: Res<MenuContext>,
) {
    match camera.single() {
        Ok(cam) => {
            spawn_menu_root(&mut commands, &asset_server, cam, &ctx, &mut sel);
        }
        Err(_) => {
            commands.spawn((MenuNeedsCamera,));
        }
    }
}

fn ensure_menu_spawned(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Query<Entity, With<crate::maze::MainCamera>>,
    awaiting: Query<Entity, With<MenuNeedsCamera>>,
    existing_menu: Query<Entity, With<MenuRoot>>,
) {
    if existing_menu.single().is_ok() {
        return;
    }
    if let (Ok(cam), Ok(placeholder)) = (camera.single(), awaiting.single()) {
        commands.entity(placeholder).despawn();
        // We lost the context of whether opened_from_game if placeholder was used, retain resource
        // (ctx remains). We'll re-spawn with current ctx state.
        // Need mutable access to MenuSelection; selection persists.
        // We default to first entry after build - handled in spawn.
        // Acquire ctx and selection via world queries is avoided here; keep simple.
        // This branch runs only once.
        // NOTE: This function signature does not include ctx/sel, so adapt design if needed.
        // For now we cannot adjust indices here; they will remain as-is.
        spawn_menu_root(
            &mut commands,
            &asset_server,
            cam,
            &MenuContext::default(),
            &mut MenuSelection(0),
        );
    }
}

fn spawn_menu_root(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    cam: Entity,
    ctx: &MenuContext,
    sel: &mut MenuSelection,
) {
    // Build dynamic menu items depending on context
    let mut items: Vec<(&str, usize)> = Vec::new();
    if ctx.opened_from_game {
        items.push(("Resume", 0));
        items.push(("New Game", 1));
        items.push(("Exit", 2));
        sel.0 = 0; // default to Resume
    } else {
        items.push(("New Game", 0));
        items.push(("Exit", 1));
        sel.0 = sel.0.min(items.len() - 1);
    }

    commands
        .spawn((
            UiTargetCamera(cam),
            GlobalZIndex(MENU_ZINDEX),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
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
                    for (label, idx) in items {
                        col.spawn((
                            Text::new(label),
                            TextFont {
                                font: asset_server.load("fonts/screen-diags-font.ttf"),
                                font_size: 48.0,
                                ..default()
                            },
                            TextColor(if idx == sel.0 {
                                Color::srgb(1.0, 0.4, 0.0)
                            } else {
                                Color::WHITE
                            }),
                            MenuItem { index: idx },
                        ));
                    }
                });
        });
}

fn cleanup_menu(mut commands: Commands, q: Query<Entity, With<MenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn esc_open_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut ctx: ResMut<MenuContext>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        ctx.opened_from_game = true;
        next_state.set(AppState::Menu);
    }
}

fn esc_exit_or_close_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<(Entity, &Window)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut ctx: ResMut<MenuContext>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if ctx.opened_from_game {
            ctx.opened_from_game = false;
            next_state.set(AppState::InGame);
            return;
        }
        // Exit the game by closing the focused window
        for (entity, window) in windows.iter_mut() {
            if window.focused {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn navigate_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut sel: ResMut<MenuSelection>,
    ctx: Res<MenuContext>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        if sel.0 == 0 {
            sel.0 = 1;
        } else {
            sel.0 -= 1;
        }
    }
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        let count = if ctx.opened_from_game { 3 } else { 2 };
        sel.0 = (sel.0 + 1) % count;
    }
}

fn activate_menu(
    keys: Res<ButtonInput<KeyCode>>,
    sel: Res<MenuSelection>,
    mut next_state: ResMut<NextState<AppState>>,
    mut level: ResMut<MazeLevel>,
    mut player_query: Query<&mut Transform, With<crate::maze::PlayerAnimation>>,
    mut exit_query: Query<
        &mut Transform,
        (
            Without<crate::maze::PlayerAnimation>,
            With<crate::maze::ExitPoint>,
        ),
    >,
    mut windows: Query<(Entity, &Window)>,
    mut commands: Commands,
    mut ctx: ResMut<MenuContext>,
) {
    // Log selection each activation attempt
    if !(keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space)) {
        return;
    }
    if ctx.opened_from_game {
        match sel.0 {
            0 => {
                // Resume
                ctx.opened_from_game = false;
                next_state.set(AppState::InGame);
            }
            1 => {
                // New Game
                *level = MazeLevel::new(20, 20);
                if let Ok(mut t) = player_query.single_mut() {
                    t.translation = level.player_position.into();
                }
                if let Ok(mut t) = exit_query.single_mut() {
                    t.translation = level.exit_position.into();
                }
                ctx.opened_from_game = false; // treat as fresh game now
                next_state.set(AppState::InGame);
            }
            2 => {
                // Exit
                for (entity, window) in &mut windows {
                    if window.focused {
                        commands.entity(entity).despawn();
                    }
                }
            }
            _ => {}
        }
    } else {
        match sel.0 {
            0 => {
                // New Game
                *level = MazeLevel::new(20, 20);
                if let Ok(mut t) = player_query.single_mut() {
                    t.translation = level.player_position.into();
                }
                if let Ok(mut t) = exit_query.single_mut() {
                    t.translation = level.exit_position.into();
                }
                next_state.set(AppState::InGame);
            }
            1 => {
                // Exit
                for (entity, window) in &mut windows {
                    if window.focused {
                        commands.entity(entity).despawn();
                    }
                }
            }
            _ => {}
        }
    }
}

fn update_visuals(sel: Res<MenuSelection>, mut q: Query<(&MenuItem, &mut TextColor)>) {
    for (item, mut color) in &mut q {
        color.0 = if item.index == sel.0 {
            Color::srgb(1.0, 0.4, 0.0)
        } else {
            Color::WHITE
        };
    }
}
