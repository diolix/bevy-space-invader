use bevy::prelude::*;

use self::systems::move_projectiles;
mod components;
pub mod systems;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_projectiles);
    }
}
