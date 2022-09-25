use bevy::prelude::*;

use crate::states::AppState;
use crate::resources::{AssetsLoading, GameTextures};


use bevy_inspector_egui::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .add_plugin(WorldInspectorPlugin::new());
        }
    }
}