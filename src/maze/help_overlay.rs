use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::core::AppState;

const HELP_OVERLAY_ZINDEX: i32 = i32::MAX - 31; // Just above FPS overlay
const HELP_FONT_SIZE: FontSize = FontSize::Rem(1.2);

/// A plugin that adds a help overlay to display control hotkeys.
pub struct HelpOverlayPlugin;

impl Plugin for HelpOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::InGame), show_overlay)
            .add_systems(OnExit(AppState::InGame), hide_overlay)
            .add_systems(
                Update,
                toggle_help_text
                    .run_if(in_state(AppState::InGame))
                    .run_if(input_just_pressed(KeyCode::F1)),
            );
    }
}

#[derive(Component, Default, Clone)]
struct HelpText;

#[derive(Component, Default, Clone)]
struct HelpOverlayRoot;

fn help_line(text: &'static str) -> impl Scene {
    bsn! { TextSpan(text) TextFont { font_size: HELP_FONT_SIZE } }
}

fn setup(mut commands: Commands) {
    let lines: Vec<_> = [
        "Movement: Arrow Keys\n",
        "Camera: Shift + Arrow Keys\n",
        "Menu: Escape / Q\n",
    ]
    .into_iter()
    .map(help_line)
    .collect();
    commands.spawn_scene(bsn! {
        #HelpOverlay
        HelpOverlayRoot
        Node { display: Display::None, position_type: PositionType::Absolute, bottom: rem(0.5), left: rem(0.5) }
        GlobalZIndex(HELP_OVERLAY_ZINDEX)
        Visibility::Hidden
        Children [
            HelpText Text("Help: F1\n") TextFont { font_size: HELP_FONT_SIZE } Visibility::Visible
            Children [ {lines} ]
        ]
    });
}

fn show_overlay(root: Single<(&mut Visibility, &mut Node), With<HelpOverlayRoot>>) {
    let (mut visibility, mut node) = root.into_inner();
    *visibility = Visibility::Visible;
    node.display = Display::Flex;
}

fn hide_overlay(root: Single<(&mut Visibility, &mut Node), With<HelpOverlayRoot>>) {
    let (mut visibility, mut node) = root.into_inner();
    *visibility = Visibility::Hidden;
    node.display = Display::None;
}

fn toggle_help_text(mut visibility: Single<&mut Visibility, With<HelpText>>) {
    visibility.toggle_visible_hidden();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::scene::ScenePlugin;
    use bevy::state::app::StatesPlugin;
    use bevy::text::TextPlugin;

    #[test]
    fn help_toggle_persists_across_pause_and_resume() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            AssetPlugin::default(),
            ScenePlugin,
            TextPlugin,
            HelpOverlayPlugin,
        ))
        .init_resource::<ButtonInput<KeyCode>>()
        .init_state::<AppState>();

        app.update();
        {
            let world = app.world_mut();
            let mut roots = world.query_filtered::<(&Visibility, &Node), With<HelpOverlayRoot>>();
            let (visibility, node) = roots.single(world).unwrap();
            assert_eq!(*visibility, Visibility::Hidden);
            assert_eq!(node.display, Display::None);
            let mut help_text = world.query_filtered::<&Visibility, With<HelpText>>();
            assert_eq!(*help_text.single(world).unwrap(), Visibility::Visible);
        }

        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::F1);
        app.update();
        {
            let world = app.world_mut();
            let mut help_text = world.query_filtered::<&Visibility, With<HelpText>>();
            assert_eq!(*help_text.single(world).unwrap(), Visibility::Hidden);
        }

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::Paused);
        app.update();
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();
        {
            let world = app.world_mut();
            let mut roots = world.query_filtered::<(&Visibility, &Node), With<HelpOverlayRoot>>();
            let (visibility, node) = roots.single(world).unwrap();
            assert_eq!(*visibility, Visibility::Visible);
            assert_eq!(node.display, Display::Flex);
            let mut help_text = world.query_filtered::<&Visibility, With<HelpText>>();
            assert_eq!(*help_text.single(world).unwrap(), Visibility::Hidden);
        }
    }
}
