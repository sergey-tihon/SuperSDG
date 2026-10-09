use bevy::ecs::system::SystemId;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::picking::prelude::PointerOver;
use bevy::prelude::*;
use bevy::scene::Ready;
use bevy::text::FontSourceTemplate;
use bevy::ui_widgets::{Activate, Button};

use super::{AppState, GameSettings};

pub const MENU_ZINDEX: i32 = i32::MAX - 10;

const MENU_FONT_PATH: &str = "fonts/screen-diags-font.ttf";
const MENU_FONT_REM: f32 = 2.4;
const SELECTED_COLOR: Color = Color::srgb(1.0, 0.4, 0.0);
const NORMAL_COLOR: Color = Color::WHITE;
const BACKDROP_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);

#[derive(Clone)]
pub enum MenuAction {
    ChangeState(AppState),
    RunSystem {
        system: SystemId,
        next_state: Option<AppState>,
    },
    CycleComplexity,
    #[cfg(not(target_arch = "wasm32"))]
    Exit,
}

#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
}

#[derive(Clone)]
pub struct MenuDef {
    pub items: Vec<MenuItem>,
    pub on_escape: Option<MenuAction>,
}

#[derive(Component, Clone)]
pub struct MenuScreen {
    on_escape: Option<MenuAction>,
}

#[derive(Component, Clone)]
struct MenuEntry {
    index: usize,
    action: MenuAction,
}
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(run_menu_action).add_systems(
            Update,
            (
                navigate_menu.run_if(any_with_component::<MenuScreen>),
                highlight_focused_item
                    .run_if(resource_changed::<InputFocus>)
                    .after(navigate_menu),
                menu_escape,
                update_complexity_labels.run_if(resource_changed::<GameSettings>),
            ),
        );
    }
}

#[derive(Event, Clone)]
pub struct RunMenuAction(pub MenuAction);

pub struct MenuPlugin;

pub struct MenuScreenPlugin {
    pub state: AppState,
    pub menu_def: MenuDef,
}

impl Plugin for MenuScreenPlugin {
    fn build(&self, app: &mut App) {
        let (state, def) = (self.state, self.menu_def.clone());
        app.add_systems(
            OnEnter(self.state),
            move |mut commands: Commands, settings: Res<GameSettings>| {
                commands.spawn_scene(menu_scene(state, &def, &settings));
            },
        );
    }

    fn is_unique(&self) -> bool {
        false
    }
}

pub struct EscToPausePlugin;

impl Plugin for EscToPausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            pause_game.run_if(in_state(AppState::InGame)).run_if(
                input_just_pressed(KeyCode::Escape).or_else(input_just_pressed(KeyCode::KeyQ)),
            ),
        );
    }
}

fn menu_item_label(item: &MenuItem, settings: &GameSettings) -> String {
    if matches!(item.action, MenuAction::CycleComplexity) {
        complexity_label(settings)
    } else {
        item.label.clone()
    }
}

fn complexity_label(settings: &GameSettings) -> String {
    format!("Size: {}", settings.complexity.label())
}

fn menu_scene(state: AppState, def: &MenuDef, settings: &GameSettings) -> impl Scene {
    let screen = MenuScreen {
        on_escape: def.on_escape.clone(),
    };
    let despawn = DespawnOnExit(state);
    let items = def
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            menu_item_scene(
                MenuEntry {
                    index,
                    action: item.action.clone(),
                },
                menu_item_label(item, settings),
            )
        })
        .collect::<Vec<_>>();
    bsn! {
        #Menu
        screen
        despawn
        on(focus_first_item)
        GlobalZIndex(MENU_ZINDEX)
        Node { position_type: PositionType::Absolute, width: percent(100), height: percent(100),
               align_items: AlignItems::Center, justify_content: JustifyContent::Center }
        BackgroundColor(BACKDROP_COLOR)
        Children [
            Node { flex_direction: FlexDirection::Column, row_gap: rem(0.8) }
            Children [ {items} ]
        ]
    }
}

fn menu_item_scene(entry: MenuEntry, label: String) -> impl Scene {
    bsn! {
        Button
        TabIndex(0)
        entry
        Text({label})
        TextFont { font: FontSourceTemplate::Handle(MENU_FONT_PATH), font_size: FontSize::Rem(MENU_FONT_REM) }
        TextColor(NORMAL_COLOR)
        on(focus_hovered_item)
        on(activate_item)
    }
}

fn focus_first_item(
    _: On<Ready>,
    entries: Query<(Entity, &MenuEntry)>,
    mut focus: ResMut<InputFocus>,
) {
    if let Some((entity, _)) = entries.iter().min_by_key(|(_, entry)| entry.index) {
        focus.set(entity, FocusCause::Navigated);
    }
}

fn focus_hovered_item(over: On<PointerOver>, mut focus: ResMut<InputFocus>) {
    if focus.get() != Some(over.entity) {
        focus.set(over.entity, FocusCause::Navigated);
    }
}

fn activate_item(
    activate: On<Activate>,
    entries: Query<&MenuEntry>,
    mut commands: Commands,
) -> Result {
    let entry = entries.get(activate.entity)?;
    commands.trigger(RunMenuAction(entry.action.clone()));
    Ok(())
}

