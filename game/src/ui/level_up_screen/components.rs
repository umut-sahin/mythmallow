use crate::prelude::*;


/// Tag component for the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreen;


/// Tag component for the perks container in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenPerksContainer;


/// Tag component for perk containers in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenPerkContainer;


/// Tag component for perk details in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenPerkDetails;


/// Tag component for perk name texts in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenPerkNameText;


/// Tag component for perk description texts in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenPerkDescriptionText;


/// Tag component for select buttons in the level up screen.
#[derive(Component, Debug)]
pub struct LevelUpScreenSelectButton {
    pub perk: Arc<dyn IPerk>,
}


/// Tag component for the footer container in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenFooterContainer;


/// Tag component for the balance container in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenBalanceContainer;


/// Tag component for the balance text in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenBalanceText;


/// Tag component for the reroll button in the level up screen.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct LevelUpScreenRerollButton {
    pub cost: Balance,
}
