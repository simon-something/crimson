//! Quests module
//!
//! Handles quest definitions, progression, and wave spawning.

pub mod database;
pub mod systems;
pub mod builders;

pub use database::*;
pub use systems::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for quest-related functionality
pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QuestDatabase>()
            .init_resource::<ActiveQuest>()
            .init_resource::<QuestProgress>()
            .add_event::<QuestCompletedEvent>()
            .add_event::<WaveCompletedEvent>()
            .add_systems(
                OnEnter(GameState::Playing),
                start_active_quest.run_if(quest_is_active),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_quest_state)
            .add_systems(
                Update,
                (
                    update_quest_progress,
                    spawn_wave_creatures,
                    track_quest_kills,
                    check_wave_completion,
                    check_quest_completion,
                    handle_wave_completion,
                    handle_quest_completion,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing))
                    .run_if(quest_is_active),
            );
    }
}
