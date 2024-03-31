use super::{
    components::*,
    events::{ProjectileHitEnnemy, ProjectileHitPlayer},
};
use crate::{ennemy::components::Ennemy, player::components::Player};
use bevy::prelude::*;

const Y_LIMIT: f32 = 600.0;
const PLAYER_PROJECTILE_SPEED: f32 = 600.0;
const ENNEMY_PROJECTILE_SPEED: f32 = 600.0;
const Z_VALUE: f32 = -1.0;

pub fn shoot_player_projectile_helper(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("PNG/Lasers/laserBlue03.png"),
            transform: Transform::from_xyz(position.x, position.y, Z_VALUE),
            ..default()
        },
        Projectile {
            projectile_type: ProjectileType::Player,
        },
        Collider {
            widht: 30.0,
            height: 70.0,
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
            continue;
        }
        match projectile.projectile_type {
            ProjectileType::Player => {
                transform.translation.y += PLAYER_PROJECTILE_SPEED * time.delta_seconds()
            }
            ProjectileType::Ennemy => {
                transform.translation.y -= ENNEMY_PROJECTILE_SPEED * time.delta_seconds()
            }
        }
    }
}

pub fn check_collision(
    mut commands: Commands,
    projectiles_collision_query: Query<(&GlobalTransform, &Collider, &Projectile, Entity)>,
    player_collision_query: Query<(&GlobalTransform, &Collider, With<Player>)>,
    ennemies_collision_query: Query<(&GlobalTransform, &Collider, Entity, With<Ennemy>)>,
    mut event_writer_player: EventWriter<ProjectileHitPlayer>,
    mut event_writer_ennemy: EventWriter<ProjectileHitEnnemy>,
) {
    let (player_transform, player_collider) = match player_collision_query.get_single() {
        Ok(ok) => (ok.0, ok.1),
        Err(error) => {
            println!("check_collision, error trying acces player components : {error}");
            return;
        }
    };

    let player_projectiles = projectiles_collision_query
        .iter()
        .filter(|projectiles| projectiles.2.projectile_type == ProjectileType::Player);

    let ennemy_projectiles = projectiles_collision_query
        .iter()
        .filter(|projectiles| projectiles.2.projectile_type == ProjectileType::Ennemy);

    let player_rec_collider = rec_collider_coordinate(
        player_transform.translation(),
        player_collider.widht,
        player_collider.height,
    );

    //check if ennemies projectiles has hit the player
    for (transform, collider, _, entity) in ennemy_projectiles {
        let projectile_rec_collider =
            rec_collider_coordinate(transform.translation(), collider.widht, collider.height);

        if !overlap(player_rec_collider, projectile_rec_collider) {
            continue;
        }

        commands.entity(entity).despawn();

        event_writer_player.send(ProjectileHitPlayer);
        break;
    }

    //check if player projectiles has hit an ennemy
    for (projectile_transform, projectile_collider, _, projectile_entity) in player_projectiles {
        let projectile_rec_collider = rec_collider_coordinate(
            projectile_transform.translation(),
            projectile_collider.widht,
            projectile_collider.height,
        );

        for (ennemy_transform, ennemy_collider, ennemy_entity, _) in ennemies_collision_query.iter()
        {
            let ennemy_rec_collider = rec_collider_coordinate(
                ennemy_transform.translation(),
                ennemy_collider.widht,
                ennemy_collider.height,
            );

            if !overlap(projectile_rec_collider, ennemy_rec_collider) {
                continue;
            }

            commands.entity(projectile_entity).despawn();

            event_writer_ennemy.send(ProjectileHitEnnemy {
                ennemy: ennemy_entity,
            })
        }
    }
}

//source : https://www.educative.io/answers/how-to-check-if-two-rectangles-overlap-each-other
fn overlap(rec1: Rect, rec2: Rect) -> bool {
    let width_is_positive = rec1.max.x.min(rec2.max.x) > rec1.min.x.max(rec2.min.x);
    let height_is_positive = rec1.max.y.min(rec2.max.y) > rec1.min.y.max(rec2.min.y);
    width_is_positive && height_is_positive
}

fn rec_collider_coordinate(position: Vec3, width: f32, height: f32) -> Rect {
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    Rect {
        min: Vec2 {
            x: position.x - half_width,
            y: position.y - half_height,
        },
        max: Vec2 {
            x: position.x + half_width,
            y: position.y + half_height,
        },
    }
}
