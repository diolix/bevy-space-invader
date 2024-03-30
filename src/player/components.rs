use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub speed: f32,
    pub shoot_timer: Timer,
}
