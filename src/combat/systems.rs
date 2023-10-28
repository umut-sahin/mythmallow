use crate::prelude::*;


/// Damages the player for every enemy it's touching.
pub fn damage_player_on_contact_with_enemies(
    mut commands: Commands,
    mut player_query: Query<&mut RemainingHealth, With<Player>>,
    mut player_hitbox_query: Query<Entity, With<PlayerHitbox>>,
    enemy_query: Query<&Damage, (Without<Player>, With<Enemy>, Without<Cooldown<Attack>>)>,
    mut collision_event_reader: EventReader<Collision>,
) {
    let mut player_remaining_health = match player_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let player_sensor_entity = match player_hitbox_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    for Collision(contact) in collision_event_reader.iter() {
        if contact.entity1 == player_sensor_entity || contact.entity2 == player_sensor_entity {
            let enemy_entity = if contact.entity1 == player_sensor_entity {
                contact.entity2
            } else {
                contact.entity1
            };

            let enemy_damage = match enemy_query.get(enemy_entity) {
                Ok(query_result) => query_result,
                Err(_) => continue,
            };

            player_remaining_health.0 -= enemy_damage.0;

            commands
                .entity(enemy_entity)
                .insert(Cooldown::<Attack>::new(Timer::from_seconds(1.00, TimerMode::Once)));
            break;
        }
    }
}


/// Checks if the player is dead and despawns the player if it is.
pub fn player_death(
    mut commands: Commands,
    player_query: Query<&RemainingHealth, With<Player>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let remaining_health = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    if remaining_health.0 <= 0.00 {
        commands.insert_resource(GameResult::Lost);
        next_game_state.set(GameState::Over);
    }
}
