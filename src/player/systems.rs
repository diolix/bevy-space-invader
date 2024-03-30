use std::time::Duration;

use bevy::prelude::*;

use super::components::*;
use crate::projectile::systems::*;

const X_LIMIT: f32 = 850.0;
const Y_POSITION: f32 = -450.0;
const TIME_BETWEEN_SHOOT: f32 = 0.4;
const PLAYER_SPEED: f32 = 400.0;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    ));
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
    shoot_player_projectile_helper(&mut commands, &asset_server, player_transform.translation);
}
