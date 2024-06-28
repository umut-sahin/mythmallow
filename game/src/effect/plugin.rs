use crate::{
    effect::{
        blood::*,
        pop::*,
        systems::*,
        walking::*,
    },
    prelude::*,
};

/// Plugin for managing the effects.
pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        // Add particle plugin.
        app.add_plugins(HanabiPlugin);

        // Register components.
        app.register_type::<WalkingParticles>();
        app.register_type::<LastWalkingParticlePosition>();
        app.register_type::<PopParticles>();
        app.register_type::<BloodParticles>();

        // Add systems.
        {
            app.add_systems(
                OnEnter(GameState::Initialization),
                (spawn_pop_particles, spawn_blood_particles, spawn_walking_effect)
                    .in_set(InitializationSystems::Effect),
            );


            app.add_systems(OnEnter(GameState::Paused), pause_effects);
            app.add_systems(
                Last,
                pause_effects.run_if(|console_state: Res<ConsoleState>| {
                    console_state.is_changed() && console_state.open
                }),
            );
            app.add_systems(
                OnEnter(GameState::Playing),
                resume_effects.run_if(|console_state: Res<ConsoleState>| !console_state.open),
            );
            app.add_systems(
                Last,
                resume_effects.run_if(
                    |game_state: Res<State<GameState>>, console_state: Res<ConsoleState>| {
                        console_state.is_changed()
                            && !console_state.open
                            && *game_state == GameState::Playing
                    },
                ),
            );


            app.add_systems(
                OnEnter(GameState::Over),
                (despawn_pop_particles, despawn_blood_particles, despawn_walking_particles),
            );
            app.add_systems(
                OnEnter(GameState::Restart),
                (despawn_pop_particles, despawn_blood_particles, despawn_walking_particles)
                    .in_set(RestartSystems::Effect),
            );
            app.add_systems(
                OnExit(AppState::Game),
                (despawn_pop_particles, despawn_blood_particles, despawn_walking_particles),
            );
        }
    }
}
