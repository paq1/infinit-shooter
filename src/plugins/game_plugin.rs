use bevy::prelude::*;
use crate::AppState;
use crate::component::velicity::Velocity;
use crate::plugins::player::PlayerPlugin;
use crate::plugins::tilemap_plugin::TilemapPlugin;

pub struct InGamePlugin;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_plugin(PlayerPlugin)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(movable_system.label("movable"))
            );
    }
}

fn movable_system(
    mut query: Query<(&Velocity, &mut Transform)>
) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
