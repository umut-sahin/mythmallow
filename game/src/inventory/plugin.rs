use crate::{
    inventory::{
        commands::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing the inventory of the player.
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // Insert resources.
        app.init_resource::<Inventory>();

        // Add console commands.
        app.add_console_command::<InventoryCommand, _>(apply_inventory_command);

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            load_inventory_when_starting_in_game
                .in_set(InitializationSystems::Inventory)
                .run_if(run_once()),
        );
        app.add_systems(
            PostUpdate,
            (
                acquire_release_items.run_if(|inventory: Res<Inventory>| inventory.is_changed()),
                reposition_weapons.run_if(
                    |weapon_query: Query<Entity, Added<Weapon>>,
                     player_query: Query<&Collider, (With<Player>, Changed<Collider>)>| {
                        !weapon_query.is_empty() || !player_query.is_empty()
                    },
                ),
            ),
        );
        app.add_systems(OnEnter(GameState::Over), clear_inventory);
        app.add_systems(
            OnEnter(GameState::Restart),
            clear_inventory.in_set(RestartSystems::Inventory),
        );
        app.add_systems(OnExit(AppState::Game), clear_inventory);
    }
}
