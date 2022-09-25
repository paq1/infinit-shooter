use bevy::prelude::*;
use rand::Rng;

use crate::states::AppState;
use crate::resources::{AssetsLoading, GameTextures};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(load_assets)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(check_assets_ready)
            );
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut loading_asset: ResMut<AssetsLoading>
) {
    let bg: Handle<Image> = asset_server.load("bg.png");
    loading_asset.0.push(bg.clone_untyped());

    let mur: Handle<Image> = asset_server.load("mur.png");
    loading_asset.0.push(mur.clone_untyped());

    println!("Chargement des assets ...");
}


fn check_assets_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loading_asset: Res<AssetsLoading>,
    mut app_state: ResMut<State<AppState>>,
) {
    use bevy::asset::LoadState;

    match asset_server.get_group_load_state(loading_asset.0.iter().map(|handle| handle.id)) {

        LoadState::Loaded => {

            println!("toutes les ressources sont chargées");

            // on ajoute nos resources
            let game_textures = GameTextures {
                wall: asset_server.get_handle("mur.png"),
            };

            commands.insert_resource(game_textures);

            println!("les ressources sont bien ajoutées");

            // on passe à l'état suivant
            app_state.set(AppState::InGame).unwrap();

            println!("changement d'etat OK");

            // plus besoin de la resource, on la supprime
            commands.remove_resource::<AssetsLoading>();
        },
        LoadState::Failed => {
            panic!("Failed to load assets");
        },
        _ => {

            let emojis = vec!["😀", "😋", "🤑"];
            let mut rng = rand::thread_rng();
            let emoji = emojis[rng.gen_range(0..emojis.len())];
            println!("assets loading {emoji} ...");

        }
    };
}
