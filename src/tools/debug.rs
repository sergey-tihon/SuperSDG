use bevy::dev_tools::render_debug::RenderDebugOverlayKeybindings;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        LogDiagnosticsPlugin::default(),
        //FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin::default(),
    ))
    .insert_resource(RenderDebugOverlayKeybindings {
        enable_keybindings: true,
        cycle_mode: KeyCode::F3,
        cycle_opacity: KeyCode::F4,
    })
    .edit_schedule(Update, |schedule| {
        let seed = std::env::var("SUPERSDG_SCHEDULE_SEED")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or_else(rand::random::<u64>);
        info!("Randomizing Update schedule with seed={seed}");
        schedule.set_build_settings(ScheduleBuildSettings {
            shuffle_seed: Some(seed),
            ..default()
        });
    });
}
