use bevy::prelude::*;

use super::{OverlayCamera, OverlayCameraSpawned};

pub const HELP_OVERLAY_ZINDEX: i32 = i32::MAX - 31; // Just above FPS overlay

/// A plugin that adds a help overlay to display control hotkeys.
#[derive(Default)]
pub struct HelpOverlayPlugin {
    /// Starting configuration of overlay, this can be later be changed through [`HelpOverlayConfig`] resource.
    pub config: HelpOverlayConfig,
}

impl Plugin for HelpOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .add_systems(Startup, setup.after(OverlayCameraSpawned))
            .add_systems(OnEnter(crate::core::AppState::InGame), show_overlay)
            .add_systems(OnExit(crate::core::AppState::InGame), hide_overlay)
            .add_systems(
                Update,
                (
                    (customize_text, toggle_display).run_if(resource_changed::<HelpOverlayConfig>),
                    toggle_with_f1,
                )
                    .run_if(in_state(crate::core::AppState::InGame)),
            );
    }
}

/// Configuration options for the help overlay.
#[derive(Resource, Clone)]
pub struct HelpOverlayConfig {
    /// Configuration of text in the overlay.
    pub text_config: TextFont,
    /// Color of text in the overlay.
    pub text_color: Color,
    /// Displays the help overlay if true.
    pub enabled: bool,
}

impl Default for HelpOverlayConfig {
    fn default() -> Self {
        HelpOverlayConfig {
            text_config: TextFont {
                font: Handle::<Font>::default(),
                font_size: 24.0,
                ..default()
            },
            text_color: Color::WHITE,
            enabled: true,
        }
    }
}

#[derive(Component)]
struct HelpText;

#[derive(Component)]
struct HelpOverlayRoot;

fn setup(
    mut commands: Commands,
    overlay_config: Res<HelpOverlayConfig>,
    camera: Query<Entity, With<OverlayCamera>>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    commands
        .spawn((
            UiTargetCamera(camera),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            GlobalZIndex(HELP_OVERLAY_ZINDEX),
            Visibility::Hidden, // Start hidden, show on InGame
            HelpOverlayRoot,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Help: F1\n"),
                overlay_config.text_config.clone(),
                TextColor(overlay_config.text_color),
                HelpText,
            ))
            .with_child((
                TextSpan::new("Movement: Arrow Keys\n"),
                overlay_config.text_config.clone(),
            ))
            .with_child((
                TextSpan::new("Camera: Shift + Arrow Keys\n"),
                overlay_config.text_config.clone(),
            ))
            .with_child((
                TextSpan::new("Menu: Escape / Q\n"),
                overlay_config.text_config.clone(),
            ));
        });
}

fn show_overlay(mut query: Query<&mut Visibility, With<HelpOverlayRoot>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Visible;
    }
}

fn hide_overlay(mut query: Query<&mut Visibility, With<HelpOverlayRoot>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
}

fn customize_text(
    overlay_config: Res<HelpOverlayConfig>,
    query: Query<Entity, With<HelpText>>,
    mut writer: TextUiWriter,
) {
    for entity in &query {
        writer.for_each_font(entity, |mut font| {
            *font = overlay_config.text_config.clone();
        });
        writer.for_each_color(entity, |mut color| color.0 = overlay_config.text_color);
    }
}

fn toggle_display(
    overlay_config: Res<HelpOverlayConfig>,
    mut query: Query<&mut Visibility, With<HelpText>>,
) {
    for mut visibility in &mut query {
        visibility.set_if_neq(match overlay_config.enabled {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        });
    }
}

fn toggle_with_f1(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut overlay_config: ResMut<HelpOverlayConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        overlay_config.enabled = !overlay_config.enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_spawn_does_not_fail() {
        // Create a test app
        let mut app = App::new();

        // Add necessary resources and plugins
        app.insert_resource(HelpOverlayConfig::default());

        // Add a mock camera entity with OverlayCamera component
        let camera_entity = app.world_mut().spawn(OverlayCamera).id();

        // Create a system that calls setup and verify it doesn't panic
        let test_system =
            move |commands: Commands,
                  overlay_config: Res<HelpOverlayConfig>,
                  camera: Query<Entity, With<OverlayCamera>>| {
                // This should not panic
                setup(commands, overlay_config, camera);
            };

        // Add and run the test system
        app.add_systems(Update, test_system);
        app.update();

        // If we got here without panicking, the test passed

        // Clean up
        app.world_mut().despawn(camera_entity);
    }
}
