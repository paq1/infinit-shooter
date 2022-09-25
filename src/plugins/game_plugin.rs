use bevy::prelude::*;

use crate::states::AppState;
use crate::resources::{GameTextures};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(load_game)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_game)
            );
    }
}

fn load_game(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.bg.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        });

    println!("load_game");
}

fn update_game() {
    // println!("update_game");
}