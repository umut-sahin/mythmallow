use crate::prelude::*;


/// Spawns the diagnostics overlay.
pub fn spawn_diagnostics_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
) {
    let text_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        DiagnosticsOverlayText,
        TextBundle::from_section(
            "FPS: N/A\nFrame Time: N/A",
            TextStyle { font: text_font, font_size: 20.0, color: Color::DARK_GRAY },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
    ));
    general_settings
        .update(|fps_setting| {
            fps_setting.show_diagnostics_overlay = true;
        })
        .ok();
}


/// Updates diagnostics overlay.
pub fn update_diagnostics_overlay(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<DiagnosticsOverlayText>>,
) {
    let mut text = match query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let mut new_diagnostics_overlay_text = String::new();

    new_diagnostics_overlay_text += "FPS: ";

    let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed());
    match fps {
        Some(fps) => new_diagnostics_overlay_text += &format!("{fps:.2}"),
        None => new_diagnostics_overlay_text += "N/A",
    };

    new_diagnostics_overlay_text += "\nFrame Time: ";

    let frame_time = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|frame_time| frame_time.smoothed());
    match frame_time {
        Some(frame_time) => new_diagnostics_overlay_text += &format!("{frame_time:.3} ms"),
        None => new_diagnostics_overlay_text += "N/A",
    };

    text.sections[0].value = new_diagnostics_overlay_text;
}


/// Removes the diagnostics overlay.
pub fn despawn_diagnostics_overlay(
    mut commands: Commands,
    query: Query<Entity, With<DiagnosticsOverlayText>>,
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
    general_settings
        .update(|fps_setting| {
            fps_setting.show_diagnostics_overlay = false;
        })
        .ok();
}
