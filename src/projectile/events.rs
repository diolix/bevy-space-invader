use bevy::prelude::*;

#[derive(Event)]
pub struct ProjectileHitPlayer;

#[derive(Event)]
pub struct ProjectileHitEnnemy {
    pub ennemy: Entity,
}
