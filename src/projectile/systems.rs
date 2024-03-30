use super::components::*;
use bevy::prelude::*;

const Y_LIMIT: f32 = 600.0;
const PLAYER_PROJECTILE_SPEED: f32 = 600.0;

pub fn shoot_player_projectile_helper(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("PNG/Lasers/laserBlue03.png"),
            transform: Transform::from_translation(position),
            ..default()
        },
        Projectile {
            speed: PLAYER_PROJECTILE_SPEED,
            direction: ProjectileDirection::Up,
        },
    ));
}

pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query_projectile: Query<(&mut Transform, &Projectile, Entity)>,
) {
    for (mut transform, projectile, entity) in query_projectile.iter_mut() {
        if transform.translation.y < -Y_LIMIT || transform.translation.y > Y_LIMIT {
            commands.entity(entity).despawn();
            println!("despawned projectile");
            continue;
        }
        match projectile.direction {
            ProjectileDirection::Up => {
                transform.translation.y += projectile.speed * time.delta_seconds()
            }
            ProjectileDirection::_Down => {
                transform.translation.y -= projectile.speed * time.delta_seconds()
            }
        }
    }
}
