use crate::{
    prelude::*,
    ui::hud::{
        constants::*,
        localization,
        styles,
    },
};


/// Spawns the HUD.
pub fn spawn_hud(
    mut commands: Commands,
    player_query: Query<(&Health, &RemainingHealth, &Level), With<Player>>,
    asset_server: Res<AssetServer>,
    mut health_bar_materials: ResMut<Assets<HealthBarMaterial>>,
    mut experience_bar_materials: ResMut<Assets<ExperienceBarMaterial>>,
    balance: Res<Balance>,
    localization: Res<Localization>,
) {
    let (health_bar_text, experience_bar_text) = match player_query.get_single() {
        Ok((health, remaining_health, level)) => {
            (
                format!("{} / {}", remaining_health.ceil(), health.ceil()),
                localization::experience_bar(level),
            )
        },
        Err(_) => ("? / ?".to_owned(), "?".into()),
    };
    let balance_text = format!("{}", *balance);

    commands
        .spawn((Name::new("HUD"), Hud, NodeBundle { style: styles::hud(), ..default() }))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Health Bar"),
                    HudHealthBar,
                    MaterialNodeBundle {
                        style: styles::health_bar(),
                        material: health_bar_materials.add(HealthBarMaterial::default()),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Text"),
                        HudHealthBarText,
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    health_bar_text,
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: HEALTH_BAR_TEXT_FONT_SIZE,
                                        color: HEALTH_BAR_TEXT_COLOR,
                                    },
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    Name::new("Experience Bar"),
                    HudExperienceBar,
                    MaterialNodeBundle {
                        style: styles::experience_bar(),
                        material: experience_bar_materials.add(ExperienceBarMaterial::default()),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Text"),
                        HudExperienceBarText,
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    experience_bar_text.get(&localization),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: EXPERIENCE_BAR_TEXT_FONT_SIZE,
                                        color: EXPERIENCE_BAR_TEXT_COLOR,
                                    },
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        },
                        experience_bar_text,
                    ));
                });

            parent
                .spawn((
                    Name::new("Balance"),
                    HudBalanceContainer,
                    NodeBundle { style: styles::balance_container(), ..default() },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Text"),
                        HudBalanceText,
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    balance_text,
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: BALANCE_TEXT_FONT_SIZE,
                                        color: BALANCE_TEXT_COLOR,
                                    },
                                )],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

/// Show the HUD in game.
pub fn show_hud(mut hud_query: Query<&mut Visibility, With<Hud>>) {
    let mut hud_visibility = match hud_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    *hud_visibility = Visibility::Visible;
}

/// Hides the HUD outside the game.
pub fn hide_hud(mut hud_query: Query<&mut Visibility, With<Hud>>) {
    let mut hud_visibility = match hud_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    *hud_visibility = Visibility::Hidden;
}

/// Despawns the HUD.
pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<Hud>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}


/// Updates the health bar.
pub fn update_health_bar(
    player_query: Query<
        (&Health, &RemainingHealth),
        (With<Player>, Or<(Changed<Health>, Changed<RemainingHealth>)>),
    >,
    health_bar_query: Query<&Handle<HealthBarMaterial>, With<HudHealthBar>>,
    mut health_bar_text_query: Query<&mut Text, With<HudHealthBarText>>,
    mut health_bar_materials: ResMut<Assets<HealthBarMaterial>>,
) {
    let (player_health, player_remaining_health) = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let health_bar_handle = match health_bar_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let health_bar = match health_bar_materials.get_mut(health_bar_handle) {
        Some(asset) => asset,
        None => return,
    };

    let mut health_bar_text = match health_bar_text_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    health_bar.percent = (player_remaining_health.0 / player_health.0).clamp(0.00, 1.00);
    health_bar_text.sections[0].value =
        format!("{} / {}", player_remaining_health.ceil(), player_health.ceil());
}

/// Updates the experience bar.
pub fn update_experience_bar(
    player_query: Query<
        (&Experience, &Level),
        (With<Player>, Or<(Changed<Experience>, Changed<Level>)>),
    >,
    experience_bar_query: Query<&Handle<ExperienceBarMaterial>, With<HudExperienceBar>>,
    mut experience_bar_text_query: Query<&mut LocalizedText, With<HudExperienceBarText>>,
    mut experience_bar_materials: ResMut<Assets<ExperienceBarMaterial>>,
    experience_required_to_get_to_current_level: Res<ExperienceRequiredToGetToCurrentLevel>,
    experience_required_to_level_up: Res<ExperienceRequiredToLevelUp>,
) {
    let (player_experience, player_level) = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let experience_bar_handle = match experience_bar_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let experience_bar = match experience_bar_materials.get_mut(experience_bar_handle) {
        Some(asset) => asset,
        None => return,
    };

    let mut experience_bar_text = match experience_bar_text_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let experience_collected =
        player_experience.0 - experience_required_to_get_to_current_level.0.0;

    let experience_required_for_level_up =
        experience_required_to_level_up.0.0 - experience_required_to_get_to_current_level.0.0;

    experience_bar.percent =
        (experience_collected / experience_required_for_level_up).clamp(0.00, 1.00) as f32;
    *experience_bar_text = localization::experience_bar(player_level);
}

/// Updates the balance.
pub fn update_balance(
    mut balance_text_query: Query<&mut Text, With<HudBalanceText>>,
    balance: Res<Balance>,
) {
    if let Ok(mut balance_text) = balance_text_query.get_single_mut() {
        balance_text.sections[0].value = format!("{}", *balance);
    }
}
