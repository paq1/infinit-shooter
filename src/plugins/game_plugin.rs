use bevy::prelude::*;
use crate::plugins::tilemap_plugin::TilemapPlugin;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin);
    }
}
