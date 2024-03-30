use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub direction: ProjectileDirection,
}

pub enum ProjectileDirection {
    Up,
    _Down,
}
