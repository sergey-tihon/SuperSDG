use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

const FPS_OVERLAY_ZINDEX: i32 = i32::MAX - 32;
const FPS_FONT_SIZE: FontSize = FontSize::Rem(1.6);

/// A plugin that adds an FPS overlay to the Bevy application.
///
/// This plugin will add the [`FrameTimeDiagnosticsPlugin`] if it wasn't added before.
///
/// Note: It is recommended to use native overlay of rendering statistics when possible for lower overhead and more accurate results.
/// The correct way to do this will vary by platform:
/// - **Metal**: setting env variable `MTL_HUD_ENABLED=1`
pub struct FpsOverlayPlugin;

impl Plugin for FpsOverlayPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        }
        app.add_systems(Startup, setup)
            .add_systems(Update, update_text);
    }
}

#[derive(Component, Default, Clone)]
struct FpsValue;

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #FpsOverlay
        Node { position_type: PositionType::Absolute }
        GlobalZIndex(FPS_OVERLAY_ZINDEX)
        Children [
            Text("FPS: ") TextFont { font_size: FPS_FONT_SIZE }
            Children [ FpsValue TextSpan TextFont { font_size: FPS_FONT_SIZE } ]
        ]
    });
}

fn update_text(
    diagnostic: Res<DiagnosticsStore>,
    mut value: Single<&mut TextSpan, With<FpsValue>>,
) {
    if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS)
        && let Some(fps) = fps.average()
    {
        value.0 = format!("{fps:.0}");
    }
}
