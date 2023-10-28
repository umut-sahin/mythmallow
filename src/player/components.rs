use crate::prelude::*;


/// Tag component for the player.
#[derive(Clone, Copy, Component, Debug, Reflect)]
pub struct Player;


/// Tag component for the player hit box.
#[derive(Component, Debug, Reflect)]
pub struct PlayerHitbox;
