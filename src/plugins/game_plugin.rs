use bevy::prelude::*;
use crate::plugins::tilemap_plugin::TilemapPlugin;

use crate::states::AppState;
use crate::resources::{GameTextures};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin);
    }
}
