//! Player module
//!
//! Contains player entity components, systems, and resources.

pub mod components;
pub mod resources;
pub mod systems;

#[allow(unused_imports)]
pub use components::*;
pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

use crate::states::GameState;

/// Plugin for player-related functionality
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_event::<PlayerDamageEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<PlayerLevelUpEvent>()
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(OnExit(GameState::Playing), despawn_players)
            .add_systems(
                Update,
                (
                    player_movement,
                    player_aim,
                    player_shooting,
                    apply_player_damage,
                    check_player_death,
                    update_player_experience,
                    player_invincibility_timer,
                    grant_experience_on_kill,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
