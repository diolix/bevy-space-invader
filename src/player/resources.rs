use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Resource, Default)]
pub struct HpSprites {
    pub sprites_entity: Vec<Entity>,
}
