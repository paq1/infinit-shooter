use bevy::prelude::*;

use crate::states::AppState;
use crate::resources::{GameTextures};

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(affichage_tile_map)
            );
    }
}

fn affichage_tile_map(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {

    let tiles = vec![
        "####################",
        "#                  #",
        "#     ###          #",
        "#       ######     #",
        "#            #     #",
        "#            #     #",
        "#                  #",
        "####################",
    ];

    let tile_size = Vec2::new(32., 32.);

    tiles
        .into_iter()
        .enumerate()
        .for_each(|line| {
            let current_line = line.0;

            line.1
                .chars()
                .enumerate()
                .for_each(|tile| {
                    let current_column = tile.0;

                    let x = current_column as f32 * tile_size.x;
                    let y = current_line as f32 * tile_size.y;

                    match tile.1 {
                        '#' => {
                            commands
                                .spawn_bundle(SpriteBundle {
                                    texture: game_textures.wall.clone(),
                                    transform: Transform {
                                        translation: Vec3::new(x, y, 1.),
                                        scale: Vec3::new(1., 1., 1.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                        },
                        _ => {}
                    }
                })

        });

    println!("affichage_tile_map");
}