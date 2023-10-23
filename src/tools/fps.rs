use bevy::prelude::*;
use bevy_screen_diags::ScreenDiagsText;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_screen_diags::ScreenDiagsTextPlugin)
            .add_systems(PostStartup, tweak_fps);
    }
}

fn tweak_fps(mut text_query: Query<(&mut Text, &mut Style), With<ScreenDiagsText>>) {
    let (mut text, mut style) = text_query.single_mut();

    text.sections[0].style.color = Color::GREEN;
    text.sections[0].style.font_size = 64.0;

    style.position_type = PositionType::Absolute;
    style.right = Val::Percent(0.0);
    style.bottom = Val::Percent(1.0);
}
