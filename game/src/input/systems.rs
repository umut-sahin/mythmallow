use crate::prelude::*;


/// Pauses the game when the application loses it's focus.
pub fn pause_on_losing_focus(
    mut window_focused_reader: EventReader<WindowFocused>,
    general_settings: Res<Persistent<GeneralSettings>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for event in window_focused_reader.read() {
        if !event.focused && general_settings.pause_on_losing_focus {
            next_game_state.set(GameState::Paused);
            break;
        }
    }
}


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

/// Toggles the window mode of the browser between fullscreen and windowed.
#[cfg(feature = "wasm")]
pub fn toggle_fullscreen(global_action_state: Res<ActionState<GlobalAction>>) {
    if global_action_state.just_pressed(GlobalAction::ToggleFullscreen) {
        let window = match web_sys::window() {
            Some(window) => window,
            None => {
                log::error!("unable to get the window to toggle fullscreen");
                return;
            },
        };

        let document = match window.document() {
            Some(document) => document,
            None => {
                log::error!("unable to get the document to toggle fullscreen");
                return;
            },
        };

        let element = match document.document_element() {
            Some(element) => element,
            None => {
                log::error!("unable to get the document element to toggle fullscreen");
                return;
            },
        };

        if let Err(error) = element.request_fullscreen() {
            log::error!("unable to toggle fullscreen ({:?})", error);
        }
    }
}


/// Toggles the diagnostics overlay.
pub fn toggle_diagnostics_overlay(
    diagnostics_overlay_state: Res<State<DiagnosticsOverlayState>>,
    global_action_state: Res<ActionState<GlobalAction>>,
    mut next_diagnostics_overlay_state: ResMut<NextState<DiagnosticsOverlayState>>,
) {
    if global_action_state.just_pressed(GlobalAction::ToggleDiagnosticsOverlay) {
        next_diagnostics_overlay_state.set(match diagnostics_overlay_state.get() {
            DiagnosticsOverlayState::Enabled => DiagnosticsOverlayState::Disabled,
            DiagnosticsOverlayState::Disabled => DiagnosticsOverlayState::Enabled,
        });
    }
}


/// Toggles physics debug.
#[cfg(feature = "development")]
pub fn toggle_physics_debug(
    global_action_state: Res<ActionState<GlobalAction>>,
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
    mut physics_debug_config: ResMut<PhysicsDebugConfig>,
) {
    if global_action_state.just_pressed(GlobalAction::TogglePhysicsDebug) {
        general_settings
            .update(|general_settings| {
                general_settings.debug_physics = !general_settings.debug_physics;
            })
            .ok();

        physics_debug_config.enabled = general_settings.debug_physics;
    }
}
