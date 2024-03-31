use std::time::Duration;

use bevy::prelude::*;

use super::{components::*, resources::HpSprites};
use crate::projectile::{
    components::{Collider, Projectile, ProjectileType},
    events::ProjectileHitPlayer,
};

const X_LIMIT: f32 = 900.0;
const Y_POSITION: f32 = -450.0;
const TIME_BETWEEN_SHOOT: f32 = 0.4;
const PLAYER_SPEED: f32 = 400.0;
const Z_VALUE_PROJECTILE: f32 = -1.0;
const HP_SPRITE_INITIAL_POSITION: Vec3 = Vec3::new(900.0, 500.0, 1.0);
const HP_SPRITE_SPACE_BETWEEN: f32 = 50.0;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut hp_sprites_resource: ResMut<HpSprites>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("PNG/playerShip3_blue.png"),
            transform: Transform::from_xyz(0.0, Y_POSITION, 0.0),
            ..default()
        },
        Player {
            speed: PLAYER_SPEED,
            shoot_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Collider {
            widht: 100.0,
            height: 100.0,
        },
    ));

    let mut vec_hp_sprites: Vec<Entity> = Vec::new();
    for i in 0..3 {
        let mut pos = HP_SPRITE_INITIAL_POSITION;
        pos.x -= i as f32 * HP_SPRITE_SPACE_BETWEEN;
        vec_hp_sprites.push(
            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load("PNG/UI/playerLife3_blue.png"),
                    transform: Transform::from_translation(pos),
                    ..default()
                })
                .id(),
        );
    }
    hp_sprites_resource.sprites_entity = vec_hp_sprites;
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query_player: Query<(&mut Transform, &Player)>,
) {
    let (mut transform, player) = query_player.single_mut();

    //check iput and that the player is in the border x
    if (keyboard_input.pressed(KeyCode::A) && !keyboard_input.pressed(KeyCode::D))
        && transform.translation.x > -X_LIMIT
    {
        transform.translation.x -= player.speed * time.delta_seconds();
    } else if (keyboard_input.pressed(KeyCode::D) && !keyboard_input.pressed(KeyCode::A))
        && transform.translation.x < X_LIMIT
    {
        transform.translation.x += player.speed * time.delta_seconds();
    }
}

pub fn shoot_projectile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    key_board_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Player)>,
) {
    let (player_transform, mut player) = match player_query.get_single_mut() {
        Ok(ok) => ok,
        Err(error) => {
            print!(
                "shoot_projectile, error while trying to get_single player components : {:?}",
                error
            );
            return;
        }
    };

    player.shoot_timer.tick(time.delta());
    if !player.shoot_timer.finished() || !key_board_input.just_pressed(KeyCode::Space) {
        return;
    }

    player
        .shoot_timer
        .set_duration(Duration::from_secs_f32(TIME_BETWEEN_SHOOT));
    player.shoot_timer.reset();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("PNG/Lasers/laserBlue03.png"),
            transform: Transform::from_xyz(
                player_transform.translation.x,
                player_transform.translation.y,
                Z_VALUE_PROJECTILE,
            ),
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

pub fn on_hit(
    mut commands: Commands,
    event_reader: EventReader<ProjectileHitPlayer>,
    mut hp_res: ResMut<HpSprites>,
) {
    if event_reader.is_empty() || hp_res.sprites_entity.is_empty() {
        return;
    }

    println!("player hit");
    let sprite = hp_res.sprites_entity.remove(0);
    commands.entity(sprite).despawn();
}
