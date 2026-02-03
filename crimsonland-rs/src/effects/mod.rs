//! Effects module
//!
//! Handles visual effects like particles, explosions, and screen effects.

pub mod components;
pub mod systems;

#[allow(unused_imports)]
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
                    // Event listeners that spawn effects
                    spawn_blood_on_death,
                    spawn_levelup_effect,
                    spawn_pickup_effect,
                    spawn_muzzle_flash,
                    spawn_hit_effect,
                    // Effect processing
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
