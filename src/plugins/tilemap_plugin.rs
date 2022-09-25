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
    default_tilemap()
        .into_iter()
        .enumerate()
        .for_each(|line| {
            let current_line = line.0;
            line.1
                .chars()
                .enumerate()
                .for_each(|tile| {
                    let current_column = tile.0;
                    let coord = (current_line as f32, current_column as f32);
                    create_tile_from_char(tile.1, &mut commands, coord, &game_textures);
                })

        });
}

fn default_tilemap() -> Vec<&'static str> {
    vec![
        "####################",
        "#                  #",
        "#     ###          #",
        "#       ######     #",
        "#            #     #",
        "#            #     #",
        "#                  #",
        "####################",
    ]
}

fn create_tile_from_char(
    tile: char,
    mut commands: &mut Commands,
    coord: (f32, f32),
    game_textures: &Res<GameTextures>
) {

    let current_column = coord.1;
    let current_line = coord.0;

    let tile_size = Vec2::new(32., 32.);
    let x = current_column * tile_size.x;
    let y = current_line * tile_size.y;

    match tile {
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
}
