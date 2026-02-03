//! Bonuses module
//!
//! Handles pickup bonuses that spawn from killed enemies.

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for bonus-related functionality
pub struct BonusesPlugin;

impl Plugin for BonusesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBonusEvent>()
            .add_event::<BonusCollectedEvent>()
            .add_systems(OnExit(GameState::Playing), despawn_all_bonuses)
            .add_systems(
                Update,
                (
                    spawn_bonus_on_death,
                    handle_bonus_spawns,
                    bonus_attraction,
                    bonus_collection,
                    bonus_lifetime,
                    apply_bonus_effects,
                    update_active_bonus_effects,
                    apply_speed_boost,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
