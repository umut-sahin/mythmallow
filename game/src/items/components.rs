use crate::prelude::*;


/// Tag component for items.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Item;


/// Tag component for weapons.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Weapon;
