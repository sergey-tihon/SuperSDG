use bevy::{prelude::*, window::PrimaryWindow};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui_camera)
            .add_systems(Update, update_rem_size);
    }
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #UiCamera
        Camera2d
        Camera { order: 1000, clear_color: ClearColorConfig::None }
        IsDefaultUiCamera
    });
}

const WINDOW_HEIGHT_PER_REM: f32 = 50.0;
const MIN_REM_SIZE_PX: f32 = 10.0;

pub fn rem_size_for_height(logical_height: f32) -> f32 {
    (logical_height / WINDOW_HEIGHT_PER_REM).max(MIN_REM_SIZE_PX)
}

fn update_rem_size(window: Single<&Window, With<PrimaryWindow>>, mut rem: ResMut<RemSize>) {
    rem.set_if_neq(RemSize(rem_size_for_height(window.height())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rem_size_matches_default_at_thousand_pixels() {
        assert_eq!(rem_size_for_height(1000.0), 20.0);
    }

    #[test]
    fn rem_size_clamps_to_minimum() {
        assert_eq!(rem_size_for_height(200.0), MIN_REM_SIZE_PX);
    }
}
