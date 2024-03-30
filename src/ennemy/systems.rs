use bevy::prelude::*;

use super::components::{EnnemiesDirection, EnnemiesManager, Ennemy};

const Y_POSITION: f32 = 480.0;
const COLUMNS: u32 = 11;
const ROWS: u32 = 4;
const SPACE_BETWEEN_COLUMNS: u32 = 120;
const SPACE_BETWEEN_ROWS: u32 = 100;

const ENNEMIES_SPEED: f32 = 40.0;

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
            let ennemies_width = (max_colum - min_column + 1) * SPACE_BETWEEN_COLUMNS;
            transform.translation.x -= (ennemies_manager.count_down_movements + 1) as f32
                * ENNEMIES_SPEED
                * time.delta_seconds();
            if transform.translation.x - (ennemies_width as f32) / 2.0 < -X_LIMIT {
                handle_down_transition(&mut ennemies_manager, &transform, EnnemiesDirection::Right);
            }
        }
        EnnemiesDirection::Right => {
            let ennemies_width = (max_colum - min_column + 1) * SPACE_BETWEEN_COLUMNS;
            transform.translation.x += (ennemies_manager.count_down_movements + 1) as f32
                * ENNEMIES_SPEED
                * time.delta_seconds();
            if transform.translation.x + (ennemies_width as f32) / 2.0 > X_LIMIT {
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
