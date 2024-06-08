use crate::prelude::*;


/// Tag component for the settings menu.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct SettingsMenu;


/// Tag component for the language setting in the settings menu.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct SettingsMenuLanguageSettingContainer;


/// Tag component for the previous button of the language setting in the settings menu.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct SettingsMenuLanguageSettingPreviousButton;


/// Tag component for the name of the language setting in the settings menu.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct SettingsMenuLanguageSettingName;


/// Tag component for the value of the language setting in the settings menu.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct SettingsMenuLanguageSettingValue;


/// Tag component for the next button of the language setting in the settings menu.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct SettingsMenuLanguageSettingNextButton;


/// Tag component for the back button in the settings menu.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct SettingsMenuBackButton;