fn navigate_menu(
    keys: Res<ButtonInput<KeyCode>>,
    entries: Query<(Entity, &MenuEntry)>,
    mut focus: ResMut<InputFocus>,
) {
    let up = keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW);
    let down = keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS);
    if !up && !down {
        return;
    }

    let mut ordered: Vec<_> = entries.iter().collect();
    ordered.sort_unstable_by_key(|(_, entry)| entry.index);
    let count = ordered.len();
    if count == 0 {
        return;
    }

    let current = focus
        .get()
        .and_then(|focused| ordered.iter().position(|(entity, _)| *entity == focused));
    let next = if down {
        current.map_or(0, |index| (index + 1) % count)
    } else {
        current.map_or(count - 1, |index| (index + count - 1) % count)
    };
    focus.set(ordered[next].0, FocusCause::Navigated);
}

fn highlight_focused_item(
    focus: Res<InputFocus>,
    mut items: Query<(Entity, &mut TextColor), With<MenuEntry>>,
) {
    for (entity, mut color) in &mut items {
        color.set_if_neq(TextColor(if focus.get() == Some(entity) {
            SELECTED_COLOR
        } else {
            NORMAL_COLOR
        }));
    }
}

fn menu_escape(keys: Res<ButtonInput<KeyCode>>, menu: Single<&MenuScreen>, mut commands: Commands) {
    if (keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyQ))
        && let Some(action) = &menu.on_escape
    {
        commands.trigger(RunMenuAction(action.clone()));
    }
}

fn update_complexity_labels(
    settings: Res<GameSettings>,
    mut items: Query<(&MenuEntry, &mut Text)>,
) {
    for (entry, mut text) in &mut items {
        if matches!(entry.action, MenuAction::CycleComplexity) {
            **text = complexity_label(&settings);
        }
    }
}

fn run_menu_action(
    action: On<RunMenuAction>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut settings: ResMut<GameSettings>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: MessageWriter<AppExit>,
) {
    match &action.0 {
        MenuAction::ChangeState(state) => next_state.set(*state),
        MenuAction::RunSystem {
            system,
            next_state: state,
        } => {
            commands.run_system(*system);
            if let Some(state) = state {
                next_state.set(*state);
            }
        }
        MenuAction::CycleComplexity => settings.complexity = settings.complexity.next(),
        #[cfg(not(target_arch = "wasm32"))]
        MenuAction::Exit => {
            exit.write(AppExit::Success);
        }
    }
}

fn pause_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Paused);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::scene::ScenePlugin;
    use bevy::state::app::StatesPlugin;
    use bevy::text::TextPlugin;

    #[test]
    fn menu_screen_is_despawned_when_its_state_exits() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            AssetPlugin::default(),
            ScenePlugin,
            TextPlugin,
        ))
        .insert_resource(GameSettings::default())
        .init_resource::<InputFocus>()
        .add_plugins(MenuScreenPlugin {
            state: AppState::Menu,
            menu_def: MenuDef {
                items: vec![MenuItem {
                    label: "Start".to_string(),
                    action: MenuAction::ChangeState(AppState::InGame),
                }],
                on_escape: None,
            },
        })
        .init_state::<AppState>();

        app.update();
        let mut screens = app.world_mut().query_filtered::<Entity, With<MenuScreen>>();
        assert_eq!(screens.iter(app.world()).count(), 1);
        let first_entry = {
            let world = app.world_mut();
            let mut entries = world.query::<(Entity, &MenuEntry)>();
            entries
                .iter(world)
                .min_by_key(|(_, entry)| entry.index)
                .map(|(entity, _)| entity)
        };
        assert_eq!(app.world().resource::<InputFocus>().get(), first_entry);

        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();

        assert_eq!(screens.iter(app.world()).count(), 0);
    }

    #[test]
    fn arrow_up_from_first_item_wraps_to_last_and_highlights_it() {
        let menu_def = MenuDef {
            items: ["One", "Two", "Three"]
                .into_iter()
                .map(|label| MenuItem {
                    label: label.to_string(),
                    action: MenuAction::ChangeState(AppState::InGame),
                })
                .collect(),
            on_escape: None,
        };
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            AssetPlugin::default(),
            ScenePlugin,
            TextPlugin,
            MenuPlugin,
        ))
        .insert_resource(GameSettings::default())
        .init_resource::<InputFocus>()
        .init_resource::<ButtonInput<KeyCode>>()
        .init_state::<AppState>()
        .add_plugins(MenuScreenPlugin {
            state: AppState::Menu,
            menu_def,
        });

        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowUp);
        app.update();

        let (last, first) = {
            let world = app.world_mut();
            let mut entries = world.query::<(Entity, &MenuEntry, &TextColor)>();
            let entries: Vec<_> = entries.iter(world).collect();
            let last = entries
                .iter()
                .find(|(_, entry, _)| entry.index == 2)
                .map(|(entity, _, color)| (*entity, color.0));
            let first = entries
                .iter()
                .find(|(_, entry, _)| entry.index == 0)
                .map(|(_, _, color)| color.0);
            (last, first)
        };
        let (last_entity, last_color) = last.expect("last menu entry should exist");
        assert_eq!(
            app.world().resource::<InputFocus>().get(),
            Some(last_entity)
        );
        assert_eq!(last_color, SELECTED_COLOR);
        assert_eq!(first, Some(NORMAL_COLOR));
    }
}
