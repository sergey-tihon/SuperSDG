use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub const FPS_OVERLAY_ZINDEX: i32 = i32::MAX - 32;

/// A plugin that adds an FPS overlay to the Bevy application.
///
/// This plugin will add the [`FrameTimeDiagnosticsPlugin`] if it wasn't added before.
///
/// Note: It is recommended to use native overlay of rendering statistics when possible for lower overhead and more accurate results.
/// The correct way to do this will vary by platform:
/// - **Metal**: setting env variable `MTL_HUD_ENABLED=1`
#[derive(Default)]
pub struct FpsOverlayPlugin {
    /// Starting configuration of overlay, this can be later be changed through [`FpsOverlayConfig`] resource.
    pub config: FpsOverlayConfig,
}

impl Plugin for FpsOverlayPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Use plugin dependencies, see https://github.com/bevyengine/bevy/issues/69
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        }
        app.insert_resource(self.config.clone())
            .add_systems(Startup, setup.after(super::CameraSwawned))
            .add_systems(
                Update,
                (
                    (customize_text, toggle_display).run_if(resource_changed::<FpsOverlayConfig>),
                    update_text,
                ),
            );
    }
}

/// Configuration options for the FPS overlay.
#[derive(Resource, Clone)]
pub struct FpsOverlayConfig {
    /// Configuration of text in the overlay.
    pub text_config: TextFont,
    /// Color of text in the overlay.
    pub text_color: Color,
    /// Displays the FPS overlay if true.
    pub enabled: bool,
}

impl Default for FpsOverlayConfig {
    fn default() -> Self {
        FpsOverlayConfig {
            text_config: TextFont {
                font: Handle::<Font>::default(),
                font_size: 32.0,
                ..default()
            },
            text_color: Color::WHITE,
            enabled: true,
        }
    }
}

#[derive(Component)]
struct FpsText;

fn setup(
    mut commands: Commands,
    overlay_config: Res<FpsOverlayConfig>,
    camera: Query<Entity, With<super::MainCamera>>,
) {
    if let Ok(camera) = camera.single() {
        commands
            .spawn((
                UiTargetCamera(camera),
                Node {
                    // We need to make sure the overlay doesn't affect the position of other UI nodes
                    position_type: PositionType::Absolute,
                    ..default()
                },
                // Render overlay on top of everything
                GlobalZIndex(FPS_OVERLAY_ZINDEX),
            ))
            .with_children(|p| {
                p.spawn((
                    Text::new("FPS: "),
                    overlay_config.text_config.clone(),
                    TextColor(overlay_config.text_color),
                    FpsText,
                ))
                .with_child((TextSpan::default(), overlay_config.text_config.clone()));
            });
    }
}

fn update_text(
    diagnostic: Res<DiagnosticsStore>,
    query: Query<Entity, With<FpsText>>,
    mut writer: TextUiWriter,
) {
    for entity in &query {
        if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                *writer.text(entity, 1) = format!("{value:.2}");
            }
        }
    }
}

fn customize_text(
    overlay_config: Res<FpsOverlayConfig>,
    query: Query<Entity, With<FpsText>>,
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
    overlay_config: Res<FpsOverlayConfig>,
    mut query: Query<&mut Visibility, With<FpsText>>,
) {
    for mut visibility in &mut query {
        visibility.set_if_neq(match overlay_config.enabled {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        });
    }
}
