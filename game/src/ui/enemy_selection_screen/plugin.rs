use crate::{
    prelude::*,
    ui::enemy_selection_screen::systems::*,
};

/// Plugin for managing the enemy selection screen.
pub struct EnemySelectionScreenPlugin;

impl Plugin for EnemySelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<EnemySelectionScreen>();

        // Add systems.
        app.add_systems(OnEnter(AppState::EnemySelectionScreen), spawn_enemy_selection_screen);
        app.add_systems(
            PostUpdate,
            enemy_pack_selected.in_set(EnemySelectionScreenSystems).run_if(
                |enemy_pack_index: Option<Res<SelectedEnemyPackIndex>>| enemy_pack_index.is_some(),
            ),
        );
        app.add_systems(OnExit(AppState::EnemySelectionScreen), despawn_enemy_selection_screen);

        // Select the enemies when starting in game.
        let args = app.world.resource::<Args>();
        if args.start_in_game {
            app.add_systems(
                OnEnter(AppState::EnemySelectionScreen),
                select_enemy_pack_when_starting_in_game.run_if(run_once()),
            );
        }
    }
}
