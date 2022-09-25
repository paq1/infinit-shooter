use bevy::prelude::*;
use crate::component::player::Player;
use crate::component::velicity::Velocity;

use crate::states::AppState;
use crate::resources::{GameTextures};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(create_player)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(camera_follow.after("movable").after("movement"))
                    .with_system(player_movement.label("movement"))
            );
    }
}

fn create_player(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 2.),
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity {x: 0., y: 0.});
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.x = -1.;
        } else if keyboard_input.pressed(KeyCode::Right) {
            velocity.x = 1.;
        } else {
            velocity.x = 0.;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            velocity.y = -1.;
        } else if keyboard_input.pressed(KeyCode::Up) {
            velocity.y = 1.;
        } else {
            velocity.y = 0.;
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
