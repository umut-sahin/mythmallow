use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    text::DEFAULT_FONT_HANDLE,
};

#[derive(Component)]
pub struct FpsText;

//creates text
pub fn text_setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font: DEFAULT_FONT_HANDLE.typed(),
                font_size: 20.0,
                color: Color::TOMATO,
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(40.0),
                left: Val::Px(20.0),
                ..default()
            }),
        FpsText,
    ));
}

//updates fps and frame time
pub fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                if let Some(frame_time)= diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME){
                    if let Some(value2) = frame_time.smoothed(){
                        // Update the value of the second section
                        text.sections[0].value = format!("FPS: {value:.2}\nFrame Time: {value2:.3}");
                    }
                }

            }
        }
    }
}

pub fn remove_fps_text(mut commands: Commands, query: Query<Entity, With<FpsText>>){
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}



