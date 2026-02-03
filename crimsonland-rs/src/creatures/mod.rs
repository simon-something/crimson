//! Creatures module
//!
//! Handles enemy creatures, their AI, spawning, and behavior.

pub mod components;
pub mod spawner;
pub mod systems;

pub use components::*;
pub use spawner::*;
pub use systems::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for creature-related functionality
pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CreatureRegistry>()
            .add_event::<SpawnCreatureEvent>()
            .add_event::<CreatureDeathEvent>()
            .add_systems(OnExit(GameState::Playing), despawn_all_creatures)
            .add_systems(
                Update,
                (
                    handle_creature_spawns,
                    creature_ai_update,
                    creature_movement,
                    creature_attack,
                    check_creature_death,
                    cleanup_dead_creatures,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
