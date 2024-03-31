use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub projectile_type: ProjectileType,
}

#[derive(Component)]
pub struct Collider {
    pub widht: f32,
    pub height: f32,
}

#[derive(PartialEq)]
pub enum ProjectileType {
    Ennemy,
    Player,
}
