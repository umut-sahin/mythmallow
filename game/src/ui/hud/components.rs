use crate::prelude::*;


/// Tag component for the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Hud;


/// Tag component for the health bar in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HudHealthBar;


/// Tag component for the health bar text in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HudHealthBarText;


/// Tag component for the experience bar in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HudExperienceBar;


/// Tag component for the experience bar text in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HudExperienceBarText;


/// Tag component for the balance container in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HudBalanceContainer;


/// Tag component for the balance text in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HudBalanceText;
