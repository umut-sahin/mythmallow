use crate::prelude::*;


/// Toggles the window mode between fullscreen and windowed.
#[cfg(feature = "native")]
pub fn toggle_fullscreen(
    global_action_state: Res<ActionState<GlobalAction>>,
    mut window_state_query: Query<&mut Persistent<WindowState>, With<PrimaryWindow>>,
) {
    if global_action_state.just_pressed(GlobalAction::ToggleFullscreen) {
        window_state_query
            .single_mut()
            .update(|window_state| {
                window_state.mode = match window_state.mode {
                    WindowMode::Windowed => WindowMode::BorderlessFullscreen,
                    _ => WindowMode::Windowed,
                };
            })
            .ok();
    }
}

/// Toggles the browser mode between fullscreen and windowed.
#[cfg(feature = "wasm")]
pub fn toggle_fullscreen(global_action_state: Res<ActionState<GlobalAction>>) {
    if global_action_state.just_pressed(GlobalAction::ToggleFullscreen) {
        let maybe_error = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
            .and_then(|element| element.request_fullscreen().err());

        if let Some(error) = maybe_error {
            error!("unable to toggle fullscreen ({:?})", error);
        }
    }
}


/// Pauses the game when focus to the application is lost.
pub fn pause_on_losing_focus(
    mut window_focused_reader: EventReader<WindowFocused>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for event in window_focused_reader.iter() {
        if !event.focused {
            next_game_state.set(GameState::Paused);
            break;
        }
    }
}
