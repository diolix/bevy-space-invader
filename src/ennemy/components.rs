use bevy::prelude::*;

#[derive(Component)]
pub struct Ennemy {
    pub column_index: u32,
    pub row_index: u32,
}

#[derive(Component)]
pub struct EnnemiesManager {
    pub ennemies_speed: f32,
    pub direction: EnnemiesDirection,
    pub previous_y_position: f32,
    pub count_down_movements: u32,
    pub shoot_timer: Timer,
}

pub enum EnnemiesDirection {
    Left,
    Right,
    Down,
}
