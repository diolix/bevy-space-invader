use bevy::prelude::*;

use self::{
    events::{ProjectileHitEnnemy, ProjectileHitPlayer},
    systems::{check_collision, move_projectiles},
};
pub mod components;
pub mod events;
pub mod systems;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileHitEnnemy>()
            .add_event::<ProjectileHitPlayer>()
            .add_systems(Update, (move_projectiles, check_collision));
    }
}
