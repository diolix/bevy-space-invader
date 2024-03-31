use bevy::prelude::*;
pub mod components;
mod resources;
mod systems;
use systems::*;

use self::resources::HpSprites;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HpSprites::default())
            .add_systems(Startup, setup_player)
            .add_systems(Update, (move_player, shoot_projectile, on_hit));
    }
}
