use crate::prelude::*;


/// Tag component for the current wave container in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CurrentWaveContainer;


/// Tag component for the current wave text in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CurrentWaveText;


/// Tag component for the remaining seconds container in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct RemainingSecondsContainer;


/// Tag component for the remaining seconds text in the HUD.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct RemainingSecondsText;
