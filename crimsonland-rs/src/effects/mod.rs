//! Effects module
//!
//! Handles visual effects like particles, explosions, and screen effects.

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for visual effects
pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEffectEvent>()
            .add_systems(OnExit(GameState::Playing), cleanup_all_effects)
            .add_systems(
                Update,
                (
                    handle_effect_spawns,
                    update_particles,
                    update_screen_shake,
                    cleanup_expired_effects,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
