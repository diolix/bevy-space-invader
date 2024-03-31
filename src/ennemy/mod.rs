use bevy::prelude::*;
pub mod components;
mod systems;

use systems::*;

pub struct EnnemyPlugin;

impl Plugin for EnnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ennemies)
            .add_systems(Update, (move_ennemies, on_hit));
    }
}
