use std::time::Duration;

use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::projectile::{
    components::{Collider, Projectile, ProjectileType},
    events::ProjectileHitEnnemy,
};

use super::components::{EnnemiesDirection, EnnemiesManager, Ennemy};

const Y_POSITION: f32 = 440.0;
const COLUMNS: u32 = 11;
const ROWS: u32 = 4;
const SPACE_BETWEEN_COLUMNS: u32 = 130;
const SPACE_BETWEEN_ROWS: u32 = 100;

const ENNEMIES_SPEED: f32 = 40.0;
const BASE_SHOOT_CADENCE: f32 = 2.0;
const ACCELERATION_SHOOT_CADENCE: f32 = 0.2;

//width resolution / 2
const X_LIMIT: f32 = 960.0;
const DOWN_MOVEMENT_LENGTH: f32 = 50.0;
const MAX_DOWN_MOVEMENT: u32 = 7;

pub fn spawn_ennemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ennemies_manager = commands
        .spawn((
            EnnemiesManager {
                ennemies_speed: ENNEMIES_SPEED,
                direction: EnnemiesDirection::Right,
                previous_y_position: 0.0,
                count_down_movements: 0,
                shoot_timer: Timer::from_seconds(BASE_SHOOT_CADENCE, TimerMode::Once),
            },
            SpatialBundle {
                visibility: Visibility::Visible,
                ..default()
            },
        ))
        .id();

    let half_ennemies_width = (SPACE_BETWEEN_COLUMNS * COLUMNS) as f32 / 2.0;

    for row_index in 0..(ROWS) {
        for column_index in 0..(COLUMNS) {
            commands
                .spawn((
                    SpriteBundle {
                        texture: asset_server.load("PNG/Enemies/enemyBlack1.png"),
                        transform: Transform::from_xyz(
                            -half_ennemies_width
                                + (column_index * SPACE_BETWEEN_COLUMNS) as f32
                                + SPACE_BETWEEN_COLUMNS as f32 / 2.0,
                            Y_POSITION - (row_index * SPACE_BETWEEN_ROWS) as f32,
                            0.0,
                        ),
                        ..default()
                    },
                    Ennemy {
                        row_index,
                        column_index,
                    },
                    Collider {
                        widht: 100.0,
                        height: 100.0,
                    },
                ))
                .set_parent(ennemies_manager);
        }
    }
}

pub fn move_ennemies(
    time: Res<Time>,
    mut ennemies_manager_query: Query<(&mut Transform, &mut EnnemiesManager)>,
    ennemies_query: Query<&Ennemy>,
) {
    let (mut transform, mut ennemies_manager) = ennemies_manager_query.single_mut();

    let min_column = ennemies_query
        .iter()
        .map(|ennemy| ennemy.column_index)
        .min()
        .unwrap_or_default();

    let max_colum = ennemies_query
        .iter()
        .map(|ennemy| ennemy.column_index)
        .max()
        .unwrap_or(1);

    match ennemies_manager.direction {
        EnnemiesDirection::Left => {
            transform.translation.x -= (ennemies_manager.count_down_movements + 1) as f32
                * ENNEMIES_SPEED
                * time.delta_seconds();
            if check_touch_bound(transform.translation.x, max_colum, min_column) {
                handle_down_transition(&mut ennemies_manager, &transform, EnnemiesDirection::Right);
            }
        }
        EnnemiesDirection::Right => {
            transform.translation.x += (ennemies_manager.count_down_movements + 1) as f32
                * ENNEMIES_SPEED
                * time.delta_seconds();
            if check_touch_bound(transform.translation.x, max_colum, min_column) {
                handle_down_transition(&mut ennemies_manager, &transform, EnnemiesDirection::Left);
            }
        }
        EnnemiesDirection::Down => {
            transform.translation.y -= (ennemies_manager.count_down_movements + 1) as f32
                * ENNEMIES_SPEED
                * time.delta_seconds();
            if ennemies_manager.previous_y_position - transform.translation.y > DOWN_MOVEMENT_LENGTH
            {
                if transform.translation.x < 0.0 {
                    ennemies_manager.direction = EnnemiesDirection::Right;
                } else {
                    ennemies_manager.direction = EnnemiesDirection::Left;
                }
            }
        }
    }
}

fn check_touch_bound(translation_x: f32, max_colum: u32, min_column: u32) -> bool {
    let half_ennemies_width = ((max_colum - min_column + 1) * SPACE_BETWEEN_COLUMNS) as f32 / 2.0;
    let center = translation_x + (min_column as f32 * SPACE_BETWEEN_COLUMNS as f32 / 2.0)
        - ((COLUMNS - max_colum - 1) as f32 * SPACE_BETWEEN_COLUMNS as f32 / 2.0);
    center + half_ennemies_width > X_LIMIT || center - half_ennemies_width < -X_LIMIT
}

fn handle_down_transition(
    ennemies_manager: &mut EnnemiesManager,
    transform: &Transform,
    nex_direction: EnnemiesDirection,
) {
    ennemies_manager.previous_y_position = transform.translation.y;
    if ennemies_manager.count_down_movements >= MAX_DOWN_MOVEMENT {
        ennemies_manager.direction = nex_direction;
        return;
    }
    ennemies_manager.direction = EnnemiesDirection::Down;
    ennemies_manager.count_down_movements += 1;
}

pub fn on_hit(mut event_reader: EventReader<ProjectileHitEnnemy>, mut commands: Commands) {
    for ev in event_reader.read() {
        commands.entity(ev.ennemy).despawn();
    }
}

pub fn shoot_projectile(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut ennemies_manager_query: Query<&mut EnnemiesManager>,
    ennemies_transform_query: Query<&GlobalTransform, With<Ennemy>>,
) {
    let mut ennemies_manager = match ennemies_manager_query.get_single_mut() {
        Ok(ok) => ok,
        Err(error) => {
            print!("ennemies_manager, error getting EnnemiesManager component : {error}");
            return;
        }
    };

    ennemies_manager.shoot_timer.tick(time.delta());

    if !ennemies_manager.shoot_timer.finished() {
        return;
    }

    //source : https://stackoverflow.com/questions/34215280/how-can-i-randomly-select-one-element-from-a-vector-or-array
    let spawn_position = match ennemies_transform_query
        .iter()
        .choose(&mut rand::thread_rng())
    {
        Some(ok) => ok,
        None => {
            return;
        }
    };

    let duration = BASE_SHOOT_CADENCE
        - (ACCELERATION_SHOOT_CADENCE * ennemies_manager.count_down_movements as f32);
    ennemies_manager
        .shoot_timer
        .set_duration(Duration::from_secs_f32(duration));
    ennemies_manager.shoot_timer.reset();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("PNG/Lasers/laserRed03.png"),
            transform: Transform::from_xyz(
                spawn_position.translation().x,
                spawn_position.translation().y,
                -1.0,
            ),
            ..default()
        },
        Projectile {
            projectile_type: ProjectileType::Ennemy,
        },
        Collider {
            widht: 30.0,
            height: 70.0,
        },
    ));
}
