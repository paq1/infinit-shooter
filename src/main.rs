mod resources;
mod states;
mod plugins;

use bevy::prelude::*;
use crate::resources::{AssetsLoading, WinSize};
use crate::states::AppState;
use crate::plugins::{
    loading_asset_plugin::LoadingPlugin,
    game_plugin::InGamePlugin,
    debug_plugin::DebugPlugin
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            title: "infinit-shooter".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_state(AppState::Loading)
        .add_plugin(LoadingPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(DebugPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    commands.insert_resource(AssetsLoading(vec![]));
}
