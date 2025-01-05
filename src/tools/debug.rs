use bevy::app::App;
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
    ));
}
