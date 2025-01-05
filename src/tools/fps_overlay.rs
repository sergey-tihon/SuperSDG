use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: 42.0,
                font: default(),
                font_smoothing: FontSmoothing::default(),
            },
            text_color: Color::srgb(0.0, 1.0, 0.0),
            enabled: true,
        },
    });
}
