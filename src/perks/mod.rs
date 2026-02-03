//! Perks module
//!
//! Handles player perks and their effects.

pub mod components;
pub mod registry;
pub mod systems;

pub use components::*;
pub use registry::*;
pub use systems::*;

use bevy::prelude::*;

use crate::states::{GameState, PlayingState};

/// Plugin for perk-related functionality
pub struct PerksPlugin;

impl Plugin for PerksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PerkRegistry>()
            .add_event::<PerkSelectedEvent>()
            .add_systems(OnEnter(PlayingState::PerkSelect), setup_perk_selection)
            .add_systems(
                Update,
                (
                    apply_perk_effects.run_if(in_state(GameState::Playing)),
                    handle_perk_selection.run_if(in_state(PlayingState::PerkSelect)),
                ),
            );
    }
}
